use crate::i2c::I2c;
use core::fmt::{Error, Write};
use cortex_m::peripheral::SYST;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::blocking::i2c::Write as I2cWrite;
use sam3x8e_hal::delay::Delay;

const BACKLIGHT_BIT: u8 = 0x8;
const ENABLE_BIT: u8 = 0x4;
const REGISTER_SELECT_BIT: u8 = 0x1;
const LCD_FUNCTIONSET: u8 = 0x20;
const LCD_ENTRYLEFT: u8 = 0x2;
const LCD_2LINE: u8 = 0x8;
const LCD_CURSORON: u8 = 0x2;
const LCD_DISPLAYON: u8 = 0x4;
const LCD_BLINKON: u8 = 0x1;
const LCD_DISPLAYCONTROL: u8 = 0x8;
const LCD_CLEARDISPLAY: u8 = 0x1;
const LCD_ENTRYMODESET: u8 = 0x4;

pub struct Jhd1802<'a> {
  i2c: I2c,
  address: u8,
  delay: &'a mut Delay<SYST>,
}

impl<'a> Jhd1802<'a> {
  pub fn new(i2c: I2c, address: u8, delay: &'a mut Delay<SYST>) -> Self {
    let mut jhd = Jhd1802 {
      i2c,
      address,
      delay,
    };

    jhd.init();
    jhd
  }

  fn init(&mut self) {
    self.delay.try_delay_us(50000_u32).unwrap();
    self.send_byte(LCD_FUNCTIONSET | LCD_2LINE, false, 4500);
    self.send_byte(LCD_FUNCTIONSET | LCD_2LINE, false, 150);
    self.send_byte(LCD_FUNCTIONSET | LCD_2LINE, false, 0);
    self.send_byte(LCD_FUNCTIONSET | LCD_2LINE, false, 0);
    self.send_command(LCD_DISPLAYCONTROL | LCD_DISPLAYON | LCD_CURSORON | LCD_BLINKON);
    self.send_byte(LCD_CLEARDISPLAY, false, 2000);
    self.send_command(LCD_ENTRYMODESET | LCD_ENTRYLEFT);
  }

  fn send_command(&mut self, value: u8) {
    self.send_byte(value, false, 100);
  }

  fn send_char(&mut self, value: u8) {
    self.send_byte(value, true, 100);
  }

  fn send_nibble(&mut self, value: u8, char: bool) {
    let rs = match char {
      false => 0_u8,
      true => REGISTER_SELECT_BIT,
    };

    let byte = value | rs | BACKLIGHT_BIT;

    self
      .i2c
      .try_write(self.address, &[byte, byte | ENABLE_BIT])
      .unwrap();

    self.delay.try_delay_ms(2_u8).unwrap();
    self.i2c.try_write(self.address, &[byte]).unwrap();
  }

  fn send_byte(&mut self, value: u8, char: bool, delay_us: u32) {
    let upper_nibble = value & 0xF0;
    let lower_nibble = (value & 0x0F) << 4;

    self.send_nibble(upper_nibble, char);
    self.send_nibble(lower_nibble, char);

    if delay_us != 0 {
      self.delay.try_delay_us(delay_us).unwrap();
    }
  }
}

impl Write for Jhd1802<'_> {
  fn write_str(&mut self, string: &str) -> Result<(), Error> {
    for char in string.as_bytes() {
      self.send_char(*char);
    }

    Ok(())
  }
}
