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
  /// Display game status with turn and last move
  /// Line 1: "White to move" or "Black to move"
  /// Line 2: Last move (e.g., "Last: e2-e4")
  pub fn show_turn(&self, is_white_turn: bool) {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);
      if is_white_turn {
        jhd1802.send_str("White to move");
      } else {
        jhd1802.send_str("Black to move");
      }
    });
  }

  /// Display last move on line 2
  /// Format: "Last: e2-e4" or "Last: Nf3xg5"
  pub fn show_last_move(&self, from_sq: u8, to_sq: u8) {
    JHD1802::with(|jhd1802| {
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str("Last: ");
      jhd1802.send_str(Self::square_name(from_sq));
      jhd1802.send_str("-");
      jhd1802.send_str(Self::square_name(to_sq));
      jhd1802.send_str("        "); // Clear rest of line
    });
  }

  /// Convert square index (0-63) to algebraic notation
  fn square_name(sq: u8) -> &'static str {
    const SQUARES: [&str; 64] = [
      "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
      "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
      "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
      "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
      "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
      "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
      "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
      "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    ];
    if sq < 64 {
      SQUARES[sq as usize]
    } else {
      "??"
    }
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

  /// Display pawn promotion prompt
  pub fn show_promotion_prompt(&self) {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);
      jhd1802.send_str("PROMOTION!");
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str("Place new piece");
    });
  }

  pub fn show_takeback_complete(&self) {
    JHD1802::with(|jhd1802| {
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str("Move taken back ");
    });
  }

  pub fn show_takeback_capture(&self) {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);
      jhd1802.send_str("TAKEBACK");
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str("Restore capture");
    });
  }

  pub fn show_takeback_castling(&self) {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);
      jhd1802.send_str("TAKEBACK");
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str("Move rook back");
    });
  }

  pub fn show_takeback_en_passant(&self) {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);
      jhd1802.send_str("TAKEBACK");
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str("Restore pawn");
    });
  }

  pub fn show_takeback_promotion(&self) {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);
      jhd1802.send_str("TAKEBACK");
      jhd1802.set_cursor(0, 1);
      jhd1802.send_str("Place pawn back");
    });
  }
}
