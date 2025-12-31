//! TM1637 4-digit 7-segment display driver for chess clocks
//!
//! Drives two Grove 4-digit displays:
//! - White's clock: D7 connector (PC23=CLK, PC22=DIO)
//! - Black's clock: D5 connector (PC25=CLK, PC24=DIO)

use crate::peripherals::Peripherals;
use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal::digital::OutputPin;

/// 7-segment digit patterns (0-9)
const DIGITS: [u8; 10] = [
  0x3F, // 0
  0x06, // 1
  0x5B, // 2
  0x4F, // 3
  0x66, // 4
  0x6D, // 5
  0x7D, // 6
  0x07, // 7
  0x7F, // 8
  0x6F, // 9
];

/// Colon segment (bit 7 on digit 1)
const COLON: u8 = 0x80;

/// TM1637 commands
const CMD_DATA: u8 = 0x40; // Data command: write data, auto-increment
const CMD_ADDR: u8 = 0xC0; // Address command: start at digit 0
const CMD_CTRL: u8 = 0x88; // Control command: display on, brightness 0

static CHESS_CLOCKS: Mutex<RefCell<Option<ChessClockDisplays>>> = Mutex::new(RefCell::new(None));

/// Chess clock display manager
pub struct ChessClockDisplays {
  /// Whether the colon is currently visible (for blinking)
  colon_visible: bool,
}

impl ChessClockDisplays {
  /// Initialize the chess clock displays
  pub fn init() {
    let displays = ChessClockDisplays { colon_visible: true };

    // Initialize both displays with "00:00"
    displays.update_white(0, 0, false);
    displays.update_black(0, 0, false);

    critical_section::with(|cs| {
      CHESS_CLOCKS.borrow(cs).replace(Some(displays));
    });
  }

  /// Access the displays within a critical section
  pub fn with<F, R>(f: F) -> R
  where
    F: FnOnce(&mut ChessClockDisplays) -> R,
  {
    critical_section::with(|cs| {
      let mut borrow = CHESS_CLOCKS.borrow(cs).borrow_mut();
      let displays = borrow.as_mut().expect("ChessClockDisplays not initialized");
      f(displays)
    })
  }

  /// Toggle colon visibility (call every 500ms for blinking effect)
  pub fn toggle_colon(&mut self) {
    self.colon_visible = !self.colon_visible;
  }

  /// Update White's clock display
  ///
  /// - `minutes`: 0-99
  /// - `seconds`: 0-59
  /// - `active`: if true, show blinking colon
  pub fn update_white(&self, minutes: u8, seconds: u8, active: bool) {
    let show_colon = if active { self.colon_visible } else { true };
    let data = self.format_time(minutes, seconds, show_colon);

    Peripherals::with(|p| {
      self.send_to_display(&mut p.white_clk, &mut p.white_dio, &data);
    });
  }

  /// Update Black's clock display
  ///
  /// - `minutes`: 0-99
  /// - `seconds`: 0-59
  /// - `active`: if true, show blinking colon
  pub fn update_black(&self, minutes: u8, seconds: u8, active: bool) {
    let show_colon = if active { self.colon_visible } else { true };
    let data = self.format_time(minutes, seconds, show_colon);

    Peripherals::with(|p| {
      self.send_to_display(&mut p.black_clk, &mut p.black_dio, &data);
    });
  }

  /// Format time as 4 segment bytes [M1, M0:, S1, S0]
  fn format_time(&self, minutes: u8, seconds: u8, show_colon: bool) -> [u8; 4] {
    let m1 = (minutes / 10) as usize;
    let m0 = (minutes % 10) as usize;
    let s1 = (seconds / 10) as usize;
    let s0 = (seconds % 10) as usize;

    [
      DIGITS[m1],
      DIGITS[m0] | if show_colon { COLON } else { 0 },
      DIGITS[s1],
      DIGITS[s0],
    ]
  }

  /// Send 4 bytes of segment data to a TM1637 display
  fn send_to_display<CLK, DIO>(&self, clk: &mut CLK, dio: &mut DIO, data: &[u8; 4])
  where
    CLK: OutputPin,
    DIO: OutputPin,
  {
    // Set data command (auto-increment)
    self.start(clk, dio);
    self.write_byte(clk, dio, CMD_DATA);
    self.stop(clk, dio);

    // Set address and write 4 digits
    self.start(clk, dio);
    self.write_byte(clk, dio, CMD_ADDR);
    for &byte in data {
      self.write_byte(clk, dio, byte);
    }
    self.stop(clk, dio);

    // Set display control (on, brightness 0)
    self.start(clk, dio);
    self.write_byte(clk, dio, CMD_CTRL);
    self.stop(clk, dio);
  }

  /// Send start condition: DIO goes low while CLK is high
  fn start<CLK, DIO>(&self, clk: &mut CLK, dio: &mut DIO)
  where
    CLK: OutputPin,
    DIO: OutputPin,
  {
    let _ = dio.set_high();
    let _ = clk.set_high();
    self.delay();
    let _ = dio.set_low();
    self.delay();
    let _ = clk.set_low();
    self.delay();
  }

  /// Send stop condition: DIO goes high while CLK is high
  fn stop<CLK, DIO>(&self, clk: &mut CLK, dio: &mut DIO)
  where
    CLK: OutputPin,
    DIO: OutputPin,
  {
    let _ = clk.set_low();
    self.delay();
    let _ = dio.set_low();
    self.delay();
    let _ = clk.set_high();
    self.delay();
    let _ = dio.set_high();
    self.delay();
  }

  /// Write a byte, LSB first
  fn write_byte<CLK, DIO>(&self, clk: &mut CLK, dio: &mut DIO, byte: u8)
  where
    CLK: OutputPin,
    DIO: OutputPin,
  {
    let mut data = byte;

    for _ in 0..8 {
      let _ = clk.set_low();
      self.delay();

      if data & 0x01 != 0 {
        let _ = dio.set_high();
      } else {
        let _ = dio.set_low();
      }
      self.delay();

      let _ = clk.set_high();
      self.delay();

      data >>= 1;
    }

    // ACK bit (we don't read it, just clock it)
    let _ = clk.set_low();
    let _ = dio.set_high(); // Release DIO for ACK
    self.delay();
    let _ = clk.set_high();
    self.delay();
    let _ = clk.set_low();
    self.delay();
  }

  /// Small delay for bit-bang timing (~2µs at 84MHz)
  #[inline(always)]
  fn delay(&self) {
    // At 84MHz, ~168 cycles = 2µs
    for _ in 0..50 {
      cortex_m::asm::nop();
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_digit_patterns() {
    // Verify segment patterns match expected values
    assert_eq!(DIGITS[0], 0x3F); // 0: a,b,c,d,e,f
    assert_eq!(DIGITS[1], 0x06); // 1: b,c
    assert_eq!(DIGITS[8], 0x7F); // 8: all segments
  }

  #[test]
  fn test_colon_bit() {
    assert_eq!(COLON, 0x80); // Bit 7
  }

  #[test]
  fn test_commands() {
    assert_eq!(CMD_DATA, 0x40);
    assert_eq!(CMD_ADDR, 0xC0);
    assert_eq!(CMD_CTRL, 0x88);
  }
}
