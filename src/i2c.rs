use embedded_hal::blocking::i2c::Write;

const XMIT_TIMEOUT: u32 = 100_000;

pub struct I2c {
  twi0: sam3x8e_hal::pac::TWI0,
}

impl I2c {
  pub fn new(twi0: sam3x8e_hal::pac::TWI0) -> Self {
    twi0.cr.write_with_zero(|w| w.swrst().set_bit());

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
