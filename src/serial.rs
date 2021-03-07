use core::fmt;
use sam3x8e_hal::pmc::Pmc;
use sam3x8e_hal::time::Hertz;

pub struct Serial {
  uart: sam3x8e_hal::pac::UART,
}

impl Serial {
  pub fn new(baud: Hertz, pmc: &mut Pmc, uart: sam3x8e_hal::pac::UART) -> Self {
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
        .bits(((pmc.clocks.master_clk().0 / baud.0) / 16) as u16)
    });

    uart
      .cr
      .write_with_zero(|w| w.rxen().set_bit().txen().set_bit().rststa().set_bit());

    Serial { uart }
  }

  pub fn write(&mut self, string: &str) {
    for char in string.as_bytes() {
      while self.uart.sr.read().txrdy().bit_is_clear() {}

      self
        .uart
        .thr
        .write_with_zero(|w| unsafe { w.txchr().bits(*char) });
    }
  }
}

impl fmt::Write for Serial {
  fn write_str(&mut self, string: &str) -> fmt::Result {
    self.write(string);
    Ok(())
  }
}
