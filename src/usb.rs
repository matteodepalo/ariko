mod generic;
mod serial;

use crate::peripherals::Peripherals;
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
}

pub enum Error {
  DeviceInitIncomplete,
  DeviceNotSupported,
  TooManyDevices,
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

impl Device {
  pub fn poll(&self) -> Result<(), Error> {
    Ok(())
  }
  pub fn release(&self) -> Result<(), Error> {
    Ok(())
  }
}

impl DeviceClass {
  pub fn configure(&self, address: u8) -> Result<Device, Error> {
    match self {
      DeviceClass::Serial(serial) => serial.configure(address),
      DeviceClass::Generic(generic) => generic.configure(address),
    }
  }
}

pub struct USB {
  state: State,
  vbus_state: VBusState,
  devices: [Option<Device>; 16],
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

  pub fn handle_interrupt(&mut self) {
    let u = &Peripherals::get().uotghs;

    let is_error = u.sr.read().vberri().bit_is_set()
      || u.sr.read().bcerri().bit_is_set()
      || u.sr.read().hnperri().bit_is_set()
      || u.sr.read().stoi().bit_is_set();

    if u.hstisr.read().ddisci().bit_is_set() && u.hstimr.read().ddiscie().bit_is_set() {
      u.hsticr.write_with_zero(|w| w.ddiscic().set_bit());
      u.hstidr.write_with_zero(|w| w.ddisciec().set_bit());
      u.hstctrl.write(|w| w.reset().clear_bit());
      u.hsticr.write_with_zero(|w| w.dconnic().set_bit());
      u.hstier.write_with_zero(|w| w.dconnies().set_bit());
      self.vbus_state = VBusState::Disconnected
    } else if u.hstisr.read().dconni().bit_is_set() && u.hstimr.read().dconnie().bit_is_set() {
      u.hsticr.write_with_zero(|w| w.dconnic().set_bit());
      u.hstidr.write_with_zero(|w| w.dconniec().set_bit());
      u.hsticr.write_with_zero(|w| w.ddiscic().set_bit());
      u.hstier.write_with_zero(|w| w.ddiscies().set_bit());
      self.vbus_state = VBusState::Connected
    } else if u.sr.read().vberri().bit_is_set() {
      u.scr.write_with_zero(|w| w.vberric().set_bit());
      self.vbus_state = VBusState::Disconnected
    } else {
      while !u.sr.read().clkusable().bit_is_set() {}
      u.ctrl.write(|w| w.frzclk().clear_bit());

      if u.sr.read().vbusti().bit_is_set() {
        u.scr.write_with_zero(|w| w.vbustic().set_bit());

        if u.sr.read().vbus().bit_is_set() {
          self.vbus_state = VBusState::Disconnected
        } else {
          u.ctrl.write(|w| w.frzclk().set_bit());
          self.vbus_state = VBusState::Off
        }
      } else if is_error {
        u.scr.write_with_zero(|w| {
          w.vberric()
            .set_bit()
            .bcerric()
            .set_bit()
            .hnperric()
            .set_bit()
            .stoic()
            .set_bit()
        })
      }
    }
  }

  fn default() -> Self {
    USB {
      state: State::DetachedInitialize,
      vbus_state: VBusState::Off,
      devices: [None; 16],
    }
  }

  fn try_poll(&mut self) -> Result<(), Error> {
    let peripherals = Peripherals::get();
    let uotghs = &peripherals.uotghs;
    let timer = &mut peripherals.timer;

    match self.vbus_state {
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
        let _ = self.configure_device()?;
        self.state = State::Running;
      }
      _ => (),
    }

    Ok(())
  }

  fn start(&mut self) -> Result<(), Error> {
    cortex_m::interrupt::free(|_| {
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
    })
  }

  fn next_free_address(&self) -> Result<u8, Error> {
    let mut result = Err(Error::TooManyDevices);

    for (address, device) in self.devices.iter().enumerate() {
      if device.is_none() {
        result = Ok(address as u8);
        break;
      }
    }

    result
  }

  fn configure_device(&mut self) -> Result<(), Error> {
    let mut result = Err(Error::Unknown);
    let address = self.next_free_address()?;

    for device_class in DEVICE_CLASSES.iter() {
      match device_class.configure(address) {
        Ok(device) => {
          self.devices[address as usize] = Some(device);
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
      State::DetachedInitialize | State::DetachedWaitForDevice => true,
      _ => false,
    }
  }

  fn poll_devices(&self) -> Result<(), Error> {
    for option in self.devices.iter() {
      if let Some(device) = option {
        device.poll()?
      }
    }

    Ok(())
  }

  fn release_devices(&mut self) -> Result<(), Error> {
    for option in self.devices.iter_mut() {
      if let Some(device) = option {
        device.release()?;
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
  USB::get().handle_interrupt();
}
