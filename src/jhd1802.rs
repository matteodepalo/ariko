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

// Helper function for cursor position calculation (testable without hardware)
pub fn calculate_cursor_address(col: u8, row: u8) -> u8 {
  if row == 0 {
    col | 0x80
  } else {
    col | 0xc0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_command_constants() {
    // Verify LCD command values match HD44780 spec
    assert_eq!(CMD_CLEAR_DISPLAY, 0x01);
    assert_eq!(CMD_ENTRY_MODE_SET, 0x04);
    assert_eq!(CMD_DISPLAY_CONTROL, 0x08);
    assert_eq!(CMD_FUNCTION_SET, 0x20);
  }

  #[test]
  fn test_entry_mode_bits() {
    assert_eq!(ENTRY_MODE_INCREMENT, 0b00000010);
  }

  #[test]
  fn test_display_control_bits() {
    assert_eq!(DISPLAY_CONTROL_DISPLAY_ON, 0b00000100);
    assert_eq!(DISPLAY_CONTROL_CURSOR_ON, 0b00000010);
    assert_eq!(DISPLAY_CONTROL_CURSOR_BLINKING_ON, 0b00000001);
  }

  #[test]
  fn test_function_set_bits() {
    assert_eq!(FUNCTION_SET_2_LINES, 0b00001000);
  }

  #[test]
  fn test_control_byte() {
    // RS bit is bit 6
    assert_eq!(CONTROL_BYTE_RS, 0b01000000);
  }

  #[test]
  fn test_cursor_position_row_0() {
    // Row 0 starts at DDRAM address 0x00, with bit 7 set
    assert_eq!(calculate_cursor_address(0, 0), 0x80);
    assert_eq!(calculate_cursor_address(1, 0), 0x81);
    assert_eq!(calculate_cursor_address(15, 0), 0x8F);
  }

  #[test]
  fn test_cursor_position_row_1() {
    // Row 1 starts at DDRAM address 0x40, with bit 7 set = 0xC0
    assert_eq!(calculate_cursor_address(0, 1), 0xC0);
    assert_eq!(calculate_cursor_address(1, 1), 0xC1);
    assert_eq!(calculate_cursor_address(15, 1), 0xCF);
  }

  #[test]
  fn test_init_commands() {
    // Test that initialization commands are correct
    let function_set_cmd = CMD_FUNCTION_SET | INIT_FUNCTION_SET;
    assert_eq!(function_set_cmd, 0x28); // 0x20 | 0x08 = 2-line mode

    let entry_mode_cmd = CMD_ENTRY_MODE_SET | ENTRY_MODE_INCREMENT;
    assert_eq!(entry_mode_cmd, 0x06); // 0x04 | 0x02 = increment, no shift

    let display_on_cmd = CMD_DISPLAY_CONTROL | INIT_DISPLAY_CONTROL | DISPLAY_CONTROL_DISPLAY_ON;
    assert_eq!(display_on_cmd, 0x0C); // 0x08 | 0x00 | 0x04 = display on, cursor off
  }

  #[test]
  fn test_device_address() {
    // JHD1802 I2C address
    assert_eq!(DEVICE_ADDRESS, 0x3E);
  }
}
