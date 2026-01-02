//! LED control for Certabo board
//!
//! The board has LEDs on each square that can be controlled via USB.
//! LED control format: 8 bytes, one per rank (row 1-8).
//! Byte 0 = rank 8, byte 7 = rank 1.
//! Each bit in a byte represents a file (column): bit 0 = file a, bit 7 = file h.
//!
//! Example: To light e4, set byte[4] = 0b00010000 (byte 4 for rank 4, bit 4 for file e)

/// LED control state for the board
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LedState {
  /// One byte per rank (row), bits represent files (columns)
  /// Index 0 = rank 8, index 7 = rank 1
  ranks: [u8; 8],
}

impl LedState {
  /// Create a new LED state with all LEDs off
  pub fn new() -> Self {
    Self::default()
  }

  /// Create a new LED state with all LEDs on
  pub fn all_on() -> Self {
    Self { ranks: [0xFF; 8] }
  }

  /// Turn on LED at specific square (0-63, where 0=a1, 7=h1, 8=a2, etc.)
  pub fn set(&mut self, square: u8) {
    let file = square % 8;
    let rank = square / 8;
    // Byte index: rank 1 (rank=0) -> byte 7, rank 8 (rank=7) -> byte 0
    let byte_idx = (7 - rank) as usize;
    self.ranks[byte_idx] |= 1 << file;
  }

  /// Turn off LED at specific square
  pub fn clear(&mut self, square: u8) {
    let file = square % 8;
    let rank = square / 8;
    let byte_idx = (7 - rank) as usize;
    self.ranks[byte_idx] &= !(1 << file);
  }

  /// Toggle LED at specific square
  pub fn toggle(&mut self, square: u8) {
    let file = square % 8;
    let rank = square / 8;
    let byte_idx = (7 - rank) as usize;
    self.ranks[byte_idx] ^= 1 << file;
  }

  /// Check if LED at specific square is on
  pub fn is_on(&self, square: u8) -> bool {
    let file = square % 8;
    let rank = square / 8;
    let byte_idx = (7 - rank) as usize;
    (self.ranks[byte_idx] & (1 << file)) != 0
  }

  /// Turn off all LEDs
  pub fn clear_all(&mut self) {
    self.ranks = [0; 8];
  }

  /// Get the raw bytes to send to the board
  pub fn as_bytes(&self) -> &[u8; 8] {
    &self.ranks
  }

  /// Create LED state from raw bytes
  pub fn from_bytes(bytes: &[u8; 8]) -> Self {
    Self { ranks: *bytes }
  }

  /// Set LED state from file and rank coordinates (both 0-indexed)
  pub fn set_coord(&mut self, file: u8, rank: u8) {
    if file < 8 && rank < 8 {
      let byte_idx = (7 - rank) as usize;
      self.ranks[byte_idx] |= 1 << file;
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

  /// Create a wave animation sweeping left-to-right across piece rows
  ///
  /// Lights up ranks 1-2 (white) and 7-8 (black) with a wave pattern.
  /// `frame` determines which file(s) are lit (wraps every 8 frames).
  /// Uses 3 adjacent files for a smoother wave effect.
  pub fn wave(frame: u8) -> Self {
    let mut state = Self::new();
    // Slow down: advance every 2 frames
    let pos = ((frame / 2) % 8) as u8;

    // Light 3 adjacent files for a wider, smoother wave
    let mut file_mask: u8 = 0;
    for i in 0..3 {
      let file = (pos + i) % 8;
      file_mask |= 1 << file;
    }

    // Set on ranks 1, 2, 7, 8 (byte indices 7, 6, 1, 0)
    state.ranks[7] = file_mask; // rank 1
    state.ranks[6] = file_mask; // rank 2
    state.ranks[1] = file_mask; // rank 7
    state.ranks[0] = file_mask; // rank 8

    state
  }

  /// Create a spinner frame for loading animation
  ///
  /// The spinner rotates around the board's perimeter (28 squares).
  /// `frame` determines which segment is lit (wraps automatically).
  /// `segment_len` is how many consecutive LEDs are lit (default 4).
  pub fn spinner(frame: u8, segment_len: u8) -> Self {
    const PERIMETER: [u8; 28] = [
      0, 1, 2, 3, 4, 5, 6, 7,       // bottom row (a1-h1)
      15, 23, 31, 39, 47, 55,      // right column (h2-h7)
      63, 62, 61, 60, 59, 58, 57, 56, // top row (h8-a8)
      48, 40, 32, 24, 16, 8,       // left column (a7-a2)
    ];

    let mut state = Self::new();
    let start = (frame as usize) % PERIMETER.len();

    for i in 0..segment_len as usize {
      let idx = (start + i) % PERIMETER.len();
      state.set(PERIMETER[idx]);
    }

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
    state.set(28); // e4 = file 4, rank 3 (0-indexed)

    let bytes = state.as_bytes();
    // Rank 4 (index 3) -> byte index 7-3=4, file e (4) -> bit 4
    assert_eq!(bytes[4], 0b00010000);

    // All other ranks should be 0
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
    // e2 = file 4, rank 1 (0-indexed) -> byte 6, bit 4
    // e4 = file 4, rank 3 (0-indexed) -> byte 4, bit 4
    assert_eq!(bytes[4], 0b00010000); // e4
    assert_eq!(bytes[6], 0b00010000); // e2
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

  #[test]
  fn test_from_bytes() {
    let bytes: [u8; 8] = [0x01, 0x00, 0x00, 0x00, 0x10, 0x00, 0x10, 0x01];
    let state = LedState::from_bytes(&bytes);
    assert!(state.is_on(56)); // a8 (byte 0, bit 0)
    assert!(state.is_on(28)); // e4 (byte 4, bit 4)
    assert!(state.is_on(12)); // e2 (byte 6, bit 4)
    assert!(state.is_on(0));  // a1 (byte 7, bit 0)
    assert!(!state.is_on(63));
  }
}
