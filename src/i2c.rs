use crate::peripherals::Peripherals;
use embedded_hal::blocking::i2c::Write;
use sam3x8e_hal::pmc::PeripheralClock;

const TRANSMIT_TIMEOUT: u32 = 100_000;
const FAST_MODE_HZ: u32 = 400_000;

static mut S_I2C: Option<I2C> = None;

pub struct I2C;

impl I2C {
  pub fn init() {
    let i2c = I2C;
    let peripherals = Peripherals::get();
    let pmc = &mut peripherals.pmc;
    let twi0 = &peripherals.twi0;

    pmc.enable_clock(PeripheralClock::Twi0);

    twi0.cr.write_with_zero(|w| w.swrst().set_bit());

    twi0
      .cr
      .write_with_zero(|w| w.svdis().set_bit().msen().set_bit());

    let mut ck_div: u32 = 0;
    let mut cl_div: u32;

    loop {
      cl_div = ((pmc.clocks.master_clk().0 / (2 * FAST_MODE_HZ)) - 8) / (1 << ck_div);

      if cl_div <= 255 {
        break;
      } else {
        ck_div += 1;
      }
    }

    twi0
      .cwgr
      .write_with_zero(|w| unsafe { w.ckdiv().bits(ck_div as u8).cldiv().bits(cl_div as u8) });

    unsafe { S_I2C = Some(i2c) }
  }

  pub fn get() -> &'static mut Self {
    unsafe { S_I2C.as_mut().unwrap() }
  }

  fn wait_byte_sent(&mut self) -> Result<(), <I2C as Write>::Error> {
    let mut timeout = TRANSMIT_TIMEOUT;
    let twi0 = &Peripherals::get().twi0;

    loop {
      if twi0.sr.read().txrdy().bit_is_set() {
        break;
      }

      if twi0.sr.read().nack().bit_is_set() {
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

impl Write for I2C {
  type Error = ();

  fn try_write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
    let twi0 = &Peripherals::get().twi0;

    twi0
      .mmr
      .write_with_zero(|w| unsafe { w.dadr().bits(address as u8) });

    for byte in bytes {
      twi0
        .thr
        .write_with_zero(|w| unsafe { w.bits(*byte as u32) });

      if let Err(e) = self.wait_byte_sent() {
        return Err(e);
      }
    }

    twi0.cr.write_with_zero(|w| w.stop().set_bit());

    while twi0.sr.read().txcomp().bit_is_clear() {}

    Ok(())
  }
}
