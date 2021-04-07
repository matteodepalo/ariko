use crate::peripherals::Peripherals;
use crate::serial::Serial;
use crate::usb::device::{Device, DeviceClass};
use crate::usb::pipe::{InnerPipe, MessagePipe, Pipe};
use core::fmt::Write;
use cortex_m::peripheral::NVIC;
use embedded_hal::timer::CountDown;
use nb::block;
use sam3x8e_hal::pac::Interrupt::UOTGHS as UOTGHS_INTERRUPT;
use sam3x8e_hal::pac::{RTT, UOTGHS};
use sam3x8e_hal::pmc::PeripheralClock;
use sam3x8e_hal::time::U32Ext;
use sam3x8e_hal::timer::Timer;

mod device;
mod packet;
mod pipe;

static mut S_USB: Option<USB> = None;

const CONNECTION_DELAY: u32 = 200;
const RESET_DELAY: u32 = 20;

#[derive(PartialEq, Debug)]
enum State {
  Disabled,
  Disconnected,
  Ready,
  Error,
}

#[derive(Debug)]
pub enum Error {
  DeviceNotSupported,
  TooManyDevices,
  TooManyPipes,
  TransferTimeout,
}

pub struct USB {
  state: State,
  devices: [Option<Device>; 16],
  control_pipe: Option<MessagePipe>,
  pipes: [Option<Pipe>; 8],
}

impl USB {
  pub fn init() {
    unsafe {
      S_USB = Some(USB {
        state: State::Disabled,
        devices: [
          None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
          None,
        ],
        control_pipe: None,
        pipes: [None, None, None, None, None, None, None, None],
      })
    }
  }

  pub fn poll(&mut self) {
    match self.try_poll() {
      Ok(()) => (),
      Err(error) => {
        Serial::get()
          .write_fmt(format_args!("[USB] Error: {:?}\n\r", error))
          .unwrap();

        self.set_state(State::Error);
      }
    }
  }

  pub fn alloc_pipe(
    &mut self,
    configure_callback: fn(pipe: &Pipe) -> Pipe,
  ) -> Result<&Pipe, Error> {
    let index = self.next_pipe_index()?;
    let option = &mut self.pipes[(index - 1) as usize];

    *option = Some(configure_callback(&Pipe::new(index)));

    Ok(option.as_ref().unwrap())
  }

  pub fn release_pipe(&mut self, pipe: &Pipe) {
    let index = (pipe.index() - 1) as usize;

    if let Some(pipe) = &self.pipes[index] {
      pipe.inner_pipe().release();
      self.pipes[index] = None
    }
  }

  pub fn control_pipe(&mut self) -> &MessagePipe {
    if self.control_pipe.is_none() {
      self.control_pipe = Some(MessagePipe::new(InnerPipe::new(0)))
    }

    self.control_pipe.as_ref().unwrap()
  }

  fn try_poll(&mut self) -> Result<(), Error> {
    match self.state {
      State::Disabled => {
        self.enable();
        self.set_state(State::Disconnected)
      }
      State::Disconnected => {
        if self.uotghs().hstisr.read().dconni().bit_is_set() {
          self.start()?;
          self.set_state(State::Ready)
        }
      }
      State::Ready => {
        if self.uotghs().hstisr.read().ddisci().bit_is_set() {
          self.stop();
          self.set_state(State::Disconnected)
        } else {
          self.poll_devices()?
        }
      }
      State::Error => (),
    }

    Ok(())
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

  fn start(&mut self) -> Result<(), Error> {
    self
      .uotghs()
      .hsticr
      .write_with_zero(|w| w.dconnic().set_bit());

    self.delay(CONNECTION_DELAY);
    self.peripherals().pmc.enable_clock(PeripheralClock::UOtgHs);

    while self.uotghs().sr.read().clkusable().bit_is_clear() {}

    self.reset()
  }

  fn reset(&mut self) -> Result<(), Error> {
    self.uotghs().hstctrl.modify(|_, w| w.reset().set_bit());

    while self.uotghs().hstisr.read().rsti().bit_is_clear() {}

    self
      .uotghs()
      .hsticr
      .write_with_zero(|w| w.rstic().set_bit());
    self.delay(RESET_DELAY);
    self.configure()
  }

  fn configure(&mut self) -> Result<(), Error> {
    let mut result = Ok(());
    let address = self.next_device_address()?;

    for device_class in DeviceClass::all().iter() {
      match device_class.configure(address) {
        Ok(device) => {
          self.devices[address as usize] = Some(device);
          result = Ok(());
          break;
        }
        Err(Error::DeviceNotSupported) => (),
        Err(error) => {
          result = Err(error);
          break;
        }
      }
    }

    result
  }

  fn enable(&mut self) {
    cortex_m::interrupt::free(|_| {
      self
        .uotghs()
        .hstier
        .write_with_zero(|w| w.dconnies().set_bit().ddiscies().set_bit());

      self.uotghs().ctrl.modify(|_, w| {
        w.uide()
          .clear_bit()
          .uimod()
          .clear_bit()
          .vbuspo()
          .set_bit()
          .otgpade()
          .set_bit()
          .usbe()
          .set_bit()
          .vbushwc()
          .set_bit()
          .vbuste()
          .set_bit()
          .frzclk()
          .set_bit()
      });

      self.uotghs().sfr.write_with_zero(|w| w.vbusrqs().set_bit());

      unsafe { NVIC::unmask(UOTGHS_INTERRUPT) };
    })
  }

  fn stop(&mut self) {
    self
      .uotghs()
      .hsticr
      .write_with_zero(|w| w.ddiscic().set_bit());

    if let Some(pipe) = &self.control_pipe {
      pipe.inner_pipe().release();
      self.control_pipe = None;
    }

    for option in self.devices.iter_mut() {
      if let Some(device) = option {
        device.release();
        *option = None
      }
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

  fn next_pipe_index(&self) -> Result<u8, Error> {
    let mut result = Err(Error::TooManyPipes);

    for (index, pipe) in self.pipes.iter().enumerate() {
      if pipe.is_none() {
        result = Ok(index as u8);
        break;
      }
    }

    result
  }

  fn next_device_address(&self) -> Result<u8, Error> {
    let mut result = Err(Error::TooManyDevices);

    for (address, device) in self.devices.iter().enumerate() {
      if device.is_none() {
        result = Ok(address as u8);
        break;
      }
    }

    result
  }

  fn delay(&self, ms: u32) {
    self.timer().try_start(ms.hz()).unwrap();
    block!(self.timer().try_wait()).unwrap()
  }

  fn peripherals(&self) -> &mut Peripherals {
    unsafe { Peripherals::get() }
  }

  fn uotghs(&self) -> &UOTGHS {
    &self.peripherals().uotghs
  }

  fn timer(&self) -> &mut Timer<RTT> {
    &mut self.peripherals().timer
  }

  pub fn get() -> &'static mut Self {
    unsafe { S_USB.as_mut().unwrap() }
  }
}
