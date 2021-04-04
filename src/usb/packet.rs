use modular_bitfield::prelude::*;

const DPRAM_BASE: *mut [u8; 0x8000] = 0x20180000 as *mut [u8; 0x8000];
const TRANSFER_TIMEOUT: u32 = 5000;

use crate::peripherals::Peripherals;
use crate::serial::Serial;
use crate::usb::{Error, VBusState, USB};
use core::fmt::Write;
use core::mem::size_of;
use core::ptr::write_volatile;
use core::slice;
use embedded_hal::timer::CountDown;
use sam3x8e_hal::pac::uotghs::{HSTPIPICR, HSTPIPIDR, HSTPIPIER, HSTPIPIMR, HSTPIPISR};
use sam3x8e_hal::pac::{RTT, UOTGHS};
use sam3x8e_hal::time::U32Ext;
use sam3x8e_hal::timer::Timer;

#[repr(C)]
pub struct DataInPacket<'a>(pub &'a mut [u8]);

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

impl<'a> Packet<'a> {
  fn send(index: u8, data: &[u8]) -> &mut Timer<RTT> {
    let timer = unsafe { &mut Peripherals::get().timer };
    let fifo = Self::fifo(index);
    let hstpipicr = Self::hstpipicr(index);
    let hstpipidr = Self::hstpipidr(index);

    timer.try_start(TRANSFER_TIMEOUT.hz()).unwrap();

    for (i, byte) in data.iter().enumerate() {
      unsafe { write_volatile(fifo.as_mut_ptr().offset(i as isize), *byte) }
    }

    hstpipicr.write_with_zero(|w| {
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

    hstpipidr.write_with_zero(|w| w.fifoconc().set_bit().pfreezec().set_bit());
    timer
  }

  fn hstpipicr(index: u8) -> &'a HSTPIPICR {
    &Self::uotghs().hstpipicr()[index as usize]
  }

  fn hstpipidr(index: u8) -> &'a HSTPIPIDR {
    &Self::uotghs().hstpipidr()[index as usize]
  }

  fn hstpipier(index: u8) -> &'a HSTPIPIER {
    &Self::uotghs().hstpipier()[index as usize]
  }

  fn hstpipisr(index: u8) -> &'a HSTPIPISR {
    &Self::uotghs().hstpipisr()[index as usize]
  }

  fn hstpipimr(index: u8) -> &'a HSTPIPIMR {
    &Self::uotghs().hstpipimr()[index as usize]
  }

  fn fifo(index: u8) -> &'a mut [u8; 0x8000] {
    unsafe { &mut *DPRAM_BASE.offset(index as isize) }
  }

  fn uotghs() -> &'a UOTGHS {
    unsafe { &Peripherals::get().uotghs }
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
  pub fn new(request_type: SetupRequestType, request: u8, value: u16, index: u16) -> Self {
    SetupPacket {
      request_type,
      request,
      value,
      index,
      length: 0,
    }
  }

  pub fn send(&self, index: u8) -> Result<(), Error> {
    let data_pointer = self as *const Self as *const u8;
    let data = unsafe { slice::from_raw_parts(data_pointer, size_of::<Self>()) };

    Serial::get()
      .write_fmt(format_args!("[USB :: Packet] Sending Setup packet\n\r"))
      .unwrap();

    let timer = Packet::send(index, data);
    let mut result = Err(Error::TransferTimeout);

    while USB::get().vbus_state == VBusState::Connected && timer.try_wait().is_err() {
      if Packet::hstpipisr(index).read().txstpi().bit_is_set() {
        Packet::hstpipier(index).write_with_zero(|w| w.pfreezes().set_bit());
        Packet::hstpipicr(index).write_with_zero(|w| w.txstpic().set_bit());

        result = Ok(());
        break;
      }
    }

    result
  }
}

impl<'a> DataOutPacket<'a> {
  pub fn send(&self, index: u8) -> Result<(), Error> {
    Serial::get()
      .write_fmt(format_args!(
        "[USB :: Packet] Sending Data packet ({} bytes)\n\r",
        self.0.len()
      ))
      .unwrap();

    let timer = Packet::send(index, self.0);
    let mut result = Err(Error::TransferTimeout);

    while USB::get().vbus_state == VBusState::Connected && timer.try_wait().is_err() {
      if Packet::hstpipisr(index).read().txouti().bit_is_set() {
        Packet::hstpipier(index).write_with_zero(|w| w.pfreezes().set_bit());
        Packet::hstpipicr(index).write_with_zero(|w| w.txoutic().set_bit());

        result = Ok(());
        break;
      }
    }

    result
  }
}

impl<'a> DataInPacket<'a> {
  pub fn receive(&mut self, index: u8) -> Result<(), Error> {
    Serial::get()
      .write_fmt(format_args!(
        "[USB :: Packet] Receiving Data packet ({} bytes)\n\r",
        self.0.len()
      ))
      .unwrap();

    let timer = Packet::send(index, &[]);
    let fifo = Packet::fifo(index);
    let mut bytes_received = 0;

    self.wait_transfer_complete(index, timer)?;
    timer.try_start(TRANSFER_TIMEOUT.hz()).unwrap();

    while bytes_received < self.0.len() && timer.try_wait().is_err() {
      let byte_count = Packet::hstpipisr(index).read().pbyct().bits();

      for i in 0..byte_count {
        bytes_received += 1;
        self.0[i as usize] = fifo[i as usize]
      }
    }

    if timer.try_wait().is_ok() {
      Err(Error::TransferTimeout)
    } else {
      Ok(())
    }
  }

  fn wait_transfer_complete(&mut self, index: u8, timer: &mut Timer<RTT>) -> Result<(), Error> {
    let mut result = Err(Error::TransferTimeout);

    while USB::get().vbus_state == VBusState::Connected && timer.try_wait().is_err() {
      if Packet::hstpipisr(index).read().rxini().bit_is_set() {
        while Packet::hstpipimr(index).read().pfreeze().bit_is_clear() {}

        Packet::hstpipier(index).write_with_zero(|w| w.pfreezes().set_bit());
        Packet::hstpipicr(index).write_with_zero(|w| w.rxinic().set_bit());

        result = Ok(());
        break;
      }
    }

    result
  }
}
