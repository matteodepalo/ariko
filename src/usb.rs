mod generic;
mod serial;

use crate::peripherals::Peripherals;
use crate::serial::Serial;
use crate::usb::generic::{GenericDevice, GenericDeviceClass};
use crate::usb::serial::{SerialDevice, SerialDeviceClass};
use cortex_m::peripheral::NVIC;
use embedded_hal::timer::CountDown;
use sam3x8e_hal::pac::interrupt;
use sam3x8e_hal::pac::Interrupt::UOTGHS as I_UOTGHS;
use sam3x8e_hal::pmc::PeripheralClock;
use sam3x8e_hal::time::U32Ext;

static mut S_USB: Option<USB> = None;

const SETTLE_DELAY: u32 = 200;

const DEVICE_CLASSES: [DeviceClass; 2] = [
  DeviceClass::Serial(SerialDeviceClass {}),
  DeviceClass::Generic(GenericDeviceClass {}),
];

#[derive(PartialEq)]
enum State {
  DetachedInitialize,
  DetachedIllegal,
  DetachedWaitForDevice,
  AttachedSettle,
  AttachedResetDevice,
  AttachedWaitResetComplete,
  AttachedWaitSOF,
  Configuring,
  Running,
  Error,
}

#[derive(PartialEq)]
enum VBusState {
  Off,
  Disconnected,
  Connected,
  Error,
}

pub enum Error {
  DeviceInitIncomplete,
  DeviceNotSupported,
  Unknown,
}

#[derive(Copy, Clone)]
pub enum Device {
  Serial(SerialDevice),
  Generic(GenericDevice),
}

#[derive(Copy, Clone)]
pub enum DeviceClass {
  Serial(SerialDeviceClass),
  Generic(GenericDeviceClass),
}

#[derive(Copy, Clone)]
pub struct DeviceConfiguration {
  pub address: u8,
  pub device: Device,
}

impl Device {
  pub fn poll(&self) -> Result<(), Error> {
    Ok(())
  }
  pub fn release(&self) -> Result<(), Error> {
    Ok(())
  }
}

impl DeviceClass {
  pub fn configure(&self) -> Result<DeviceConfiguration, Error> {
    match self {
      DeviceClass::Serial(serial) => serial.configure(),
      DeviceClass::Generic(generic) => generic.configure(),
    }
  }
}

impl DeviceConfiguration {
  pub fn new(device: Device) -> Self {
    Self { device, address: 0 }
  }
}

pub struct USB {
  state: State,
  vbus_state: VBusState,
  device_configurations: [Option<DeviceConfiguration>; 16],
}

impl USB {
  pub fn init() {
    unsafe { S_USB = Some(Self::default()) }
  }

  pub fn poll(&mut self) {
    match self.try_poll() {
      Ok(()) | Err(Error::DeviceInitIncomplete) => (),
      Err(_) => {
        self.state = State::Error;
      }
    }
  }

  fn default() -> Self {
    USB {
      state: State::DetachedInitialize,
      vbus_state: VBusState::Off,
      device_configurations: [None; 16],
    }
  }

  fn try_poll(&mut self) -> Result<(), Error> {
    let low_speed = false;
    let peripherals = Peripherals::get();
    let uotghs = &peripherals.uotghs;
    let timer = &mut peripherals.timer;

    match self.vbus_state {
      VBusState::Error => self.state = State::DetachedIllegal,
      VBusState::Disconnected => {
        if !self.is_detached() {
          self.state = State::DetachedInitialize
        }
      }
      VBusState::Connected => {
        if self.is_detached() {
          timer.try_start(SETTLE_DELAY.hz()).unwrap();
          self.state = State::AttachedSettle
        }
      }
      _ => (),
    }

    self.poll_devices()?;

    match self.state {
      State::DetachedInitialize => {
        self.start()?;
        self.state = State::DetachedWaitForDevice
      }
      State::AttachedSettle => {
        if timer.try_wait().is_ok() {
          self.state = State::AttachedResetDevice
        }
      }
      State::AttachedResetDevice => {
        uotghs.hstctrl.write_with_zero(|w| w.reset().set_bit());
        self.state = State::AttachedWaitResetComplete
      }
      State::AttachedWaitResetComplete => {
        if uotghs.hstisr.read().rsti().bit_is_set() {
          uotghs.hsticr.write_with_zero(|w| w.rstic().set_bit());
          uotghs.hstctrl.write(|w| w.sofe().set_bit());
          self.state = State::AttachedWaitSOF;
          timer.try_start(20.hz()).unwrap()
        }
      }
      State::AttachedWaitSOF => {
        if uotghs.hstisr.read().hsofi().bit_is_set() && timer.try_wait().is_ok() {
          self.state = State::Configuring
        }
      }
      State::Configuring => {
        let _ = self.configure_device(0, 0, low_speed)?;
        self.state = State::Running;
      }
      _ => (),
    }

    Ok(())
  }

  fn start(&mut self) -> Result<(), Error> {
    self.release_devices()?;

    let peripherals = Peripherals::get();
    let nvic = &mut peripherals.nvic;
    let uotghs = &mut peripherals.uotghs;
    let pmc = &mut peripherals.pmc;
    let ctrl = &uotghs.ctrl;

    // Enable USB peripheral clock
    pmc.enable_clock(PeripheralClock::UOtgHs);

    // Always authorize asynchronous USB interrupts to exit sleep mode
    unsafe { nvic.set_priority(I_UOTGHS, 0) };
    unsafe { NVIC::unmask(I_UOTGHS) };

    // Disable ID pin
    ctrl.modify(|_, w| w.uide().clear_bit());

    // Force host mode
    ctrl.modify(|_, w| w.uimod().clear_bit());

    // Set VBOF active high
    ctrl.modify(|_, w| w.vbuspo().set_bit());

    // Enable OTG pad
    ctrl.modify(|_, w| w.otgpade().set_bit());

    // Enable USB macro
    ctrl.modify(|_, w| w.usbe().set_bit());

    // Unfreeze USB clock
    ctrl.modify(|_, w| w.frzclk().clear_bit());

    // Check USB clock
    while !uotghs.sr.read().clkusable().bit_is_set() {}

    // Clear VBus interrupt
    uotghs.scr.write_with_zero(|w| w.vbustic().set_bit());

    // Enable VBus transition and error interrupts
    // automatic VBus control after VBus error
    ctrl.modify(|_, w| w.vbushwc().set_bit().vbuste().set_bit().vberre().set_bit());

    // Requests VBus activation
    uotghs.sfr.write_with_zero(|w| w.vbusrqs().set_bit());

    // Force VBus transition
    uotghs.sfr.write_with_zero(|w| w.vbustis().set_bit());

    // Enable main control interrupt
    // Connection, SOF and reset
    uotghs
      .hstier
      .write_with_zero(|w| w.dconnies().set_bit().hwupies().set_bit());

    // Freeze USB clock
    ctrl.modify(|_, w| w.frzclk().set_bit());

    Ok(())
  }

  fn configure_device(&self, _parent: u32, _port: u32, _low_speed: bool) -> Result<(), Error> {
    let mut result = Err(Error::Unknown);

    for device_class in DEVICE_CLASSES.iter() {
      match device_class.configure() {
        Ok(_configuration) => {
          result = Ok(());
          break;
        }
        Err(Error::DeviceNotSupported) => {}
        Err(error) => {
          result = Err(error);
          break;
        }
      }
    }

    result
  }

  fn is_detached(&self) -> bool {
    match self.state {
      State::DetachedIllegal | State::DetachedInitialize | State::DetachedWaitForDevice => true,
      _ => false,
    }
  }

  fn poll_devices(&self) -> Result<(), Error> {
    for option in self.device_configurations.iter() {
      if let Some(configuration) = option {
        configuration.device.poll()?
      }
    }

    Ok(())
  }

  fn release_devices(&mut self) -> Result<(), Error> {
    for option in self.device_configurations.iter_mut() {
      if let Some(configuration) = option {
        configuration.device.release()?;
        *option = None
      };
    }

    Ok(())
  }

  pub fn get() -> &'static mut Self {
    unsafe { S_USB.as_mut().unwrap() }
  }
}

#[interrupt]
unsafe fn UOTGHS() {
  let _uotghs = &Peripherals::get().uotghs;
  let _serial = Serial::get();
}
