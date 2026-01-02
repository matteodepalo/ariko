//! Piece calibration system
//!
//! Maps 40-bit RFID chip IDs to chess piece types.
//! Calibration is stored in RAM only (lost on power cycle).

use crate::certabo::protocol::{RfidReading, RFID_BYTES};

/// Maximum number of distinct pieces we can calibrate
/// (32 standard pieces + 2 extra queens for promotion)
pub const MAX_PIECES: usize = 34;

pub const STARTING_LAYOUT: [(u8, Piece); 32] = [
  (0, Piece::WhiteRook),
  (1, Piece::WhiteKnight),
  (2, Piece::WhiteBishop),
  (3, Piece::WhiteQueen),
  (4, Piece::WhiteKing),
  (5, Piece::WhiteBishop),
  (6, Piece::WhiteKnight),
  (7, Piece::WhiteRook),
  (8, Piece::WhitePawn),
  (9, Piece::WhitePawn),
  (10, Piece::WhitePawn),
  (11, Piece::WhitePawn),
  (12, Piece::WhitePawn),
  (13, Piece::WhitePawn),
  (14, Piece::WhitePawn),
  (15, Piece::WhitePawn),
  (48, Piece::BlackPawn),
  (49, Piece::BlackPawn),
  (50, Piece::BlackPawn),
  (51, Piece::BlackPawn),
  (52, Piece::BlackPawn),
  (53, Piece::BlackPawn),
  (54, Piece::BlackPawn),
  (55, Piece::BlackPawn),
  (56, Piece::BlackRook),
  (57, Piece::BlackKnight),
  (58, Piece::BlackBishop),
  (59, Piece::BlackQueen),
  (60, Piece::BlackKing),
  (61, Piece::BlackBishop),
  (62, Piece::BlackKnight),
  (63, Piece::BlackRook),
];

/// Chess piece types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
  WhitePawn,
  WhiteKnight,
  WhiteBishop,
  WhiteRook,
  WhiteQueen,
  WhiteKing,
  BlackPawn,
  BlackKnight,
  BlackBishop,
  BlackRook,
  BlackQueen,
  BlackKing,
}

impl Piece {
  /// Get the piece color
  pub fn is_white(self) -> bool {
    matches!(
      self,
      Piece::WhitePawn
        | Piece::WhiteKnight
        | Piece::WhiteBishop
        | Piece::WhiteRook
        | Piece::WhiteQueen
        | Piece::WhiteKing
    )
  }

  /// Get the piece character for display (uppercase = white, lowercase = black)
  pub fn to_char(self) -> char {
    match self {
      Piece::WhitePawn => 'P',
      Piece::WhiteKnight => 'N',
      Piece::WhiteBishop => 'B',
      Piece::WhiteRook => 'R',
      Piece::WhiteQueen => 'Q',
      Piece::WhiteKing => 'K',
      Piece::BlackPawn => 'p',
      Piece::BlackKnight => 'n',
      Piece::BlackBishop => 'b',
      Piece::BlackRook => 'r',
      Piece::BlackQueen => 'q',
      Piece::BlackKing => 'k',
    }
  }
}

/// Single piece calibration entry
#[derive(Clone, Copy, Debug)]
pub struct PieceCalibration {
  /// 40-bit RFID chip ID
  pub chip_id: [u8; RFID_BYTES],
  /// Piece type this ID maps to
  pub piece: Piece,
}

/// Calibration data storage
pub struct CalibrationData {
  /// Calibrated pieces
  pieces: [Option<PieceCalibration>; MAX_PIECES],
  /// Number of calibrated pieces
  count: usize,
}

impl Default for CalibrationData {
  fn default() -> Self {
    Self::new()
  }
}

impl CalibrationData {
  /// Create empty calibration data
  pub fn new() -> Self {
    Self {
      pieces: [None; MAX_PIECES],
      count: 0,
    }
  }

  /// Clear all calibration data
  pub fn clear(&mut self) {
    self.pieces = [None; MAX_PIECES];
    self.count = 0;
  }

  /// Add a piece calibration entry
  pub fn add(&mut self, chip_id: [u8; RFID_BYTES], piece: Piece) -> bool {
    if self.count >= MAX_PIECES {
      return false;
    }

    // Check if this chip ID is already calibrated
    if self.lookup(&chip_id).is_some() {
      return false;
    }

    self.pieces[self.count] = Some(PieceCalibration { chip_id, piece });
    self.count += 1;
    true
  }

  /// Look up a piece by its RFID chip ID
  pub fn lookup(&self, chip_id: &[u8; RFID_BYTES]) -> Option<Piece> {
    for entry in &self.pieces[..self.count] {
      if let Some(cal) = entry {
        if &cal.chip_id == chip_id {
          return Some(cal.piece);
        }
      }
    }
    None
  }

  /// Number of calibrated pieces
  pub fn count(&self) -> usize {
    self.count
  }

  /// Check if calibration is complete (all 32 pieces calibrated)
  pub fn is_complete(&self) -> bool {
    self.count >= 32
  }

  /// Calibrate from an RFID reading assuming pieces are in starting position
  ///
  /// Returns the number of pieces successfully calibrated.
  pub fn calibrate_from_starting_position(&mut self, reading: &RfidReading) -> usize {
    self.clear();
    let mut calibrated = 0;

    for &(square, piece) in &STARTING_LAYOUT {
      let chip_id = reading.chip_id(square);

      // Skip empty squares (no RFID detected)
      if chip_id == [0, 0, 0, 0, 0] {
        continue;
      }

      if self.add(chip_id, piece) {
        calibrated += 1;
      }
    }

    calibrated
  }

  /// Convert an RFID reading to a board state using calibration
  ///
  /// Returns an array of 64 optional pieces.
  pub fn reading_to_board(&self, reading: &RfidReading) -> [Option<Piece>; 64] {
    let mut board = [None; 64];

    for square in 0..64 {
      let chip_id = reading.chip_id(square as u8);
      if chip_id != [0, 0, 0, 0, 0] {
        board[square] = self.lookup(&chip_id);
      }
    }

    board
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_piece_is_white() {
    assert!(Piece::WhitePawn.is_white());
    assert!(Piece::WhiteKing.is_white());
    assert!(!Piece::BlackPawn.is_white());
    assert!(!Piece::BlackKing.is_white());
  }

  #[test]
  fn test_piece_to_char() {
    assert_eq!(Piece::WhiteKing.to_char(), 'K');
    assert_eq!(Piece::BlackKing.to_char(), 'k');
    assert_eq!(Piece::WhitePawn.to_char(), 'P');
    assert_eq!(Piece::BlackQueen.to_char(), 'q');
  }

  #[test]
  fn test_calibration_add_lookup() {
    let mut cal = CalibrationData::new();

    let chip1 = [1, 2, 3, 4, 5];
    let chip2 = [6, 7, 8, 9, 10];

    assert!(cal.add(chip1, Piece::WhiteKing));
    assert!(cal.add(chip2, Piece::BlackQueen));

    assert_eq!(cal.lookup(&chip1), Some(Piece::WhiteKing));
    assert_eq!(cal.lookup(&chip2), Some(Piece::BlackQueen));
    assert_eq!(cal.lookup(&[0, 0, 0, 0, 0]), None);
  }

  #[test]
  fn test_calibration_no_duplicates() {
    let mut cal = CalibrationData::new();

    let chip = [1, 2, 3, 4, 5];

    assert!(cal.add(chip, Piece::WhiteKing));
    assert!(!cal.add(chip, Piece::BlackKing)); // Duplicate

    assert_eq!(cal.count(), 1);
  }

  #[test]
  fn test_calibration_clear() {
    let mut cal = CalibrationData::new();

    cal.add([1, 2, 3, 4, 5], Piece::WhiteKing);
    assert_eq!(cal.count(), 1);

    cal.clear();
    assert_eq!(cal.count(), 0);
  }
}
