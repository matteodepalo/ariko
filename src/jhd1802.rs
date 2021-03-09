use crate::i2c::I2c;
use core::fmt::{Error, Write};
use cortex_m::peripheral::SYST;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::blocking::i2c::Write as I2cWrite;
use sam3x8e_hal::delay::Delay;

// commands
const LCD_CLEARDISPLAY: u8 = 0x01;
const LCD_RETURNHOME: u8 = 0x02;
const LCD_ENTRYMODESET: u8 = 0x04;
const LCD_DISPLAYCONTROL: u8 = 0x08;
const LCD_CURSORSHIFT: u8 = 0x10;
const LCD_FUNCTIONSET: u8 = 0x20;
const LCD_SETCGRAMADDR: u8 = 0x40;
const LCD_SETDDRAMADDR: u8 = 0x80;

// flags for display entry mode
const LCD_ENTRYRIGHT: u8 = 0x00;
const LCD_ENTRYLEFT: u8 = 0x02;
const LCD_ENTRYSHIFTINCREMENT: u8 = 0x01;
const LCD_ENTRYSHIFTDECREMENT: u8 = 0x00;

// flags for display on/off control
const LCD_DISPLAYON: u8 = 0x04;
const LCD_DISPLAYOFF: u8 = 0x00;
const LCD_CURSORON: u8 = 0x02;
const LCD_CURSOROFF: u8 = 0x00;
const LCD_BLINKON: u8 = 0x01;
const LCD_BLINKOFF: u8 = 0x00;

// flags for display/cursor shift
const LCD_DISPLAYMOVE: u8 = 0x08;
const LCD_CURSORMOVE: u8 = 0x00;
const LCD_MOVERIGHT: u8 = 0x04;
const LCD_MOVELEFT: u8 = 0x00;

// flags for function set
const LCD_8BITMODE: u8 = 0x10;
const LCD_4BITMODE: u8 = 0x00;
const LCD_2LINE: u8 = 0x08;
const LCD_1LINE: u8 = 0x00;
const LCD_5x10DOTS: u8 = 0x04;
const LCD_5x8DOTS: u8 = 0x00;

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

    self.command(LCD_FUNCTIONSET | LCD_2LINE | LCD_5x8DOTS | LCD_4BITMODE);
    self.delay.try_delay_us(50_u32);

    self.command(LCD_DISPLAYCONTROL | LCD_DISPLAYON | LCD_CURSORON | LCD_BLINKON);
    self.delay.try_delay_us(50_u32);

    self.command(LCD_CLEARDISPLAY);
    self.delay.try_delay_ms(2_u32);

    self.command(LCD_ENTRYMODESET | LCD_ENTRYLEFT);
  }

  fn write_nibble(&mut self, char: u8, mode: u8) {
    self
      .i2c
      .try_write(self.address, &[mode | (char & 0xF0)])
      .unwrap();

    self
      .i2c
      .try_write(self.address, &[mode | ((char << 4) & 0xF0)])
      .unwrap();
  }

  fn command(&mut self, value: u8) {
    self.write_nibble(value, 0);
    self.delay.try_delay_us(50_u32).unwrap();
  }
}

impl Write for Jhd1802<'_> {
  fn write_str(&mut self, string: &str) -> Result<(), Error> {
    for char in string.as_bytes() {
      self.write_nibble(*char, 1);
      self.delay.try_delay_us(50_u32).unwrap();
    }

    Ok(())
  }
}
