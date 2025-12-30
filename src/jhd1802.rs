#![allow(dead_code)]

use crate::i2c::{I2cWrite, I2C};
use crate::peripherals::Peripherals;
use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal::delay::DelayNs;

const DEVICE_ADDRESS: u8 = 0x3E;
const CMD_CLEAR_DISPLAY: u8 = 0x01;
const CMD_ENTRY_MODE_SET: u8 = 0x04;
const CMD_DISPLAY_CONTROL: u8 = 0x08;
const CMD_FUNCTION_SET: u8 = 0x20;
const ENTRY_MODE_INCREMENT: u8 = 0b00000010;
const DISPLAY_CONTROL_DISPLAY_ON: u8 = 0b00000100;
const DISPLAY_CONTROL_CURSOR_ON: u8 = 0b00000010;
const DISPLAY_CONTROL_CURSOR_BLINKING_ON: u8 = 0b00000001;
const FUNCTION_SET_2_LINES: u8 = 0b00001000;
const CONTROL_BYTE_RS: u8 = 0b01000000;
const INIT_FUNCTION_SET: u8 = FUNCTION_SET_2_LINES;
const INIT_DISPLAY_CONTROL: u8 = 0;

static JHD1802_INSTANCE: Mutex<RefCell<Option<JHD1802>>> = Mutex::new(RefCell::new(None));

pub struct JHD1802;

impl JHD1802 {
  pub fn init() {
    let jhd1802 = JHD1802;

    Peripherals::with(|p| p.delay.delay_ms(50));
    jhd1802.send_command(CMD_FUNCTION_SET | INIT_FUNCTION_SET);

    Peripherals::with(|p| p.delay.delay_ms(5));
    jhd1802.send_command(CMD_FUNCTION_SET | INIT_FUNCTION_SET);

    Peripherals::with(|p| p.delay.delay_us(500));
    jhd1802.send_command(CMD_FUNCTION_SET | INIT_FUNCTION_SET);
    jhd1802.send_command(CMD_DISPLAY_CONTROL | INIT_DISPLAY_CONTROL);
    jhd1802.clear();

    Peripherals::with(|p| p.delay.delay_us(1700));
    jhd1802.send_command(CMD_ENTRY_MODE_SET | ENTRY_MODE_INCREMENT);
    jhd1802.send_command(CMD_DISPLAY_CONTROL | INIT_DISPLAY_CONTROL | DISPLAY_CONTROL_DISPLAY_ON);

    critical_section::with(|cs| {
      JHD1802_INSTANCE.borrow(cs).replace(Some(jhd1802));
    });
  }

  pub fn with<F, R>(f: F) -> R
  where
    F: FnOnce(&mut JHD1802) -> R,
  {
    critical_section::with(|cs| {
      let mut borrow = JHD1802_INSTANCE.borrow(cs).borrow_mut();
      let jhd1802 = borrow.as_mut().expect("JHD1802 not initialized");
      f(jhd1802)
    })
  }

  pub fn set_cursor(&self, col: u8, row: u8) {
    let col = if row == 0 { col | 0x80 } else { col | 0xc0 };
    self.send_command(col);
  }

  pub fn clear(&self) {
    self.send_command(CMD_CLEAR_DISPLAY);
    Peripherals::with(|p| p.delay.delay_us(2000));
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
    let control_byte = if is_cmd { 0x00 } else { CONTROL_BYTE_RS };

    I2C::with(|i2c| {
      i2c
        .write(DEVICE_ADDRESS, &[control_byte, value])
        .unwrap();
    });
  }

  fn send_command(&self, value: u8) {
    self.send_byte(value, true);
  }
  fn send_char(&self, value: u8) {
    self.send_byte(value, false);
  }
}
