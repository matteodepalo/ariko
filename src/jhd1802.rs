use crate::i2c::I2C;
use crate::peripherals::Peripherals;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::blocking::i2c::Write;

const DEVICE_ADDRESS: u8 = 0x3E;

const CMD_CLEAR_DISPLAY: u8 = 0x01;
const CMD_ENTRY_MODE_SET: u8 = 0x04;
const CMD_DISPLAY_CONTROL: u8 = 0x08;
const CMD_FUNCTION_SET: u8 = 0x20;

const ENTRY_MODE_INCREMENT: u8 = 0b00000010;

const DISPLAY_CONTROL_DISPLAY_ON: u8 = 0b00000100;
const _DISPLAY_CONTROL_CURSOR_ON: u8 = 0b00000010;
const _DISPLAY_CONTROL_CURSOR_BLINKING_ON: u8 = 0b00000001;

const FUNCTION_SET_2_LINES: u8 = 0b00001000;

const CONTROL_BYTE_RS: u8 = 0b01000000;

const INIT_FUNCTION_SET: u8 = FUNCTION_SET_2_LINES;
const INIT_DISPLAY_CONTROL: u8 = 0;

static mut S_JHD1802: Option<JHD1802> = None;

pub struct JHD1802;

impl JHD1802 {
  pub fn init() {
    let delay = unsafe { &mut Peripherals::get().delay };
    let jhd1802 = JHD1802;

    delay.try_delay_ms(50_u8).unwrap();
    jhd1802.send_command(CMD_FUNCTION_SET | INIT_FUNCTION_SET);

    delay.try_delay_ms(5_u8).unwrap();
    jhd1802.send_command(CMD_FUNCTION_SET | INIT_FUNCTION_SET);

    delay.try_delay_us(500_u32).unwrap();
    jhd1802.send_command(CMD_FUNCTION_SET | INIT_FUNCTION_SET);
    jhd1802.send_command(CMD_DISPLAY_CONTROL | INIT_DISPLAY_CONTROL);
    jhd1802.clear();

    delay.try_delay_us(1700_u32).unwrap();
    jhd1802.send_command(CMD_ENTRY_MODE_SET | ENTRY_MODE_INCREMENT);
    jhd1802.send_command(CMD_DISPLAY_CONTROL | INIT_DISPLAY_CONTROL | DISPLAY_CONTROL_DISPLAY_ON);

    unsafe { S_JHD1802 = Some(jhd1802) }
  }

  pub fn get() -> &'static mut Self {
    unsafe { S_JHD1802.as_mut().unwrap() }
  }

  pub fn set_cursor(&self, col: u8, row: u8) {
    let col = if row == 0 { col | 0x80 } else { col | 0xc0 };
    self.send_command(col);
  }

  pub fn clear(&self) {
    self.send_command(CMD_CLEAR_DISPLAY);
    unsafe { Peripherals::get().delay.try_delay_us(2000_u32).unwrap() }
  }

  pub fn send_str(&self, value: &str) {
    for (i, char) in value.as_bytes().iter().enumerate() {
      if i == 16 {
        self.set_cursor(0, 1);
      }

      self.send_char(*char);
    }
  }

  fn send_byte(&self, value: u8, is_cmd: bool) {
    let i2c = I2C::get();
    let control_byte = if is_cmd { 0x00 } else { CONTROL_BYTE_RS };

    i2c
      .try_write(DEVICE_ADDRESS, &[control_byte, value])
      .unwrap();
  }

  fn send_command(&self, value: u8) {
    self.send_byte(value, true);
  }
  fn send_char(&self, value: u8) {
    self.send_byte(value, false);
  }
}
