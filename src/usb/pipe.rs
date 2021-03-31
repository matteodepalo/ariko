use crate::peripherals::Peripherals;
use crate::usb::packet::{Packet, SetupPacket};

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

  pub fn configure(&self, address: u8, endpoint: u8) {
    self.allocated_pipe().configure(address, endpoint)
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

  fn alloc(&self) {}
  fn configure(&self, _address: u8, _endpoint: u8) {}
  fn send_packet(&self, _packet: &Packet) {}

  fn is_enabled(&self) -> bool {
    let uotghs = &Peripherals::get().uotghs;

    match self.0 {
      0 => uotghs.hstpip.read().pen0().bit_is_set(),
      1 => uotghs.hstpip.read().pen1().bit_is_set(),
      2 => uotghs.hstpip.read().pen2().bit_is_set(),
      3 => uotghs.hstpip.read().pen3().bit_is_set(),
      4 => uotghs.hstpip.read().pen4().bit_is_set(),
      5 => uotghs.hstpip.read().pen5().bit_is_set(),
      6 => uotghs.hstpip.read().pen6().bit_is_set(),
      7 => uotghs.hstpip.read().pen7().bit_is_set(),
      8 => uotghs.hstpip.read().pen8().bit_is_set(),
      _ => panic!("Pipe index out of bounds"),
    }
  }

  fn enable(&self) {
    let uotghs = &Peripherals::get().uotghs;

    match self.0 {
      0 => uotghs.hstpip.modify(|_, w| w.pen0().set_bit()),
      1 => uotghs.hstpip.modify(|_, w| w.pen1().set_bit()),
      2 => uotghs.hstpip.modify(|_, w| w.pen2().set_bit()),
      3 => uotghs.hstpip.modify(|_, w| w.pen3().set_bit()),
      4 => uotghs.hstpip.modify(|_, w| w.pen4().set_bit()),
      5 => uotghs.hstpip.modify(|_, w| w.pen5().set_bit()),
      6 => uotghs.hstpip.modify(|_, w| w.pen6().set_bit()),
      7 => uotghs.hstpip.modify(|_, w| w.pen7().set_bit()),
      8 => uotghs.hstpip.modify(|_, w| w.pen8().set_bit()),
      _ => panic!("Pipe index out of bounds"),
    }
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
    self.0.configure(address, 0);

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
