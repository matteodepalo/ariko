use crate::usb::packet::{DataInPacket, DataOutPacket, Packet, SetupPacket, SetupRequestDirection};
use crate::usb::Error;
use core::cmp::min;
use sam3x8e_hal::pac::{uotghs, UOTGHS};

const PIPE_SIZE: usize = 64;

// Macro to handle indexed pipe configuration register access
macro_rules! with_hstpipcfg {
    ($uotghs:expr, $index:expr, |$cfg:ident| $body:expr) => {
        match $index {
            0 => { let $cfg = $uotghs.hstpipcfg0(); $body }
            1 => { let $cfg = $uotghs.hstpipcfg1(); $body }
            2 => { let $cfg = $uotghs.hstpipcfg2(); $body }
            3 => { let $cfg = $uotghs.hstpipcfg3(); $body }
            4 => { let $cfg = $uotghs.hstpipcfg4(); $body }
            5 => { let $cfg = $uotghs.hstpipcfg5(); $body }
            6 => { let $cfg = $uotghs.hstpipcfg6(); $body }
            7 => { let $cfg = $uotghs.hstpipcfg7(); $body }
            8 => { let $cfg = $uotghs.hstpipcfg8(); $body }
            _ => panic!("Pipe index out of bounds"),
        }
    };
}

macro_rules! with_hstpipisr {
    ($uotghs:expr, $index:expr, |$isr:ident| $body:expr) => {
        match $index {
            0 => { let $isr = $uotghs.hstpipisr0(); $body }
            1 => { let $isr = $uotghs.hstpipisr1(); $body }
            2 => { let $isr = $uotghs.hstpipisr2(); $body }
            3 => { let $isr = $uotghs.hstpipisr3(); $body }
            4 => { let $isr = $uotghs.hstpipisr4(); $body }
            5 => { let $isr = $uotghs.hstpipisr5(); $body }
            6 => { let $isr = $uotghs.hstpipisr6(); $body }
            7 => { let $isr = $uotghs.hstpipisr7(); $body }
            8 => { let $isr = $uotghs.hstpipisr8(); $body }
            _ => panic!("Pipe index out of bounds"),
        }
    };
}

macro_rules! with_hstpipier {
    ($uotghs:expr, $index:expr, |$ier:ident| $body:expr) => {
        match $index {
            0 => { let $ier = $uotghs.hstpipier0(); $body }
            1 => { let $ier = $uotghs.hstpipier1(); $body }
            2 => { let $ier = $uotghs.hstpipier2(); $body }
            3 => { let $ier = $uotghs.hstpipier3(); $body }
            4 => { let $ier = $uotghs.hstpipier4(); $body }
            5 => { let $ier = $uotghs.hstpipier5(); $body }
            6 => { let $ier = $uotghs.hstpipier6(); $body }
            7 => { let $ier = $uotghs.hstpipier7(); $body }
            8 => { let $ier = $uotghs.hstpipier8(); $body }
            _ => panic!("Pipe index out of bounds"),
        }
    };
}

#[derive(Debug)]
pub struct InnerPipe(u8);
#[derive(Debug)]
pub struct MessagePipe(InnerPipe);
#[derive(Debug)]
pub struct StreamInPipe(InnerPipe);
#[derive(Debug)]
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

  fn transfer(&self, packet: &mut Packet) -> Result<usize, Error> {
    let uotghs = self.uotghs();
    with_hstpipcfg!(uotghs, self.0, |cfg| {
      cfg.modify(|_, w| match packet {
        Packet::Setup(_) => w.ptoken().setup(),
        Packet::DataIn(_) => w.ptoken().in_(),
        Packet::DataOut(_) => w.ptoken().out(),
      })
    });

    with_hstpipier!(uotghs, self.0, |ier| {
      unsafe { ier.write_with_zero(|w| w.pfreezes().set_bit()) }
    });

    match packet {
      Packet::DataOut(_) => {
        // Use ceiling division to ensure packets smaller than PIPE_SIZE are sent
        let num_chunks = (packet.len() + PIPE_SIZE - 1) / PIPE_SIZE;
        for i in 0..num_chunks {
          let start = i * PIPE_SIZE;
          let end = min(packet.len(), start + PIPE_SIZE);
          let mut slice = DataOutPacket::new(packet.slice(start..end));
          slice.send(self.0)?
        }
        Ok(packet.len())
      }
      Packet::DataIn(packet) => packet.receive(self.0),
      Packet::Setup(packet) => {
        packet.send(self.0)?;
        Ok(8)
      }
    }
  }

  pub fn configure(&self, address: u8, endpoint: u8, transfer: Transfer) {

    let uotghs = self.uotghs();
    let hstaddr1 = uotghs.hstaddr1();
    let hstaddr2 = uotghs.hstaddr2();
    let hstaddr3 = uotghs.hstaddr3();

    // Use modify instead of write to preserve other pipe addresses
    match self.0 {
      0 => hstaddr1.modify(|_, w| unsafe { w.hstaddrp0().bits(address) }),
      1 => hstaddr1.modify(|_, w| unsafe { w.hstaddrp1().bits(address) }),
      2 => hstaddr1.modify(|_, w| unsafe { w.hstaddrp2().bits(address) }),
      3 => hstaddr1.modify(|_, w| unsafe { w.hstaddrp3().bits(address) }),
      4 => hstaddr2.modify(|_, w| unsafe { w.hstaddrp4().bits(address) }),
      5 => hstaddr2.modify(|_, w| unsafe { w.hstaddrp5().bits(address) }),
      6 => hstaddr2.modify(|_, w| unsafe { w.hstaddrp6().bits(address) }),
      7 => hstaddr2.modify(|_, w| unsafe { w.hstaddrp7().bits(address) }),
      8 => hstaddr3.modify(|_, w| unsafe { w.hstaddrp8().bits(address) }),
      _ => panic!(),
    };

    with_hstpipcfg!(uotghs, self.0, |cfg| {
      cfg.modify(|_, w| unsafe { w.pepnum().bits(endpoint) })
    });

    with_hstpipcfg!(uotghs, self.0, |cfg| {
      cfg.modify(|_, w| match transfer {
        Transfer::Control => w.ptype().ctrl(),
        Transfer::Interrupt(frequency) => unsafe { w.ptype().intrpt().intfrq().bits(frequency) },
        Transfer::Bulk => w.ptype().blk(),
        Transfer::Isochronous => w.ptype().iso(),
      })
    });

    let cfgok = with_hstpipisr!(uotghs, self.0, |isr| {
      isr.read().cfgok().bit_is_clear()
    });
    if cfgok {
      panic!("Failed to configure pipe")
    }
  }

  pub fn release(&self) {

    let uotghs = self.uotghs();
    with_hstpipcfg!(uotghs, self.0, |cfg| {
      cfg.modify(|_, w| w.alloc().clear_bit())
    })
  }

  fn alloc(&mut self) {
    let uotghs = self.uotghs();
    let hstpip = uotghs.hstpip();


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

    with_hstpipcfg!(uotghs, self.0, |cfg| {
      cfg.modify(|_, w| w.psize()._64_byte().pbk()._1_bank().alloc().set_bit())
    });
  }

  fn uotghs(&self) -> &uotghs::RegisterBlock {
    // Use pointer to get a static reference to the UOTGHS peripheral
    unsafe { &*UOTGHS::ptr() }
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


    pipe.configure(address, 0, Transfer::Control);
    pipe.transfer(&mut Packet::Setup(setup_packet))?;

    match data {
      Some(data) => {
        let mut packet = match direction {
          SetupRequestDirection::HostToDevice => Packet::DataOut(DataOutPacket::new(data)),
          SetupRequestDirection::DeviceToHost => Packet::DataIn(DataInPacket::new(data)),
        };

        pipe.transfer(&mut packet)?;

        match direction {
          SetupRequestDirection::HostToDevice => {
            pipe.transfer(&mut Packet::DataIn(DataInPacket::empty()))?;
          }
          SetupRequestDirection::DeviceToHost => {
            pipe.transfer(&mut Packet::DataOut(DataOutPacket::empty()))?;
          }
        }
      }
      None => {
        pipe.transfer(&mut Packet::DataIn(DataInPacket::empty()))?;
      }
    }
    Ok(())
  }
}

impl StreamInPipe {
  pub fn new(pipe: InnerPipe) -> Self {
    Self(pipe)
  }

  pub fn in_transfer(&self, data: &mut [u8]) -> Result<usize, Error> {
    self
      .0
      .transfer(&mut Packet::DataIn(DataInPacket::new(data)))
  }
}

impl StreamOutPipe {
  pub fn new(pipe: InnerPipe) -> Self {
    Self(pipe)
  }

  pub fn out_transfer(&self, data: &mut [u8]) -> Result<usize, Error> {
    self
      .0
      .transfer(&mut Packet::DataOut(DataOutPacket::new(data)))
  }
}
