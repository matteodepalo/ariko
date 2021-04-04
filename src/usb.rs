mod device;
mod interrupt;
mod packet;
mod pipe;

use crate::peripherals::Peripherals;
use crate::serial::Serial;
use crate::usb::device::{Device, DeviceClass};
use crate::usb::pipe::{AllocatedPipe, MessagePipe, Pipe};
use core::fmt::Write;
use cortex_m::peripheral::NVIC;
use embedded_hal::timer::CountDown;
use sam3x8e_hal::pac::Interrupt::UOTGHS as I_UOTGHS;
use sam3x8e_hal::pmc::PeripheralClock;
use sam3x8e_hal::time::U32Ext;

static mut S_USB: Option<USB> = None;

const SETTLE_DELAY: u32 = 200;

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
enum VBusState {
  Off,
  Disconnected,
  Connected,
}

#[derive(Debug)]
pub enum Error {
  DeviceInitIncomplete,
  DeviceNotSupported,
  TooManyDevices,
  TooManyPipes,
  TransferTimeout,
  Unknown,
}

pub struct USB {
  state: State,
  vbus_state: VBusState,
  devices: [Option<Device>; 16],
  control_pipe: Option<MessagePipe>,
  pipes: [Option<Pipe>; 9],
}

impl USB {
  pub fn init() {
    unsafe { S_USB = Some(Self::default()) }
  }

  pub fn poll(&mut self) {
    match self.try_poll() {
      Ok(()) | Err(Error::DeviceInitIncomplete) => (),
      Err(_) => {
        self.set_state(State::Error);
      }
    }
  }

  pub fn alloc_pipe(
    &mut self,
    configure_callback: fn(pipe: &Pipe) -> Pipe,
  ) -> Result<&Pipe, Error> {
    let index = self.next_free_pipe_index()?;
    self.pipes[index as usize] = Some(configure_callback(&Pipe::new(index + 1)));
    Ok(self.pipes[index as usize].as_ref().unwrap())
  }

  pub fn release_pipe(&mut self, pipe: &Pipe) {
    let index = (pipe.index() - 1) as usize;

    match self.pipes[index] {
      Some(pipe) => {
        pipe.release();
        self.pipes[index] = None
      }
      None => (),
    }
  }

  pub fn control_pipe(&mut self) -> &MessagePipe {
    self.control_pipe = match self.control_pipe {
      None => Some(MessagePipe::new(AllocatedPipe::new(0))),
      pipe => pipe,
    };

    self.control_pipe.as_ref().unwrap()
  }

  fn default() -> Self {
    USB {
      state: State::DetachedInitialize,
      vbus_state: VBusState::Off,
      devices: [None; 16],
      control_pipe: None,
      pipes: [None; 9],
    }
  }

  fn set_state(&mut self, state: State) {
    Serial::get()
      .write_fmt(format_args!(
        "[USB] Transitioning state from {:?} to {:?}\n\r",
        self.state, state
      ))
      .unwrap();

    self.state = state;
  }

  fn set_vbus_state(&mut self, state: VBusState) {
    Serial::get()
      .write_fmt(format_args!(
        "[USB] Transitioning VBus state from {:?} to {:?}\n\r",
        self.vbus_state, state
      ))
      .unwrap();

    self.vbus_state = state;
  }

  fn try_poll(&mut self) -> Result<(), Error> {
    let peripherals = unsafe { Peripherals::get() };
    let uotghs = &peripherals.uotghs;
    let timer = &mut peripherals.timer;

    match self.vbus_state {
      VBusState::Disconnected => {
        if !self.is_detached() {
          self.set_state(State::DetachedInitialize)
        }
      }
      VBusState::Connected => {
        if self.is_detached() {
          timer.try_start(SETTLE_DELAY.hz()).unwrap();
          self.set_state(State::AttachedSettle)
        }
      }
      _ => (),
    }

    self.poll_devices()?;

    match self.state {
      State::DetachedInitialize => {
        self.start()?;
        self.set_state(State::DetachedWaitForDevice)
      }
      State::AttachedSettle => {
        if timer.try_wait().is_ok() {
          self.set_state(State::AttachedResetDevice)
        }
      }
      State::AttachedResetDevice => {
        uotghs.hstctrl.modify(|_, w| w.reset().set_bit());
        self.set_state(State::AttachedWaitResetComplete)
      }
      State::AttachedWaitResetComplete => {
        if uotghs.hstisr.read().rsti().bit_is_set() {
          uotghs.hsticr.write_with_zero(|w| w.rstic().set_bit());
          uotghs.hstctrl.modify(|_, w| w.sofe().set_bit());
          self.set_state(State::AttachedWaitSOF);
          timer.try_start(20.hz()).unwrap()
        }
      }
      State::AttachedWaitSOF => {
        if uotghs.hstisr.read().hsofi().bit_is_set() && timer.try_wait().is_ok() {
          self.set_state(State::Configuring)
        }
      }
      State::Configuring => {
        let _ = self.configure_device()?;
        self.set_state(State::Running)
      }
      _ => (),
    }

    Ok(())
  }

  fn start(&mut self) -> Result<(), Error> {
    let serial = Serial::get();

    cortex_m::interrupt::free(|_| {
      serial.write_str("[USB] Begin start\n\r").unwrap();

      self.release_devices()?;

      let peripherals = unsafe { Peripherals::get() };
      let nvic = &mut peripherals.nvic;
      let uotghs = &mut peripherals.uotghs;
      let pmc = &mut peripherals.pmc;
      let ctrl = &uotghs.ctrl;

      pmc.enable_clock(PeripheralClock::UOtgHs);

      unsafe { nvic.set_priority(I_UOTGHS, 0) };
      unsafe { NVIC::unmask(I_UOTGHS) };

      ctrl.modify(|_, w| w.uide().clear_bit());
      ctrl.modify(|_, w| w.uimod().clear_bit());
      ctrl.modify(|_, w| w.vbuspo().set_bit());
      ctrl.modify(|_, w| w.otgpade().set_bit());
      ctrl.modify(|_, w| w.usbe().set_bit());
      ctrl.modify(|_, w| w.frzclk().clear_bit());

      while !uotghs.sr.read().clkusable().bit_is_set() {}

      uotghs.scr.write_with_zero(|w| w.vbustic().set_bit());
      ctrl.modify(|_, w| w.vbushwc().set_bit().vbuste().set_bit().vberre().set_bit());
      uotghs.sfr.write_with_zero(|w| w.vbusrqs().set_bit());
      uotghs.sfr.write_with_zero(|w| w.vbustis().set_bit());
      uotghs.hstier.write_with_zero(|w| w.dconnies().set_bit());
      ctrl.modify(|_, w| w.frzclk().set_bit());

      serial.write_str("[USB] End start\n\r").unwrap();

      Ok(())
    })
  }

  fn next_free_pipe_index(&self) -> Result<u8, Error> {
    let mut result = Err(Error::TooManyPipes);

    for (index, pipe) in self.pipes.iter().enumerate() {
      if pipe.is_none() {
        result = Ok(index as u8);
        break;
      }
    }

    result
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

    for device_class in DeviceClass::all().iter() {
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
      if option.is_some() {
        *option = None
      }
    }

    for pipe in self.pipes.iter() {
      if let Some(pipe) = pipe {
        pipe.release()
      }
    }

    Ok(())
  }

  pub fn get() -> &'static mut Self {
    unsafe { S_USB.as_mut().unwrap() }
  }
}
