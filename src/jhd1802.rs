use crate::i2c::I2c;
use core::fmt::{Error, Write};
use cortex_m::peripheral::SYST;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::blocking::i2c::Write as I2cWrite;
use sam3x8e_hal::delay::Delay;

pub struct Jhd1802<'a> {
  i2c: I2c<'a>,
  address: u8,
  delay: &'a mut Delay<SYST>,
}

impl<'a> Jhd1802<'a> {
  pub fn new(i2c: I2c<'a>, address: u8, delay: &'a mut Delay<SYST>) -> Self {
    let mut jhd = Jhd1802 {
      i2c,
      address,
      delay,
    };

    jhd.init();
    jhd
  }

  fn init(&mut self) {
    self.delay.try_delay_ms(50_u32).unwrap();

    self.command(0b00101000);
    self.delay.try_delay_us(50_u32).unwrap();

    self.command(0b00001111);
    self.delay.try_delay_us(50_u32).unwrap();

    self.command(0b00000001);
    self.delay.try_delay_ms(2_u32).unwrap();

    self.command(0b00000110);
  }

  fn command(&mut self, value: u8) {
    self.i2c.try_write(self.address, &[0x80, value]).unwrap();
    self.delay.try_delay_us(50_u32).unwrap();
  }
}

impl Write for Jhd1802<'_> {
  fn write_str(&mut self, string: &str) -> Result<(), Error> {
    for char in string.as_bytes() {
      self.i2c.try_write(self.address, &[0x40, *char]).unwrap();
      self.delay.try_delay_us(50_u32).unwrap();
    }

    Ok(())
  }
}
