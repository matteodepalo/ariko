use crate::jhd1802::JHD1802;
use core::cell::RefCell;
use core::fmt;
use core::fmt::Write;
use critical_section::Mutex;

static DISPLAY: Mutex<RefCell<Option<Display>>> = Mutex::new(RefCell::new(None));

pub struct Display;

impl Display {
  pub fn init() {
    JHD1802::init();
    critical_section::with(|cs| {
      DISPLAY.borrow(cs).replace(Some(Display));
    });
  }

  pub fn with<F, R>(f: F) -> R
  where
    F: FnOnce(&mut Display) -> R,
  {
    critical_section::with(|cs| {
      let mut borrow = DISPLAY.borrow(cs).borrow_mut();
      let display = borrow.as_mut().expect("Display not initialized");
      f(display)
    })
  }
}

impl Write for Display {
  fn write_str(&mut self, string: &str) -> fmt::Result {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);
      jhd1802.send_str(string);
    });
    Ok(())
  }
}

impl Display {
  /// Clear the display
  pub fn clear(&self) {
    JHD1802::with(|jhd1802| jhd1802.clear());
  }

  /// Display game status with turn and timer
  /// Line 1: "White 09:45" or "Black 08:30"
  /// Line 2: Status message or last move
  pub fn show_game_status(&self, is_white_turn: bool, minutes: u8, seconds: u8, line2: &str) {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);

      // Line 1: Turn and timer
      if is_white_turn {
        jhd1802.send_str("White ");
      } else {
        jhd1802.send_str("Black ");
      }

      // Format time as MM:SS
      let m1 = b'0' + (minutes / 10);
      let m2 = b'0' + (minutes % 10);
      let s1 = b'0' + (seconds / 10);
      let s2 = b'0' + (seconds % 10);

      jhd1802.send_str(core::str::from_utf8(&[m1, m2, b':', s1, s2]).unwrap_or("??:??"));

      // Line 2: Status or move
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str(line2);
    });
  }

  /// Display calibration prompt
  pub fn show_calibration_prompt(&self) {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);
      jhd1802.send_str("Place pieces in");
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str("starting pos...");
    });
  }

  /// Display calibration progress
  pub fn show_calibration_progress(&self, count: u8) {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);
      jhd1802.send_str("Calibrating...");
      jhd1802.set_cursor(0, 1);

      // Show count/32
      let c1 = b'0' + (count / 10);
      let c2 = b'0' + (count % 10);
      jhd1802.send_str(core::str::from_utf8(&[c1, c2]).unwrap_or("??"));
      jhd1802.send_str("/32 pieces");
    });
  }

  /// Display calibration complete
  pub fn show_calibration_complete(&self) {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);
      jhd1802.send_str("Calibration OK!");
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str("Ready to play");
    });
  }

  /// Display waiting for setup
  pub fn show_waiting_for_setup(&self) {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);
      jhd1802.send_str("Setup board for");
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str("new game...");
    });
  }

  /// Display game paused
  pub fn show_paused(&self) {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);
      jhd1802.send_str("  ** PAUSED **");
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str("Press to resume");
    });
  }

  /// Display game over
  pub fn show_game_over(&self, winner: &str, reason: &str) {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);
      jhd1802.send_str(winner);
      jhd1802.send_str(" wins!");
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str(reason);
    });
  }

  /// Display draw
  pub fn show_draw(&self, reason: &str) {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);
      jhd1802.send_str("   DRAW");
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str(reason);
    });
  }

  /// Display invalid move message
  pub fn show_invalid_move(&self) {
    JHD1802::with(|jhd1802| {
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str("Invalid move!   ");
    });
  }

  /// Display check warning
  pub fn show_check(&self) {
    JHD1802::with(|jhd1802| {
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str("CHECK!          ");
    });
  }
}
