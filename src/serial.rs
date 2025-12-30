use crate::peripherals::Peripherals;
use core::cell::RefCell;
use core::fmt;
use core::fmt::Write;
use critical_section::Mutex;

static SERIAL: Mutex<RefCell<Option<Serial>>> = Mutex::new(RefCell::new(None));

pub struct Serial;

impl Serial {
  pub fn init(baud: u32) {
    Peripherals::with(|peripherals| {
      let uart = &peripherals.uart;
      let pmc = &mut peripherals.pmc;

      unsafe {
        uart
          .ptcr()
          .write_with_zero(|w| w.rxtdis().set_bit().txtdis().set_bit());
      }

      unsafe {
        uart.cr().write_with_zero(|w| {
          w.rstrx()
            .set_bit()
            .rsttx()
            .set_bit()
            .rxdis()
            .set_bit()
            .txdis()
            .set_bit()
        });
      }

      unsafe {
        uart.brgr().write_with_zero(|w| {
          w.cd()
            .bits(((pmc.clocks.master_clk().0 / baud) / 16) as u16)
        });
      }

      unsafe {
        uart
          .cr()
          .write_with_zero(|w| w.rxen().set_bit().txen().set_bit().rststa().set_bit());
      }
    });

    critical_section::with(|cs| {
      SERIAL.borrow(cs).replace(Some(Serial));
    });
  }

  pub fn with<F, R>(f: F) -> R
  where
    F: FnOnce(&mut Serial) -> R,
  {
    critical_section::with(|cs| {
      let mut borrow = SERIAL.borrow(cs).borrow_mut();
      let serial = borrow.as_mut().expect("Serial not initialized");
      f(serial)
    })
  }

  pub fn send_str(&mut self, string: &str) {
    Peripherals::with(|peripherals| {
      let uart = &peripherals.uart;

      for char in string.as_bytes() {
        while uart.sr().read().txrdy().bit_is_clear() {}

        unsafe {
          uart
            .thr()
            .write_with_zero(|w| w.txchr().bits(*char));
        }
      }
    });
  }
}

impl Write for Serial {
  fn write_str(&mut self, value: &str) -> fmt::Result {
    self.send_str(value);
    Ok(())
  }
}
