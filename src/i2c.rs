use embedded_hal::blocking::i2c::Write;
use sam3x8e_hal::pmc::Clocks;

const XMIT_TIMEOUT: u32 = 100_000;
const FAST_MODE_HZ: u32 = 400_000;

pub struct I2c {
  twi0: sam3x8e_hal::pac::TWI0,
}

impl I2c {
  pub fn new(twi0: sam3x8e_hal::pac::TWI0, clocks: &Clocks) -> Self {
    twi0.cr.write_with_zero(|w| w.swrst().set_bit());

    twi0
      .cr
      .write_with_zero(|w| w.svdis().set_bit().msen().set_bit());

    let mut ck_div: u32 = 0;
    let mut cl_div: u32;

    loop {
      cl_div = ((clocks.master_clk().0 / (2 * FAST_MODE_HZ)) - 8) / (1 << ck_div);

      if cl_div <= 255 {
        break;
      } else {
        ck_div += 1;
      }
    }

    twi0
      .cwgr
      .write_with_zero(|w| unsafe { w.ckdiv().bits(ck_div as u8).cldiv().bits(cl_div as u8) });

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
      .write_with_zero(|w| unsafe { w.dadr().bits(address as u8) });

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
