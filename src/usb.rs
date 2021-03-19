use crate::peripherals::Peripherals;
use crate::serial::Serial;
use cortex_m::peripheral::NVIC;
use embedded_hal::timer::CountDown;
use sam3x8e_hal::pac::interrupt;
use sam3x8e_hal::pac::Interrupt::UOTGHS as I_UOTGHS;
use sam3x8e_hal::pmc::PeripheralClock;
use sam3x8e_hal::time::U32Ext;

static mut S_USB: Option<USB> = None;

const USB_SETTLE_DELAY: u32 = 200;

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

enum ConfigureError {
  DeviceInitIncomplete,
  Error,
}

#[derive(Copy, Clone)]
struct Device {}

impl Device {
  pub fn poll(&self) {}
  pub fn release(&self) {}
}

pub struct USB {
  state: State,
  vbus_state: VBusState,
  devices: [Option<Device>; 16],
  configure_result: Result<(), ConfigureError>,
}

impl USB {
  pub fn init() {
    unsafe {
      S_USB = Some(USB {
        state: State::DetachedInitialize,
        vbus_state: VBusState::Off,
        devices: [None; 16],
        configure_result: Ok(()),
      })
    }
  }

  pub fn poll(&mut self) {
    let low_speed = false;
    let peripherals = Peripherals::get();
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
          timer.try_start(USB_SETTLE_DELAY.hz()).unwrap();
          self.state = State::AttachedSettle
        }
      }
      _ => (),
    }

    for device in self.devices.iter() {
      if let Some(device) = device {
        device.poll()
      }
    }

    match self.state {
      State::DetachedInitialize => {
        self.start();
        self.reset();

        for device in self.devices.iter() {
          if let Some(device) = device {
            device.release()
          }
        }

        self.state = State::DetachedWaitForDevice
      }
      State::AttachedSettle => {
        if timer.try_wait().is_ok() {
          self.state = State::AttachedResetDevice
        }
      }
      State::AttachedResetDevice => {
        self.reset_bus();
        self.state = State::AttachedWaitResetComplete
      }
      State::AttachedWaitResetComplete => {
        if self.has_sent_reset() {
          self.ack_sent_reset();
          self.enable_sof();
          self.state = State::AttachedWaitSOF;
          timer.try_start(20.hz()).unwrap()
        }
      }
      State::AttachedWaitSOF => {
        if self.is_sof() && timer.try_wait().is_ok() {
          self.state = State::Configuring
        }
      }
      State::Configuring => match self.configure(0, 0, low_speed) {
        Ok(()) => self.state = State::Running,
        Err(ConfigureError::DeviceInitIncomplete) => (),
        Err(error) => {
          self.configure_result = Err(error);
          self.state = State::Error
        }
      },
      _ => (),
    }
  }

  fn has_sent_reset(&self) -> bool {
    true
  }

  fn reset_bus(&self) {}
  fn ack_sent_reset(&self) {}
  fn enable_sof(&self) {}

  fn is_sof(&self) -> bool {
    true
  }

  fn reset(&mut self) {
    for device in self.devices.iter_mut() {
      *device = None
    }
  }

  fn configure(&self, _a: u32, _b: u32, _low_speed: bool) -> Result<(), ConfigureError> {
    Ok(())
  }

  fn is_detached(&self) -> bool {
    self.state == State::DetachedIllegal
      || self.state == State::DetachedInitialize
      || self.state == State::DetachedWaitForDevice
  }

  fn start(&self) {
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
