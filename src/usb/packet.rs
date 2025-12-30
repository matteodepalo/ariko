use crate::peripherals::Peripherals;
use crate::usb::Error;
use core::mem::size_of;
use core::ops::Range;
use core::ptr::write_volatile;
use core::slice;
use sam3x8e_hal::timer::CountDown;
use log::debug;
use modular_bitfield::prelude::*;
use sam3x8e_hal::pac::UOTGHS;
use sam3x8e_hal::time::U32Ext;

// Macros for indexed pipe register access
macro_rules! with_hstpipicr {
    ($uotghs:expr, $index:expr, |$icr:ident| $body:expr) => {
        match $index {
            0 => { let $icr = $uotghs.hstpipicr0(); $body }
            1 => { let $icr = $uotghs.hstpipicr1(); $body }
            2 => { let $icr = $uotghs.hstpipicr2(); $body }
            3 => { let $icr = $uotghs.hstpipicr3(); $body }
            4 => { let $icr = $uotghs.hstpipicr4(); $body }
            5 => { let $icr = $uotghs.hstpipicr5(); $body }
            6 => { let $icr = $uotghs.hstpipicr6(); $body }
            7 => { let $icr = $uotghs.hstpipicr7(); $body }
            8 => { let $icr = $uotghs.hstpipicr8(); $body }
            _ => panic!("Pipe index out of bounds"),
        }
    };
}

macro_rules! with_hstpipidr {
    ($uotghs:expr, $index:expr, |$idr:ident| $body:expr) => {
        match $index {
            0 => { let $idr = $uotghs.hstpipidr0(); $body }
            1 => { let $idr = $uotghs.hstpipidr1(); $body }
            2 => { let $idr = $uotghs.hstpipidr2(); $body }
            3 => { let $idr = $uotghs.hstpipidr3(); $body }
            4 => { let $idr = $uotghs.hstpipidr4(); $body }
            5 => { let $idr = $uotghs.hstpipidr5(); $body }
            6 => { let $idr = $uotghs.hstpipidr6(); $body }
            7 => { let $idr = $uotghs.hstpipidr7(); $body }
            8 => { let $idr = $uotghs.hstpipidr8(); $body }
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

macro_rules! with_hstpipinrq {
    ($uotghs:expr, $index:expr, |$inrq:ident| $body:expr) => {
        match $index {
            0 => { let $inrq = $uotghs.hstpipinrq0(); $body }
            1 => { let $inrq = $uotghs.hstpipinrq1(); $body }
            2 => { let $inrq = $uotghs.hstpipinrq2(); $body }
            3 => { let $inrq = $uotghs.hstpipinrq3(); $body }
            4 => { let $inrq = $uotghs.hstpipinrq4(); $body }
            5 => { let $inrq = $uotghs.hstpipinrq5(); $body }
            6 => { let $inrq = $uotghs.hstpipinrq6(); $body }
            7 => { let $inrq = $uotghs.hstpipinrq7(); $body }
            8 => { let $inrq = $uotghs.hstpipinrq8(); $body }
            _ => panic!("Pipe index out of bounds"),
        }
    };
}

const DPRAM_BASE: *mut Fifo = 0x20180000 as *mut Fifo;
const TRANSFER_TIMEOUT: u32 = 5000;

type Fifo = [u8; 0x8000];

#[repr(C)]
pub struct DataPacket<'a>(&'a mut [u8]);

#[repr(C)]
pub struct DataInPacket<'a>(DataPacket<'a>);

#[repr(C)]
pub struct DataOutPacket<'a>(DataPacket<'a>);

#[derive(BitfieldSpecifier, Debug)]
#[bits = 1]
pub enum SetupRequestDirection {
  HostToDevice,
  DeviceToHost,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 2]
pub enum SetupRequestKind {
  Standard,
  Class,
  Vendor,
}

#[derive(BitfieldSpecifier, Debug)]
#[bits = 5]
pub enum SetupRequestRecipient {
  Device,
  Interface,
  Endpoint,
  Other,
}

#[bitfield]
#[derive(Copy, Clone, Debug)]
pub struct SetupRequestType {
  pub recipient: SetupRequestRecipient,
  pub kind: SetupRequestKind,
  pub direction: SetupRequestDirection,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SetupPacket {
  pub request_type: SetupRequestType,
  pub request: u8,
  pub value: [u8; 2],
  pub index: u16,
  pub length: u16,
}

pub enum Packet<'a> {
  DataIn(DataInPacket<'a>),
  DataOut(DataOutPacket<'a>),
  Setup(SetupPacket),
}

struct Message<'a> {
  data: &'a mut [u8],
  index: u8,
}

impl<'a> DataPacket<'a> {
  fn new(data: &'a mut [u8]) -> Self {
    Self(data)
  }

  fn len(&self) -> usize {
    self.0.len()
  }

  fn data(&mut self) -> &mut [u8] {
    self.0
  }

  fn slice(&mut self, range: Range<usize>) -> &mut [u8] {
    &mut self.0[range]
  }

  fn empty() -> Self {
    Self::new(&mut [])
  }
}

impl<'a> Packet<'a> {
  pub fn len(&self) -> usize {
    match self {
      Self::DataIn(packet) => packet.0.len(),
      Self::DataOut(packet) => packet.0.len(),
      Self::Setup(_) => panic!(),
    }
  }

  pub fn slice(&mut self, range: Range<usize>) -> &mut [u8] {
    match self {
      Self::DataIn(packet) => packet.0.slice(range),
      Self::DataOut(packet) => packet.0.slice(range),
      Self::Setup(_) => panic!(),
    }
  }
}

impl<'a> Message<'a> {
  fn new(index: u8, data: &'a mut [u8]) -> Self {
    Self { data, index }
  }

  fn send(&mut self) {
    self.start_timer();

    for (i, byte) in self.data.iter().enumerate() {
      unsafe { write_volatile(self.fifo().as_mut_ptr().offset(i as isize), *byte) }
    }

    let uotghs = self.uotghs();
    with_hstpipicr!(uotghs, self.index, |icr| {
      unsafe {
        icr.write_with_zero(|w| {
          w.txstpic()
            .set_bit()
            .rxinic()
            .set_bit()
            .txoutic()
            .set_bit()
            .shortpacketic()
            .set_bit()
            .nakedic()
            .set_bit()
        })
      }
    });

    with_hstpipidr!(uotghs, self.index, |idr| {
      unsafe { idr.write_with_zero(|w| w.fifoconc().set_bit().pfreezec().set_bit()) }
    })
  }

  fn receive(&mut self) -> Result<(), Error> {
    debug!(
      "[USB :: Packet] Receiving Data packet ({} bytes)",
      self.data.len()
    );

    self.start_timer();

    let index = self.index;
    // Use pointer to avoid holding a borrow
    let uotghs = unsafe { &*UOTGHS::ptr() };
    with_hstpipinrq!(uotghs, index, |inrq| {
      unsafe { inrq.write_with_zero(|w| w.inmode().clear_bit().inrq().bits(0)) }
    });

    with_hstpipidr!(uotghs, index, |idr| {
      unsafe { idr.write_with_zero(|w| w.pfreezec().set_bit()) }
    });

    loop {
      if self.check_timer() {
        break;
      }
      let rxini_clear = with_hstpipisr!(uotghs, index, |isr| {
        isr.read().rxini().bit_is_clear()
      });
      if !rxini_clear {
        break;
      }
    }

    let rxini_set = with_hstpipisr!(uotghs, index, |isr| {
      isr.read().rxini().bit_is_set()
    });
    if rxini_set {
      let pbyct = with_hstpipisr!(uotghs, index, |isr| {
        isr.read().pbyct().bits()
      });
      for i in 0..pbyct {
        self.data[i as usize] = self.fifo()[i as usize]
      }

      with_hstpipicr!(uotghs, index, |icr| {
        unsafe { icr.write_with_zero(|w| w.rxinic().set_bit()) }
      });
      with_hstpipidr!(uotghs, index, |idr| {
        unsafe { idr.write_with_zero(|w| w.fifoconc().set_bit()) }
      });

      debug!(
        "[USB :: Packet] Finished receiving Data packet ({} bytes)",
        self.data.len()
      );

      Ok(())
    } else {
      Err(Error::TransferTimeout)
    }
  }

  fn len(&self) -> usize {
    self.data.len()
  }

  fn start_timer(&mut self) {
    Peripherals::with(|p| {
      p.timer.try_start(TRANSFER_TIMEOUT.hz()).unwrap();
    });
  }

  fn check_timer(&mut self) -> bool {
    Peripherals::with(|p| {
      p.timer.try_wait().is_ok()
    })
  }

  fn uotghs(&self) -> &UOTGHS {
    unsafe { &*UOTGHS::ptr() }
  }

  fn fifo(&self) -> &mut Fifo {
    unsafe { &mut *DPRAM_BASE.offset(self.index as isize) }
  }
}

impl SetupRequestType {
  pub fn default() -> Self {
    Self::new()
      .with_recipient(SetupRequestRecipient::Device)
      .with_direction(SetupRequestDirection::DeviceToHost)
      .with_kind(SetupRequestKind::Standard)
  }
}

impl SetupPacket {
  pub fn new(request_type: SetupRequestType, request: u8, value: [u8; 2], index: u16) -> Self {
    Self {
      request_type,
      request,
      value,
      index,
      length: 0,
    }
  }

  pub fn send(&self, index: u8) -> Result<(), Error> {
    let data_pointer = self as *const Self as *mut u8;
    let data = unsafe { slice::from_raw_parts_mut(data_pointer, size_of::<Self>()) };
    let mut message = Message::new(index, data);

    debug!("[USB :: Packet] Sending Setup packet");

    message.send();

    let mut result = Err(Error::TransferTimeout);
    // Use pointer to avoid holding a borrow
    let uotghs = unsafe { &*UOTGHS::ptr() };

    while !message.check_timer() {
      let txstpi_set = with_hstpipisr!(uotghs, index, |isr| {
        isr.read().txstpi().bit_is_set()
      });
      if txstpi_set {
        with_hstpipicr!(uotghs, index, |icr| {
          unsafe { icr.write_with_zero(|w| w.txstpic().set_bit()) }
        });

        with_hstpipier!(uotghs, index, |ier| {
          unsafe { ier.write_with_zero(|w| w.pfreezes().set_bit()) }
        });

        debug!("[USB :: Packet] Finished sending Setup packet");

        result = Ok(());
        break;
      }
    }

    result
  }
}

impl<'a> DataOutPacket<'a> {
  pub fn new(data: &'a mut [u8]) -> Self {
    Self(DataPacket::new(data))
  }

  pub fn empty() -> Self {
    Self(DataPacket::empty())
  }

  pub fn send(&mut self, index: u8) -> Result<(), Error> {
    let mut message = Message::new(index, self.0.data());

    debug!(
      "[USB :: Packet] Sending Data packet ({} bytes)",
      message.len()
    );

    message.send();

    let mut result = Err(Error::TransferTimeout);
    // Use pointer to avoid holding a borrow
    let uotghs = unsafe { &*UOTGHS::ptr() };

    while !message.check_timer() {
      let txouti_set = with_hstpipisr!(uotghs, index, |isr| {
        isr.read().txouti().bit_is_set()
      });
      if txouti_set {
        with_hstpipicr!(uotghs, index, |icr| {
          unsafe { icr.write_with_zero(|w| w.txoutic().set_bit()) }
        });

        with_hstpipier!(uotghs, index, |ier| {
          unsafe { ier.write_with_zero(|w| w.pfreezes().set_bit()) }
        });

        debug!("[USB :: Packet] Finished sending Data packet");

        result = Ok(());
        break;
      }
    }

    result
  }
}

impl<'a> DataInPacket<'a> {
  pub fn new(data: &'a mut [u8]) -> Self {
    Self(DataPacket::new(data))
  }

  pub fn empty() -> Self {
    Self(DataPacket::empty())
  }

  pub fn receive(&mut self, index: u8) -> Result<(), Error> {
    Message::new(index, self.0.data()).receive()
  }
}
