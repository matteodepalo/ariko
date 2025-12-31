use crate::peripherals::Peripherals;
use crate::usb::device::Device;
use crate::usb::pipe::{InnerPipe, MessagePipe, Pipe};
use core::cell::RefCell;
use critical_section::Mutex;
use sam3x8e_hal::timer::CountDown;
use log::debug;
use nb::block;
use sam3x8e_hal::pac::{uotghs, UOTGHS};
use sam3x8e_hal::pmc::PeripheralClock;
use sam3x8e_hal::time::U32Ext;

mod device;
mod packet;
mod pipe;

// Re-export CP210xDevice for use in main
pub use device::CP210xDevice;

static USB_INSTANCE: Mutex<RefCell<Option<USB>>> = Mutex::new(RefCell::new(None));

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
    critical_section::with(|cs| {
      USB_INSTANCE.borrow(cs).replace(Some(USB {
        state: State::Disabled,
        devices: [
          None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
          None,
        ],
        control_pipe: None,
        pipes: [None, None, None, None, None, None, None, None],
      }));
    });
  }

  pub fn with<F, R>(f: F) -> R
  where
    F: FnOnce(&mut USB) -> R,
  {
    critical_section::with(|cs| {
      let mut borrow = USB_INSTANCE.borrow(cs).borrow_mut();
      let usb = borrow.as_mut().expect("USB not initialized");
      f(usb)
    })
  }

  pub fn poll(&mut self) {
    match self.try_poll() {
      Ok(()) => (),
      Err(Error::TransferTimeout) => {
        debug!("[USB] Transfer timeout");
      }
      Err(error) => {
        debug!("[USB] Error: {:?}", error);
        self.set_state(State::Error);
      }
    }
  }

  pub fn alloc_pipe(
    &mut self,
    configure_callback: fn(pipe: Pipe) -> Pipe,
  ) -> Result<&mut Pipe, Error> {
    let index = self.next_pipe_index()?;
    let option = &mut self.pipes[(index - 1) as usize];

    *option = Some(configure_callback(Pipe::new(index)));

    Ok(option.as_mut().unwrap())
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

  pub fn get_pipe(&self, index: u8) -> Option<&Pipe> {
    if index == 0 || index > 8 {
      return None;
    }
    self.pipes[(index - 1) as usize].as_ref()
  }

  fn try_poll(&mut self) -> Result<(), Error> {
    match self.state {
      State::Disabled => {
        self.enable();
        self.set_state(State::Disconnected)
      }
      State::Disconnected => {
        if self.uotghs().hstisr().read().dconni().bit_is_set() {
          self.start()?;
          self.set_state(State::Ready)
        }
      }
      State::Ready => {
        if self.uotghs().hstisr().read().ddisci().bit_is_set() {
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
    debug!("[USB] {:?} -> {:?}", self.state, state);
    self.state = state;
  }

  fn enable(&mut self) {

    Peripherals::with(|p| p.pmc.enable_clock(PeripheralClock::UOtgHs));

    // Configure USB in host mode with all settings at once
    // UIDE=0: Mode from UIMOD bit (not ID pin)
    // UIMOD=0: Host mode (using .host() method)
    // VBUSPO=0: VBUS power active low (Arduino Due circuit)
    // OTGPADE=1: Enable OTG pad
    // USBE=1: Enable USB
    // FRZCLK=1: Keep clock frozen initially
    self.uotghs().ctrl().write(|w| {
      w.uide().clear_bit()
       .uimod().host()
       .vbuspo().clear_bit()
       .otgpade().set_bit()
       .usbe().set_bit()
       .frzclk().set_bit()
    });

    // Now unfreeze the clock
    self.uotghs().ctrl().modify(|_, w| w.frzclk().clear_bit());

    while self.uotghs().sr().read().clkusable().bit_is_clear() {}

    // Enable host mode features
    self.uotghs().hstctrl().write(|w| w.sofe().set_bit());  // Enable SOF generation

    // Clear all pending host interrupts
    unsafe {
      self
        .uotghs()
        .hsticr()
        .write_with_zero(|w| w.bits(u32::max_value()));
    }

    // Enable device connection/disconnection interrupts
    unsafe {
      self
        .uotghs()
        .hstier()
        .write_with_zero(|w| w.dconnies().set_bit().ddiscies().set_bit());
    }

    unsafe {
      self
        .uotghs()
        .scr()
        .write_with_zero(|w| w.bits(u32::max_value()));
    }

    self.uotghs().ctrl().modify(|_, w| w.vbushwc().set_bit());
    unsafe { self.uotghs().sfr().write_with_zero(|w| w.vbusrqs().set_bit()); }

  }

  fn start(&mut self) -> Result<(), Error> {

    unsafe {
      self
        .uotghs()
        .hsticr()
        .write_with_zero(|w| w.dconnic().set_bit());
    }

    self.delay(CONNECTION_DELAY);
    self.reset()
  }

  fn stop(&mut self) {
    unsafe {
      self
        .uotghs()
        .hsticr()
        .write_with_zero(|w| w.ddiscic().set_bit());
    }

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

  fn reset(&mut self) -> Result<(), Error> {
    self.uotghs().hstctrl().modify(|_, w| w.reset().set_bit());

    while self.uotghs().hstisr().read().rsti().bit_is_clear() {}

    unsafe {
      self
        .uotghs()
        .hsticr()
        .write_with_zero(|w| w.rstic().set_bit());
    }

    self.delay(RESET_DELAY);

    self.configure()
  }

  fn configure(&mut self) -> Result<(), Error> {
    let address = self.next_device_address()?;
    let device = Device::configure(address, self)?;

    self.devices[(address - 1) as usize] = Some(device);

    Ok(())
  }

  fn poll_devices(&self) -> Result<(), Error> {
    for option in self.devices.iter() {
      if let Some(device) = option {
        device.poll(self)?
      }
    }

    Ok(())
  }

  fn next_pipe_index(&self) -> Result<u8, Error> {
    let mut result = Err(Error::TooManyPipes);

    for (index, pipe) in self.pipes.iter().enumerate() {
      if pipe.is_none() {
        result = Ok((index + 1) as u8);
        break;
      }
    }

    result
  }

  fn next_device_address(&self) -> Result<u8, Error> {
    let mut result = Err(Error::TooManyDevices);

    for (address, device) in self.devices.iter().enumerate() {
      if device.is_none() {
        result = Ok((address + 1) as u8);
        break;
      }
    }

    result
  }

  fn delay(&self, ms: u32) {
    Peripherals::with(|p| {
      let timer = &mut p.timer;
      timer.try_start(ms.hz()).unwrap();
      block!(timer.try_wait()).unwrap()
    });
  }

  fn uotghs(&self) -> &uotghs::RegisterBlock {
    // Use pointer to get a static reference to the UOTGHS peripheral
    // This is safe because the hardware registers are memory-mapped and don't move
    unsafe { &*UOTGHS::ptr() }
  }
}
