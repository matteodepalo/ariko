use crate::peripherals::Peripherals;
use core::cell::RefCell;
use critical_section::Mutex;
use sam3x8e_hal::pmc::PeripheralClock;

const TRANSMIT_TIMEOUT: u32 = 100_000;
const FAST_MODE_HZ: u32 = 400_000;

static I2C_INSTANCE: Mutex<RefCell<Option<I2C>>> = Mutex::new(RefCell::new(None));

/// I2C Write trait (local definition compatible with embedded-hal 1.0 style)
pub trait I2cWrite {
  type Error;
  fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error>;
}

pub struct I2C;

impl I2C {
  pub fn init() {
    Peripherals::with(|peripherals| {
      let pmc = &mut peripherals.pmc;
      let twi0 = &peripherals.twi0;

      pmc.enable_clock(PeripheralClock::Twi0);

      unsafe { twi0.cr().write_with_zero(|w| w.swrst().set_bit()); }

      unsafe {
        twi0
          .cr()
          .write_with_zero(|w| w.svdis().set_bit().msen().set_bit());
      }

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

      unsafe {
        twi0
          .cwgr()
          .write_with_zero(|w| w.ckdiv().bits(ck_div as u8).cldiv().bits(cl_div as u8));
      }
    });

    critical_section::with(|cs| {
      I2C_INSTANCE.borrow(cs).replace(Some(I2C));
    });
  }

  pub fn with<F, R>(f: F) -> R
  where
    F: FnOnce(&mut I2C) -> R,
  {
    critical_section::with(|cs| {
      let mut borrow = I2C_INSTANCE.borrow(cs).borrow_mut();
      let i2c = borrow.as_mut().expect("I2C not initialized");
      f(i2c)
    })
  }

  fn wait_byte_sent(&mut self) -> Result<(), <I2C as I2cWrite>::Error> {
    let mut timeout = TRANSMIT_TIMEOUT;

    Peripherals::with(|peripherals| {
      let twi0 = &peripherals.twi0;

      loop {
        if twi0.sr().read().txrdy().bit_is_set() {
          return Ok(());
        }

        if twi0.sr().read().nack().bit_is_set() {
          return Err(());
        }

        timeout -= 1;

        if timeout == 0 {
          return Err(());
        }
      }
    })
  }
}

impl I2cWrite for I2C {
  type Error = ();

  fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
    Peripherals::with(|peripherals| {
      let twi0 = &peripherals.twi0;

      unsafe {
        twi0
          .mmr()
          .write_with_zero(|w| w.dadr().bits(address as u8));
      }

      for byte in bytes {
        unsafe {
          twi0
            .thr()
            .write_with_zero(|w| w.bits(*byte as u32));
        }

        // Wait for byte to be sent
        let mut timeout = TRANSMIT_TIMEOUT;
        loop {
          if twi0.sr().read().txrdy().bit_is_set() {
            break;
          }
          if twi0.sr().read().nack().bit_is_set() {
            return Err(());
          }
          timeout -= 1;
          if timeout == 0 {
            return Err(());
          }
        }
      }

      unsafe { twi0.cr().write_with_zero(|w| w.stop().set_bit()); }

      while twi0.sr().read().txcomp().bit_is_clear() {}

      Ok(())
    })
  }
}
