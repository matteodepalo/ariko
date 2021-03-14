use crate::peripherals::Peripherals;
use core::fmt;
use core::fmt::Write;
use sam3x8e_hal::pmc::PeripheralClock;

static mut S_SERIAL: Option<Serial> = None;

pub struct Serial;

impl Serial {
  pub fn init(baud: u32) {
    let peripherals = Peripherals::get();
    let uart = &peripherals.uart;
    let pmc = &mut peripherals.pmc;

    uart
      .ptcr
      .write_with_zero(|w| w.rxtdis().set_bit().txtdis().set_bit());

    uart.cr.write_with_zero(|w| {
      w.rstrx()
        .set_bit()
        .rsttx()
        .set_bit()
        .rxdis()
        .set_bit()
        .txdis()
        .set_bit()
    });

    uart.brgr.write_with_zero(|w| unsafe {
      w.cd()
        .bits(((pmc.clocks.master_clk().0 / baud) / 16) as u16)
    });

    uart
      .cr
      .write_with_zero(|w| w.rxen().set_bit().txen().set_bit().rststa().set_bit());

    unsafe { S_SERIAL = Some(Serial) }
  }

  pub fn get() -> &'static mut Self {
    unsafe { S_SERIAL.as_mut().unwrap() }
  }

  pub fn send_str(&mut self, string: &str) {
    let uart = &Peripherals::get().uart;

    for char in string.as_bytes() {
      while uart.sr.read().txrdy().bit_is_clear() {}

      uart
        .thr
        .write_with_zero(|w| unsafe { w.txchr().bits(*char) });
    }
  }
}

impl Write for Serial {
  fn write_str(&mut self, value: &str) -> fmt::Result {
    self.send_str(value);
    Ok(())
  }
}
