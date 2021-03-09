use crate::serial::Serial;
use core::fmt::Write;
use embedded_hal::blocking::i2c::Write as I2cWrite;
use sam3x8e_hal::pmc::Clocks;

const XMIT_TIMEOUT: u32 = 100_000;
const FAST_MODE_HZ: u32 = 100_000;
const LOW_LEVEL_TIME_LIMIT: u32 = 384_000;
const TWI_CLK_DIVIDER: u32 = 2;
const TWI_CLK_CALC_ARGU: u32 = 3;
const TWI_CLK_DIV_MAX: u32 = 0xFF;
const TWI_CLK_DIV_MIN: u32 = 7;

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
    twi0.cr.write_with_zero(|w| w.swrst().set_bit());

    Self { twi0, serial }
  }

  fn wait_byte_sent(&mut self) -> Result<(), <I2c as I2cWrite>::Error> {
    let mut timeout = XMIT_TIMEOUT;

    loop {
      if self.twi0.sr.read().txrdy().bit_is_set() {
        break;
      }

      if self.twi0.sr.read().nack().bit_is_set() {
        return Err(());
      }

      timeout -= 1;

      if timeout == 0 {
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
      .mmr
      .write_with_zero(|w| unsafe { w.dadr().bits(address as u8).iadrsz()._1_byte() });

    self
      .twi0
      .iadr
      .write_with_zero(|w| unsafe { w.iadr().bits(address as u32) });

    self
      .twi0
      .cr
      .write_with_zero(|w| w.svdis().set_bit().msen().set_bit());

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

    while self.twi0.sr.read().txcomp().bit_is_clear() {}

    Ok(())
  }
}
