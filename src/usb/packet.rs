use crate::peripherals::Peripherals;
use crate::usb::Error;
use core::mem::size_of;
use core::ops::Range;
use core::ptr::write_volatile;
use core::slice;
use embedded_hal::timer::CountDown;
use log::debug;
use modular_bitfield::prelude::*;
use sam3x8e_hal::pac::uotghs::{HSTPIPICR, HSTPIPIDR, HSTPIPIER, HSTPIPINRQ, HSTPIPISR};
use sam3x8e_hal::pac::{RTT, UOTGHS};
use sam3x8e_hal::time::U32Ext;
use sam3x8e_hal::timer::Timer;

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
    self.timer().try_start(TRANSFER_TIMEOUT.hz()).unwrap();

    for (i, byte) in self.data.iter().enumerate() {
      unsafe { write_volatile(self.fifo().as_mut_ptr().offset(i as isize), *byte) }
    }

    self.hstpipicr().write_with_zero(|w| {
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
    });

    self
      .hstpipidr()
      .write_with_zero(|w| w.fifoconc().set_bit().pfreezec().set_bit())
  }

  fn receive(&mut self) -> Result<(), Error> {
    debug!(
      "[USB :: Packet] Receiving Data packet ({} bytes)",
      self.data.len()
    );

    self.timer().try_start(TRANSFER_TIMEOUT.hz()).unwrap();

    self
      .hstpipinrq()
      .write_with_zero(|w| unsafe { w.inmode().clear_bit().inrq().bits(0) });

    self.hstpipidr().write_with_zero(|w| w.pfreezec().set_bit());
    while self.timer().try_wait().is_err() && self.hstpipisr().read().rxini().bit_is_clear() {}

    if self.hstpipisr().read().rxini().bit_is_set() {
      for i in 0..self.hstpipisr().read().pbyct().bits() {
        self.data[i as usize] = self.fifo()[i as usize]
      }

      self.hstpipicr().write_with_zero(|w| w.rxinic().set_bit());
      self.hstpipidr().write_with_zero(|w| w.fifoconc().set_bit());

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

  fn peripherals(&self) -> &mut Peripherals {
    unsafe { Peripherals::get() }
  }

  fn timer(&mut self) -> &mut Timer<RTT> {
    &mut self.peripherals().timer
  }

  fn uotghs(&self) -> &UOTGHS {
    &self.peripherals().uotghs
  }

  fn hstpipicr(&self) -> &HSTPIPICR {
    &self.uotghs().hstpipicr()[self.index as usize]
  }

  fn hstpipidr(&self) -> &HSTPIPIDR {
    &self.uotghs().hstpipidr()[self.index as usize]
  }

  fn hstpipier(&self) -> &HSTPIPIER {
    &self.uotghs().hstpipier()[self.index as usize]
  }

  fn hstpipisr(&self) -> &HSTPIPISR {
    &self.uotghs().hstpipisr()[self.index as usize]
  }

  fn hstpipinrq(&self) -> &HSTPIPINRQ {
    &self.uotghs().hstpipinrq[self.index as usize]
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

    while message.timer().try_wait().is_err() {
      if message.hstpipisr().read().txstpi().bit_is_set() {
        message
          .hstpipicr()
          .write_with_zero(|w| w.txstpic().set_bit());

        message
          .hstpipier()
          .write_with_zero(|w| w.pfreezes().set_bit());

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

    while message.timer().try_wait().is_err() {
      if message.hstpipisr().read().txouti().bit_is_set() {
        message
          .hstpipicr()
          .write_with_zero(|w| w.txoutic().set_bit());

        message
          .hstpipier()
          .write_with_zero(|w| w.pfreezes().set_bit());

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
