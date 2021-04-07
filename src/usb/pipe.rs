use crate::peripherals::Peripherals;
use crate::serial::Serial;
use crate::usb::packet::{DataInPacket, DataOutPacket, Packet, SetupPacket, SetupRequestDirection};
use crate::usb::Error;
use core::cmp::min;
use core::fmt::Write;
use sam3x8e_hal::pac::uotghs::{HSTPIPCFG, HSTPIPIER, HSTPIPISR};
use sam3x8e_hal::pac::UOTGHS;

const PIPE_SIZE: usize = 64;

pub struct InnerPipe(u8);
pub struct MessagePipe(InnerPipe);
pub struct StreamInPipe(InnerPipe);
pub struct StreamOutPipe(InnerPipe);

pub enum Pipe {
  Unconfigured(InnerPipe),
  Message(MessagePipe),
  StreamIn(StreamInPipe),
  StreamOut(StreamOutPipe),
}

#[derive(Debug)]
pub enum Transfer {
  Control,
  Bulk,
  Interrupt(u8),
  Isochronous,
}

impl Pipe {
  pub fn new(index: u8) -> Self {
    Self::Unconfigured(InnerPipe::new(index))
  }

  pub fn into_message(self) -> Self {
    match self {
      Self::Message(_) => self,
      _ => Self::Message(MessagePipe::new(self.unwrap_inner_pipe())),
    }
  }

  pub fn into_stream_in(self) -> Self {
    match self {
      Self::StreamIn(_) => self,
      _ => Self::StreamIn(StreamInPipe::new(self.unwrap_inner_pipe())),
    }
  }

  pub fn into_stream_out(self) -> Self {
    match self {
      Self::StreamOut(_) => self,
      _ => Self::StreamOut(StreamOutPipe::new(self.unwrap_inner_pipe())),
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

  pub fn configure(&mut self, address: u8, endpoint: u8, transfer: Transfer) -> &Self {
    let pipe = self.inner_pipe();

    pipe.configure(address, endpoint, transfer);

    self
  }

  pub fn index(&self) -> u8 {
    self.inner_pipe().0
  }

  pub fn inner_pipe(&self) -> &InnerPipe {
    match self {
      Self::Unconfigured(pipe) => pipe,
      Self::Message(pipe) => &pipe.0,
      Self::StreamIn(pipe) => &pipe.0,
      Self::StreamOut(pipe) => &pipe.0,
    }
  }

  fn unwrap_inner_pipe(self) -> InnerPipe {
    match self {
      Self::Unconfigured(pipe) => pipe,
      Self::Message(pipe) => pipe.0,
      Self::StreamIn(pipe) => pipe.0,
      Self::StreamOut(pipe) => pipe.0,
    }
  }
}

impl InnerPipe {
  pub fn new(index: u8) -> Self {
    let mut pipe = Self(index);
    pipe.alloc();
    pipe
  }

  fn transfer(&self, packet: &mut Packet) -> Result<(), Error> {
    self.hstpipcfg().modify(|_, w| match packet {
      Packet::Setup(_) => w.ptoken().setup(),
      Packet::DataIn(_) => w.ptoken().in_(),
      Packet::DataOut(_) => w.ptoken().out(),
    });

    self.hstpipier().write_with_zero(|w| w.pfreezes().set_bit());

    match packet {
      Packet::DataOut(_) => Ok(for i in 0..(packet.len() / PIPE_SIZE) {
        let start = i * PIPE_SIZE;
        let end = min(packet.len(), start + PIPE_SIZE);
        let mut slice = DataOutPacket::new(packet.slice(start..end));

        slice.send(self.0)?
      }),
      Packet::DataIn(packet) => packet.receive(self.0),
      Packet::Setup(packet) => packet.send(self.0),
    }
  }

  pub fn configure(&self, address: u8, endpoint: u8, transfer: Transfer) {
    let serial = Serial::get();

    serial
      .write_fmt(format_args!(
        "[USB :: Pipe] Configuring pipe #{} with address: {}, endpoint: {}, transfer: {:#?}\n\r",
        self.0, address, endpoint, transfer
      ))
      .unwrap();

    let uotghs = self.uotghs();
    let hstaddr1 = &uotghs.hstaddr1;
    let hstaddr2 = &uotghs.hstaddr2;
    let hstaddr3 = &uotghs.hstaddr3;

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
      _ => panic!(),
    };

    self
      .hstpipcfg()
      .modify(|_, w| unsafe { w.pepnum().bits(endpoint) });

    self.hstpipcfg().modify(|_, w| match transfer {
      Transfer::Control => w.ptype().ctrl(),
      Transfer::Interrupt(frequency) => unsafe { w.ptype().intrpt().intfrq().bits(frequency) },
      Transfer::Bulk => w.ptype().blk(),
      Transfer::Isochronous => w.ptype().iso(),
    });

    if self.hstpipisr().read().cfgok().bit_is_clear() {
      panic!("Failed to configure pipe")
    }
  }

  pub fn release(&self) {
    Serial::get()
      .write_fmt(format_args!("[USB :: Pipe] Releasing pipe #{}\n\r", self.0))
      .unwrap();

    let hstpip = &self.uotghs().hstpip;

    match self.0 {
      0 => hstpip.modify(|_, w| w.pen0().clear_bit().prst0().set_bit()),
      1 => hstpip.modify(|_, w| w.pen1().clear_bit().prst1().set_bit()),
      2 => hstpip.modify(|_, w| w.pen2().clear_bit().prst2().set_bit()),
      3 => hstpip.modify(|_, w| w.pen3().clear_bit().prst3().set_bit()),
      4 => hstpip.modify(|_, w| w.pen4().clear_bit().prst4().set_bit()),
      5 => hstpip.modify(|_, w| w.pen5().clear_bit().prst5().set_bit()),
      6 => hstpip.modify(|_, w| w.pen6().clear_bit().prst6().set_bit()),
      7 => hstpip.modify(|_, w| w.pen7().clear_bit().prst7().set_bit()),
      8 => hstpip.modify(|_, w| w.pen8().clear_bit().prst8().set_bit()),
      _ => panic!(),
    }

    self.hstpipcfg().modify(|_, w| w.alloc().clear_bit());
  }

  fn alloc(&mut self) {
    let hstpip = &self.uotghs().hstpip;

    Serial::get()
      .write_fmt(format_args!(
        "[USB :: Pipe] Allocating pipe #{}\n\r",
        self.0
      ))
      .unwrap();

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

    self
      .hstpipcfg()
      .modify(|_, w| w.psize()._64_byte().pbk()._1_bank().alloc().set_bit());
  }

  fn hstpipcfg(&self) -> &HSTPIPCFG {
    &self.uotghs().hstpipcfg()[self.0 as usize]
  }

  fn hstpipisr(&self) -> &HSTPIPISR {
    &self.uotghs().hstpipisr()[self.0 as usize]
  }

  fn hstpipier(&self) -> &HSTPIPIER {
    &self.uotghs().hstpipier()[self.0 as usize]
  }

  fn uotghs(&self) -> &UOTGHS {
    &self.peripherals().uotghs
  }

  fn peripherals(&self) -> &Peripherals {
    unsafe { Peripherals::get() }
  }
}

impl MessagePipe {
  pub fn new(pipe: InnerPipe) -> Self {
    Self(pipe)
  }

  pub fn inner_pipe(&self) -> &InnerPipe {
    &self.0
  }

  pub fn control_transfer(
    &self,
    address: u8,
    setup_packet: &SetupPacket,
    data: Option<&mut [u8]>,
  ) -> Result<(), Error> {
    let mut setup_packet = setup_packet.clone();
    let direction = setup_packet.request_type.direction();
    let pipe = self.inner_pipe();

    if let Some(data) = &data {
      setup_packet.length = data.len() as u16
    }

    Serial::get()
      .write_fmt(format_args!(
        "[USB :: Pipe] Control transfer at address: {}, Setup packet: {:?}\n\r",
        address, setup_packet
      ))
      .unwrap();

    pipe.configure(address, 0, Transfer::Control);
    pipe.transfer(&mut Packet::Setup(setup_packet))?;

    match data {
      Some(data) => {
        let mut packet = match direction {
          SetupRequestDirection::HostToDevice => Packet::DataOut(DataOutPacket::new(data)),
          SetupRequestDirection::DeviceToHost => Packet::DataIn(DataInPacket::new(data)),
        };

        pipe.transfer(&mut packet)?
      }
      None => (),
    }

    match direction {
      SetupRequestDirection::HostToDevice => {
        pipe.transfer(&mut Packet::DataIn(DataInPacket::empty()))
      }
      SetupRequestDirection::DeviceToHost => {
        pipe.transfer(&mut Packet::DataOut(DataOutPacket::empty()))
      }
    }
  }
}

impl StreamInPipe {
  pub fn new(pipe: InnerPipe) -> Self {
    Self(pipe)
  }

  pub fn in_transfer(&self, data: &mut [u8]) -> Result<(), Error> {
    self
      .0
      .transfer(&mut Packet::DataIn(DataInPacket::new(data)))
  }
}

impl StreamOutPipe {
  pub fn new(pipe: InnerPipe) -> Self {
    Self(pipe)
  }

  pub fn out_transfer(&self, data: &mut [u8]) -> Result<(), Error> {
    self
      .0
      .transfer(&mut Packet::DataOut(DataOutPacket::new(data)))
  }
}
