//! LED control for Certabo board
//!
//! The board has LEDs on each square that can be controlled via USB.
//! LED control format: 8 bytes, one per file (column a-h).
//! Each bit in a byte represents a rank (row 1-8).
//!
//! Example: To light e4, set byte[4] = 0b00001000 (bit 3 for rank 4)

/// LED control state for the board
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LedState {
  /// One byte per file (column), bits represent ranks
  files: [u8; 8],
}

impl LedState {
  /// Create a new LED state with all LEDs off
  pub fn new() -> Self {
    Self::default()
  }

  /// Create a new LED state with all LEDs on
  pub fn all_on() -> Self {
    Self { files: [0xFF; 8] }
  }

  /// Turn on LED at specific square
  pub fn set(&mut self, square: u8) {
    let file = (square % 8) as usize;
    let rank = square / 8;
    self.files[file] |= 1 << rank;
  }

  /// Turn off LED at specific square
  pub fn clear(&mut self, square: u8) {
    let file = (square % 8) as usize;
    let rank = square / 8;
    self.files[file] &= !(1 << rank);
  }

  /// Toggle LED at specific square
  pub fn toggle(&mut self, square: u8) {
    let file = (square % 8) as usize;
    let rank = square / 8;
    self.files[file] ^= 1 << rank;
  }

  /// Check if LED at specific square is on
  pub fn is_on(&self, square: u8) -> bool {
    let file = (square % 8) as usize;
    let rank = square / 8;
    (self.files[file] & (1 << rank)) != 0
  }

  /// Turn off all LEDs
  pub fn clear_all(&mut self) {
    self.files = [0; 8];
  }

  /// Get the raw bytes to send to the board
  pub fn as_bytes(&self) -> &[u8; 8] {
    &self.files
  }

  /// Set LED state from file and rank coordinates
  pub fn set_coord(&mut self, file: u8, rank: u8) {
    if file < 8 && rank < 8 {
      self.files[file as usize] |= 1 << rank;
    }
  }

  /// Set multiple squares from an iterator
  pub fn set_squares<I: IntoIterator<Item = u8>>(&mut self, squares: I) {
    for square in squares {
      self.set(square);
    }
  }

  /// Create LED state highlighting a move (source and destination)
  pub fn from_move(from: u8, to: u8) -> Self {
    let mut state = Self::new();
    state.set(from);
    state.set(to);
    state
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_all_off() {
    let state = LedState::new();
    for square in 0..64 {
      assert!(!state.is_on(square));
    }
  }

  #[test]
  fn test_all_on() {
    let state = LedState::all_on();
    for square in 0..64 {
      assert!(state.is_on(square));
    }
  }

  #[test]
  fn test_set_and_clear() {
    let mut state = LedState::new();

    state.set(0); // a1
    assert!(state.is_on(0));

    state.set(63); // h8
    assert!(state.is_on(63));

    state.clear(0);
    assert!(!state.is_on(0));
    assert!(state.is_on(63));
  }

  #[test]
  fn test_toggle() {
    let mut state = LedState::new();

    state.toggle(28); // e4
    assert!(state.is_on(28));

    state.toggle(28);
    assert!(!state.is_on(28));
  }

  #[test]
  fn test_e4_byte_representation() {
    let mut state = LedState::new();
    state.set(28); // e4 = file 4, rank 3

    let bytes = state.as_bytes();
    // File 4 (e) should have bit 3 (rank 4) set
    assert_eq!(bytes[4], 0b00001000);

    // All other files should be 0
    for (i, &byte) in bytes.iter().enumerate() {
      if i != 4 {
        assert_eq!(byte, 0);
      }
    }
  }

  #[test]
  fn test_from_move() {
    let state = LedState::from_move(12, 28); // e2 to e4

    assert!(state.is_on(12));
    assert!(state.is_on(28));

    let bytes = state.as_bytes();
    // e2 = file 4, rank 1; e4 = file 4, rank 3
    assert_eq!(bytes[4], 0b00001010); // bits 1 and 3 set
  }

  #[test]
  fn test_set_squares() {
    let mut state = LedState::new();
    state.set_squares([0, 7, 56, 63]); // Corners

    assert!(state.is_on(0)); // a1
    assert!(state.is_on(7)); // h1
    assert!(state.is_on(56)); // a8
    assert!(state.is_on(63)); // h8
  }

  #[test]
  fn test_clear_all() {
    let mut state = LedState::all_on();
    state.clear_all();

    for square in 0..64 {
      assert!(!state.is_on(square));
    }
  }
}
