use crate::serial::Serial;
use core::fmt::Write;
use embedded_hal::blocking::i2c::Write as I2cWrite;
use sam3x8e_hal::pmc::Clocks;

const XMIT_TIMEOUT: u32 = 100_000;
const FAST_MODE_HZ: u32 = 400_000;

pub struct I2c<'a> {
  twi0: sam3x8e_hal::pac::TWI0,
  serial: &'a mut Serial,
}

impl<'a> I2c<'a> {
  pub fn new(
    twi0: sam3x8e_hal::pac::TWI0,
    address: u32,
    clocks: &Clocks,
    serial: &'a mut Serial,
  ) -> Self {
    twi0
      .mmr
      .write_with_zero(|w| unsafe { w.dadr().bits(address as u8).iadrsz()._1_byte() });

    twi0
      .iadr
      .write_with_zero(|w| unsafe { w.iadr().bits(address >> 8) });

    let mut ck_div: u32 = 0;
    let mut cl_div: u32;

    loop {
      cl_div = ((clocks.master_clk().0 / (2 * FAST_MODE_HZ)) - 4) / (1 << ck_div);

      if cl_div <= 255 {
        break;
      } else {
        ck_div += 1;
      }
    }

    twi0
      .cwgr
      .write_with_zero(|w| unsafe { w.bits((ck_div << 16) | (cl_div << 8) | cl_div) });

    twi0
      .cr
      .write_with_zero(|w| w.svdis().set_bit().msen().set_bit());

    Self { twi0, serial }
  }

  fn wait_byte_sent(&mut self) -> Result<(), <I2c as I2cWrite>::Error> {
    let mut timeout = XMIT_TIMEOUT;

    loop {
      if self.twi0.sr.read().txrdy().bit_is_set() {
        self.serial.write_str("GOT ACK!\n").unwrap();
        break;
      }

      if self.twi0.sr.read().nack().bit_is_set() {
        self.serial.write_str("GOT NACK!\n").unwrap();
        return Err(());
      }

      timeout -= 1;

      if timeout == 0 {
        self.serial.write_str("TIMEOUT!\n").unwrap();
        return Err(());
      }
    }

    Ok(())
  }
}

impl I2cWrite for I2c<'_> {
  type Error = ();

  fn try_write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
    self
      .twi0
      .thr
      .write_with_zero(|w| unsafe { w.bits((address as u32) << 1) });

    if let Err(e) = self.wait_byte_sent() {
      return Err(e);
    }

    for byte in bytes {
      self
        .twi0
        .thr
        .write_with_zero(|w| unsafe { w.bits(*byte as u32) });

      if let Err(e) = self.wait_byte_sent() {
        return Err(e);
      }
    }

    self.twi0.cr.write_with_zero(|w| w.stop().set_bit());

    if let Err(e) = self.wait_byte_sent() {
      return Err(e);
    }

    Ok(())
  }
}
