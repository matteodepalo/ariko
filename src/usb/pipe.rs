use crate::peripherals::Peripherals;
use crate::serial::Serial;
use crate::usb::packet::{DataInPacket, DataOutPacket, Packet, SetupPacket, SetupRequestDirection};
use core::cmp::min;
use core::fmt::Write;
use sam3x8e_hal::pac::uotghs::{HSTPIPCFG, HSTPIPISR};
use sam3x8e_hal::pac::UOTGHS;

const PIPE_SIZE: usize = 64;

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

  pub fn release(&self) {}

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

  pub fn configure(&self, address: u8, endpoint: u8, frequency: u8) {
    self
      .allocated_pipe()
      .configure(address, endpoint, frequency)
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

  pub fn transfer(&self, packet: &Packet) {
    self.hstpipcfg().modify(|_, w| match packet {
      Packet::Setup(_) => w.ptoken().setup(),
      Packet::DataIn(_) => w.ptoken().in_(),
      Packet::DataOut(_) => w.ptoken().out(),
    });

    match packet {
      Packet::DataOut(packet) => {
        for i in 0..(packet.0.len() / PIPE_SIZE) {
          let start = i * PIPE_SIZE;
          let end = min(packet.0.len(), start + PIPE_SIZE);
          let packet = DataOutPacket(&packet.0[start..end]);

          packet.send(self.0)
        }
      }
      Packet::DataIn(packet) => packet.receive(),
      Packet::Setup(packet) => packet.send(self.0),
    }
  }

  pub fn configure(&self, address: u8, endpoint: u8, frequency: u8) {
    let serial = Serial::get();
    let uotghs = self.uotghs();
    let hstaddr1 = &uotghs.hstaddr1;
    let hstaddr2 = &uotghs.hstaddr2;
    let hstaddr3 = &uotghs.hstaddr3;

    serial
      .write_fmt(format_args!(
        "[USB :: Pipe] Configuring pipe #{} with address: {}, endpoint: {}, frequency: {}\n\r",
        self.0, address, endpoint, frequency
      ))
      .unwrap();

    self
      .hstpipcfg()
      .write_with_zero(|w| unsafe { w.pepnum().bits(endpoint).intfrq().bits(frequency) });

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
    let hstpipcfg = self.hstpipcfg();

    serial
      .write_fmt(format_args!(
        "[USB :: Pipe] Allocating pipe #{}\n\r",
        self.0
      ))
      .unwrap();

    self.enable();

    hstpipcfg.write_with_zero(|w| w.psize()._64_byte().pbk()._1_bank().autosw().set_bit());
    hstpipcfg.modify(|_, w| w.alloc().set_bit())
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

  pub fn control_transfer(&self, address: u8, setup_packet: &SetupPacket, data: Option<&mut [u8]>) {
    let mut setup_packet = setup_packet.clone();

    if let Some(data) = &data {
      setup_packet.length = data.len() as u16
    }

    self.0.configure(address, 0, 0);
    self.0.transfer(&Packet::Setup(&setup_packet));

    match data {
      Some(data) => {
        let packet = match setup_packet.request_type.direction() {
          SetupRequestDirection::HostToDevice => Packet::DataOut(DataOutPacket(data)),
          SetupRequestDirection::DeviceToHost => Packet::DataIn(DataInPacket(data)),
        };

        self.0.transfer(&packet)
      }
      None => (),
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
