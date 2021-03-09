use crate::i2c::I2c;
use core::fmt::{Error, Write};
use cortex_m::peripheral::SYST;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::blocking::i2c::Write as I2cWrite;
use sam3x8e_hal::delay::Delay;

const CMD_CLEAR_DISPLAY: u8 = 0x01;
const CMD_ENTRY_MODE_SET: u8 = 0x04;
const CMD_DISPLAY_CONTROL: u8 = 0x08;
const CMD_FUNCTION_SET: u8 = 0x20;

const BIT_ENTRY_MODE_INCREMENT: u8 = 1;
const BIT_DISPLAY_CONTROL_DISPLAY: u8 = 2;
const BIT_DISPLAY_CONTROL_CURSOR: u8 = 1;
const BIT_DISPLAY_CONTROL_CURSOR_BLINKING: u8 = 0;
const BIT_FUNCTION_SET_BITMODE: u8 = 4;
const BIT_FUNCTION_SET_LINECOUNT: u8 = 3;
const BIT_CONTROL_BYTE_RS: u8 = 6;

const INIT_FUNCTION_SET: u8 = (1 << BIT_FUNCTION_SET_BITMODE) | (1 << BIT_FUNCTION_SET_LINECOUNT);

const INIT_DISPLAY_CONTROL: u8 =
  (1 << BIT_DISPLAY_CONTROL_CURSOR) | (1 << BIT_DISPLAY_CONTROL_CURSOR_BLINKING);

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
    self.delay.try_delay_ms(50_u8).unwrap();
    self.send_command(CMD_FUNCTION_SET | INIT_FUNCTION_SET);
    self.delay.try_delay_ms(5_u8).unwrap();
    self.send_command(CMD_FUNCTION_SET | INIT_FUNCTION_SET);
    self.delay.try_delay_us(500_u32).unwrap();
    self.send_command(CMD_FUNCTION_SET | INIT_FUNCTION_SET);
    self.send_command(CMD_DISPLAY_CONTROL | INIT_DISPLAY_CONTROL);
    self.send_command(CMD_CLEAR_DISPLAY);
    self.delay.try_delay_ms(1700_u32).unwrap();
    self.send_command(CMD_ENTRY_MODE_SET | (1 << BIT_ENTRY_MODE_INCREMENT));

    self.send_command(
      CMD_DISPLAY_CONTROL | INIT_DISPLAY_CONTROL | (1 << BIT_DISPLAY_CONTROL_DISPLAY),
    );
  }

  fn send_command(&mut self, value: u8) {
    self.send_byte(value, true);
  }

  fn send_char(&mut self, value: u8) {
    self.send_byte(value, false);
  }

  fn send_byte(&mut self, value: u8, is_cmd: bool) {
    let control_byte = if is_cmd { 0 } else { 1 << BIT_CONTROL_BYTE_RS };

    self
      .i2c
      .try_write(self.address, &[control_byte, value])
      .unwrap();
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
