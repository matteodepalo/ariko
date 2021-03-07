use embedded_hal::blocking::i2c::Write;
use sam3x8e_hal::pmc::Clocks;

const XMIT_TIMEOUT: u32 = 100000;

pub struct I2c {
  twi0: sam3x8e_hal::pac::TWI0,
}

impl I2c {
  pub fn new(twi0: sam3x8e_hal::pac::TWI0, address: u8, clocks: &Clocks) -> Self {
    twi0
      .mmr
      .write_with_zero(|w| unsafe { w.dadr().bits(address) });

    let mut ck_div: u32 = 0;
    let mut cl_div: u32;

    loop {
      cl_div = ((clocks.master_clk().0 / (2 * clocks.master_clk().0)) - 4) / (1 << ck_div);

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

    Self { twi0 }
  }

  fn wait_byte_sent(&mut self) -> Result<(), <I2c as Write>::Error> {
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

impl Write for I2c {
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
