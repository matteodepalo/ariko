use modular_bitfield::prelude::*;

const DPRAM_BASE: *mut [u8; 0x8000] = 0x20180000 as *mut [u8; 0x8000];

use crate::serial::Serial;
use core::fmt::Write;
use core::mem::size_of;
use core::ptr::write_volatile;
use core::slice;

#[repr(C)]
pub struct DataInPacket<'a>(pub &'a [u8]);

#[repr(C)]
pub struct DataOutPacket<'a>(pub &'a [u8]);

#[derive(BitfieldSpecifier)]
#[bits = 1]
pub enum SetupRequestDirection {
  HostToDevice,
  DeviceToHost,
}

#[derive(BitfieldSpecifier)]
#[bits = 2]
pub enum SetupRequestKind {
  Standard,
  Class,
  Vendor,
}

#[derive(BitfieldSpecifier)]
#[bits = 5]
pub enum SetupRequestRecipient {
  Device,
  Interface,
  Endpoint,
  Other,
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct SetupRequestType {
  pub recipient: SetupRequestRecipient,
  pub kind: SetupRequestKind,
  pub direction: SetupRequestDirection,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetupPacket {
  pub request_type: SetupRequestType,
  pub request: u8,
  pub value: u16,
  pub index: u16,
  pub length: u16,
}

pub enum Packet<'a> {
  DataIn(DataInPacket<'a>),
  DataOut(DataOutPacket<'a>),
  Setup(&'a SetupPacket),
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
  pub fn new(request_type: SetupRequestType, request: u8, value: u16, index: u16) -> Self {
    SetupPacket {
      request_type,
      request,
      value,
      index,
      length: 0,
    }
  }

  pub fn send(&self, index: u8) {
    let serial = Serial::get();
    let data_pointer = self as *const Self as *const u8;
    let data = unsafe { slice::from_raw_parts(data_pointer, size_of::<Self>()) };

    serial
      .write_fmt(format_args!("[USB :: Packet] Sending Setup packet\n\r"))
      .unwrap();

    DataOutPacket(data).send(index)
  }
}

impl<'a> DataOutPacket<'a> {
  pub fn send(&self, index: u8) {
    let serial = Serial::get();
    let fifo = unsafe { &mut *DPRAM_BASE.offset(index as isize) };

    serial
      .write_fmt(format_args!(
        "[USB :: Packet] Sending Data packet ({} bytes)\n\r",
        self.0.len()
      ))
      .unwrap();

    for (i, byte) in self.0.iter().enumerate() {
      unsafe { write_volatile(fifo.as_mut_ptr().offset(i as isize), *byte) }
    }
  }
}

impl<'a> DataInPacket<'a> {
  pub fn receive(&self) {
    let serial = Serial::get();

    serial
      .write_fmt(format_args!(
        "[USB :: Packet] Receiving Data packet ({} bytes)\n\r",
        self.0.len()
      ))
      .unwrap();
  }
}
