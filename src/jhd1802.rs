use crate::i2c::I2c;
use core::fmt::{Error, Write};
use cortex_m::peripheral::SYST;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::blocking::i2c::Write as I2cWrite;
use sam3x8e_hal::delay::Delay;

const LCD_2LINE: u8 = 0x08;
const LCD_DISPLAYON: u8 = 0x04;
const LCD_CURSOROFF: u8 = 0x00;
const LCD_BLINKOFF: u8 = 0x00;
const LCD_ENTRYLEFT: u8 = 0x02;
const LCD_ENTRYSHIFTDECREMENT: u8 = 0x00;
const LCD_FUNCTIONSET: u8 = 0x20;
const LCD_ENTRYMODESET: u8 = 0x04;
const LCD_DISPLAYCONTROL: u8 = 0x08;
const LCD_CLEARDISPLAY: u8 = 0x01;
const LCD_8BITMODE: u8 = 0x10;

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
    self.delay.try_delay_ms(50_u32);
    self.command(LCD_FUNCTIONSET | LCD_8BITMODE);
    self.delay.try_delay_us(4500_u32).unwrap();
    self.command(LCD_FUNCTIONSET | LCD_8BITMODE);
    self.delay.try_delay_us(150_u32).unwrap();
    self.command(LCD_FUNCTIONSET | LCD_8BITMODE);
    self.command(LCD_FUNCTIONSET | LCD_8BITMODE | LCD_2LINE);
    self.delay.try_delay_us(100_u32).unwrap();
    self.command(LCD_DISPLAYCONTROL | LCD_DISPLAYON | 0x01 | 0x02);
    self.delay.try_delay_us(100_u32).unwrap();
    self.clear();
    self.delay.try_delay_us(2000_u32).unwrap();
    self.command(LCD_ENTRYMODESET | LCD_ENTRYLEFT | LCD_ENTRYSHIFTDECREMENT);
  }

  fn clear(&mut self) {
    self.command(LCD_CLEARDISPLAY);
    self.delay.try_delay_us(2000_u32).unwrap();
  }

  fn command(&mut self, value: u8) {
    self.i2c.try_write(self.address, &[0x80, value]).unwrap()
  }
}

impl Write for Jhd1802<'_> {
  fn write_str(&mut self, string: &str) -> Result<(), Error> {
    for char in string.as_bytes() {
      self.i2c.try_write(self.address, &[0x40, *char]).unwrap();
    }

    Ok(())
  }
}
