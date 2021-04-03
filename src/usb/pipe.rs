use crate::peripherals::Peripherals;
use crate::serial::Serial;
use crate::usb::packet::{Packet, SetupPacket, Token};
use core::fmt::Write;
use sam3x8e_hal::pac::uotghs::{HSTPIPCFG, HSTPIPISR};
use sam3x8e_hal::pac::UOTGHS;

#[derive(Copy, Clone)]
pub struct AllocatedPipe(u8);

#[derive(Copy, Clone)]
pub struct MessagePipe(AllocatedPipe);

#[derive(Copy, Clone)]
pub struct StreamInPipe(AllocatedPipe);

#[derive(Copy, Clone)]
pub struct StreamOutPipe(AllocatedPipe);

#[derive(Copy, Clone)]
pub enum Pipe {
  Allocated(AllocatedPipe),
  Message(MessagePipe),
  StreamIn(StreamInPipe),
  StreamOut(StreamOutPipe),
}

impl Pipe {
  pub fn new(index: u8) -> Self {
    Self::Allocated(AllocatedPipe::new(index))
  }

  pub fn into_message(self) -> Self {
    match self {
      Self::Message(_) => self,
      _ => Self::Message(MessagePipe::new(self.allocated_pipe())),
    }
  }

  pub fn into_stream_in(self) -> Self {
    match self {
      Self::StreamIn(_) => self,
      _ => Self::StreamIn(StreamInPipe::new(self.allocated_pipe())),
    }
  }

  pub fn into_stream_out(self) -> Self {
    match self {
      Self::StreamOut(_) => self,
      _ => Self::StreamOut(StreamOutPipe::new(self.allocated_pipe())),
    }
  }

  pub fn as_stream_in(&self) -> &StreamInPipe {
    match self {
      Self::StreamIn(pipe) => pipe,
      _ => panic!(),
    }
  }

  pub fn as_stream_out(&self) -> &StreamOutPipe {
    match self {
      Self::StreamOut(pipe) => pipe,
      _ => panic!(),
    }
  }

  pub fn as_message(&self) -> &MessagePipe {
    match self {
      Self::Message(pipe) => pipe,
      _ => panic!(),
    }
  }

  pub fn configure(&self, address: u8, endpoint: u8, frequency: u8, token: Token) {
    self
      .allocated_pipe()
      .configure(address, endpoint, frequency, token)
  }

  pub fn index(&self) -> u8 {
    self.allocated_pipe().0
  }

  fn allocated_pipe(&self) -> AllocatedPipe {
    match self {
      Pipe::Allocated(pipe) => *pipe,
      Pipe::Message(pipe) => pipe.0,
      Pipe::StreamIn(pipe) => pipe.0,
      Pipe::StreamOut(pipe) => pipe.0,
    }
  }
}

impl AllocatedPipe {
  pub fn new(index: u8) -> Self {
    let pipe = AllocatedPipe(index);

    if !pipe.is_enabled() {
      pipe.alloc()
    }

    pipe
  }

  pub fn send_packet(&self, _packet: &Packet) {}

  pub fn configure(&self, address: u8, endpoint: u8, frequency: u8, token: Token) {
    let uotghs = self.uotghs();
    let hstaddr1 = &uotghs.hstaddr1;
    let hstaddr2 = &uotghs.hstaddr2;
    let hstaddr3 = &uotghs.hstaddr3;

    self.hstpipcfg().write_with_zero(|w| unsafe {
      let w = w.pepnum().bits(endpoint).intfrq().bits(frequency);

      match token {
        Token::Setup => w.ptoken().setup(),
        Token::In => w.ptoken().in_(),
        Token::Out => w.ptoken().out(),
      }
    });

    match self.0 {
      0 => hstaddr1.write(|w| unsafe { w.hstaddrp0().bits(address) }),
      1 => hstaddr1.write(|w| unsafe { w.hstaddrp1().bits(address) }),
      2 => hstaddr1.write(|w| unsafe { w.hstaddrp2().bits(address) }),
      3 => hstaddr1.write(|w| unsafe { w.hstaddrp3().bits(address) }),
      4 => hstaddr2.write(|w| unsafe { w.hstaddrp4().bits(address) }),
      5 => hstaddr2.write(|w| unsafe { w.hstaddrp5().bits(address) }),
      6 => hstaddr2.write(|w| unsafe { w.hstaddrp6().bits(address) }),
      7 => hstaddr2.write(|w| unsafe { w.hstaddrp7().bits(address) }),
      8 => hstaddr3.write(|w| unsafe { w.hstaddrp8().bits(address) }),
      9 => hstaddr3.write(|w| unsafe { w.hstaddrp9().bits(address) }),
      _ => panic!("Pipe index out of bounds"),
    };

    if !self.is_configured() {
      panic!("Pipe configured incorrectly")
    }
  }

  fn alloc(&self) {
    let serial = Serial::get();

    serial.write_fmt(format_args!("[USB :: Pipe] Allocating pipe #{}", self.0));

    self
      .hstpipcfg()
      .write_with_zero(|w| unsafe { w.psize()._64_byte().pbk()._1_bank().alloc().set_bit() });
  }

  fn is_enabled(&self) -> bool {
    let hstpip = &self.uotghs().hstpip;

    match self.0 {
      0 => hstpip.read().pen0().bit_is_set(),
      1 => hstpip.read().pen1().bit_is_set(),
      2 => hstpip.read().pen2().bit_is_set(),
      3 => hstpip.read().pen3().bit_is_set(),
      4 => hstpip.read().pen4().bit_is_set(),
      5 => hstpip.read().pen5().bit_is_set(),
      6 => hstpip.read().pen6().bit_is_set(),
      7 => hstpip.read().pen7().bit_is_set(),
      8 => hstpip.read().pen8().bit_is_set(),
      _ => panic!("Pipe index out of bounds"),
    }
  }

  fn is_configured(&self) -> bool {
    self.hstpipisr().read().cfgok().bit_is_set()
  }

  fn enable(&self) {
    let hstpip = &self.uotghs().hstpip;

    match self.0 {
      0 => hstpip.modify(|_, w| w.pen0().set_bit()),
      1 => hstpip.modify(|_, w| w.pen1().set_bit()),
      2 => hstpip.modify(|_, w| w.pen2().set_bit()),
      3 => hstpip.modify(|_, w| w.pen3().set_bit()),
      4 => hstpip.modify(|_, w| w.pen4().set_bit()),
      5 => hstpip.modify(|_, w| w.pen5().set_bit()),
      6 => hstpip.modify(|_, w| w.pen6().set_bit()),
      7 => hstpip.modify(|_, w| w.pen7().set_bit()),
      8 => hstpip.modify(|_, w| w.pen8().set_bit()),
      _ => panic!("Pipe index out of bounds"),
    }
  }

  fn hstpipcfg(&self) -> &HSTPIPCFG {
    &self.uotghs().hstpipcfg()[self.0 as usize]
  }

  fn hstpipisr(&self) -> &HSTPIPISR {
    &self.uotghs().hstpipisr()[self.0 as usize]
  }

  fn uotghs(&self) -> &UOTGHS {
    unsafe { &Peripherals::get().uotghs }
  }
}

impl MessagePipe {
  pub fn new(pipe: AllocatedPipe) -> Self {
    Self(pipe)
  }

  pub fn control_transfer(
    &self,
    address: u8,
    setup_packet: &mut SetupPacket,
    data: Option<&mut [u8]>,
  ) {
    self.0.configure(address, 0, 0, Token::Setup);

    if let Some(data) = data {
      setup_packet.length = data.len() as u16
    }
  }
}

impl StreamInPipe {
  pub fn new(pipe: AllocatedPipe) -> Self {
    Self(pipe)
  }

  pub fn bulk_transfer(&self, _data: &mut [u8]) {}
}

impl StreamOutPipe {
  pub fn new(pipe: AllocatedPipe) -> Self {
    Self(pipe)
  }

  pub fn bulk_transfer(&self, _data: &mut [u8]) {}
}
