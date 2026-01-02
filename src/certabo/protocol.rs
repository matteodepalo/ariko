//! Certabo board RFID protocol parser
//!
//! The Certabo board sends 320 space-separated ASCII decimal numbers,
//! representing 64 squares × 5 bytes per square (40-bit RFID chip ID).
//!
//! Square order: a1=0, b1=1, ..., h1=7, a2=8, ..., h8=63
//! Message is terminated by newline (`\n`).

/// Number of squares on the chess board
pub const NUM_SQUARES: usize = 64;

/// Number of bytes per RFID chip ID
pub const RFID_BYTES: usize = 5;

/// Total number of values in a complete reading
pub const TOTAL_VALUES: usize = NUM_SQUARES * RFID_BYTES; // 320

pub fn write_ascii_number(buf: &mut [u8], pos: &mut usize, value: u8) {
    if value >= 100 {
        buf[*pos] = b'0' + (value / 100);
        *pos += 1;
        buf[*pos] = b'0' + ((value / 10) % 10);
        *pos += 1;
        buf[*pos] = b'0' + (value % 10);
        *pos += 1;
    } else if value >= 10 {
        buf[*pos] = b'0' + (value / 10);
        *pos += 1;
        buf[*pos] = b'0' + (value % 10);
        *pos += 1;
    } else {
        buf[*pos] = b'0' + value;
        *pos += 1;
    }
}

/// Raw RFID reading from the board (64 squares × 5 bytes each)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RfidReading {
  /// RFID chip IDs for each square (a1=0, b1=1, ..., h8=63)
  pub chip_ids: [[u8; RFID_BYTES]; NUM_SQUARES],
}

impl Default for RfidReading {
  fn default() -> Self {
    Self {
      chip_ids: [[0u8; RFID_BYTES]; NUM_SQUARES],
    }
  }
}

impl RfidReading {
  /// Create a new empty RFID reading
  pub fn new() -> Self {
    Self::default()
  }

  /// Parse from 320 space-separated ASCII decimal values
  ///
  /// Example input: "0 0 0 0 0 147 32 88 192 12 ..." (newline terminated)
  ///
  /// Returns `None` if parsing fails or the data is incomplete.
  pub fn parse(data: &[u8]) -> Option<Self> {
    let mut reading = Self::new();
    let mut value_count = 0;

    // Parse space-separated decimal numbers
    let mut current_value: u16 = 0;
    let mut has_digit = false;

    for &byte in data {
      match byte {
        b'0'..=b'9' => {
          current_value = current_value
            .saturating_mul(10)
            .saturating_add((byte - b'0') as u16);
          has_digit = true;
        }
        b' ' | b'\n' | b'\r' => {
          if has_digit {
            if value_count >= TOTAL_VALUES {
              return None; // Too many values
            }

            let square_idx = value_count / RFID_BYTES;
            let byte_idx = value_count % RFID_BYTES;

            if current_value > 255 {
              return None; // Value out of range
            }

            reading.chip_ids[square_idx][byte_idx] = current_value as u8;
            value_count += 1;
            current_value = 0;
            has_digit = false;
          }

          if byte == b'\n' {
            break; // End of message
          }
        }
        _ => {
          return None; // Invalid character
        }
      }
    }

    // Handle last value if no trailing space/newline
    if has_digit && value_count < TOTAL_VALUES {
      let square_idx = value_count / RFID_BYTES;
      let byte_idx = value_count % RFID_BYTES;

      if current_value > 255 {
        return None;
      }

      reading.chip_ids[square_idx][byte_idx] = current_value as u8;
      value_count += 1;
    }

    // Must have exactly 320 values
    if value_count == TOTAL_VALUES {
      Some(reading)
    } else {
      None
    }
  }

  /// Check if a square has a piece (non-zero RFID)
  pub fn has_piece(&self, square: u8) -> bool {
    let chip_id = self.chip_ids[square as usize];
    chip_id != [0, 0, 0, 0, 0]
  }

  /// Get the RFID chip ID for a specific square
  pub fn chip_id(&self, square: u8) -> [u8; RFID_BYTES] {
    self.chip_ids[square as usize]
  }

  /// Convert file (0-7) and rank (0-7) to square index
  pub fn square_index(file: u8, rank: u8) -> u8 {
    rank * 8 + file
  }

  /// Convert square index to (file, rank)
  pub fn index_to_coords(square: u8) -> (u8, u8) {
    let file = square % 8;
    let rank = square / 8;
    (file, rank)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// Helper to build a 320-value test data buffer (all zeros with spaces)
  fn build_zeros_buffer() -> [u8; 639] {
    let mut data = [0u8; 639]; // 320 single-digit values + 319 spaces
    for i in 0..320 {
      data[i * 2] = b'0';
      if i < 319 {
        data[i * 2 + 1] = b' ';
      }
    }
    data
  }

  fn write_number(data: &mut [u8], pos: &mut usize, value: u8) {
    write_ascii_number(data, pos, value);
  }

  #[test]
  fn test_empty_reading() {
    let reading = RfidReading::new();
    for square in 0..64 {
      assert!(!reading.has_piece(square));
    }
  }

  #[test]
  fn test_parse_minimal() {
    let data = build_zeros_buffer();
    let reading = RfidReading::parse(&data);
    assert!(reading.is_some());
  }

  #[test]
  fn test_parse_with_values() {
    // Create test data: first square has RFID [1, 2, 3, 4, 5], rest zeros
    // "1 2 3 4 5 0 0 0 ..." (320 values)
    let mut data = [0u8; 640];
    let base = b"1 2 3 4 5 ";
    data[..10].copy_from_slice(base);
    let mut pos = 10;
    for _ in 5..320 {
      data[pos] = b'0';
      pos += 1;
      if pos < 639 {
        data[pos] = b' ';
        pos += 1;
      }
    }

    let reading = RfidReading::parse(&data[..pos]).unwrap();
    assert_eq!(reading.chip_id(0), [1, 2, 3, 4, 5]);
    assert!(reading.has_piece(0));
    assert!(!reading.has_piece(1));
  }

  #[test]
  fn test_square_index() {
    // a1 = 0, h1 = 7, a8 = 56, h8 = 63
    assert_eq!(RfidReading::square_index(0, 0), 0); // a1
    assert_eq!(RfidReading::square_index(7, 0), 7); // h1
    assert_eq!(RfidReading::square_index(0, 7), 56); // a8
    assert_eq!(RfidReading::square_index(7, 7), 63); // h8
    assert_eq!(RfidReading::square_index(4, 3), 28); // e4
  }

  #[test]
  fn test_index_to_coords() {
    assert_eq!(RfidReading::index_to_coords(0), (0, 0)); // a1
    assert_eq!(RfidReading::index_to_coords(7), (7, 0)); // h1
    assert_eq!(RfidReading::index_to_coords(56), (0, 7)); // a8
    assert_eq!(RfidReading::index_to_coords(63), (7, 7)); // h8
  }

  #[test]
  fn test_parse_with_newline() {
    // Build data buffer with incrementing values: "0 1 2 ... 63 64 ... 255 0 1 2 ..."
    let mut data = [0u8; 1200]; // Extra space for multi-digit numbers
    let mut pos = 0;
    for i in 0u16..320 {
      write_number(&mut data, &mut pos, (i % 256) as u8);
      if i < 319 {
        data[pos] = b' ';
        pos += 1;
      }
    }
    data[pos] = b'\n';
    pos += 1;

    let reading = RfidReading::parse(&data[..pos]);
    assert!(reading.is_some());
  }

  #[test]
  fn test_parse_incomplete() {
    // Only 100 values instead of 320
    let mut data = [0u8; 199]; // 100 single-digit + 99 spaces
    for i in 0..100 {
      data[i * 2] = b'0';
      if i < 99 {
        data[i * 2 + 1] = b' ';
      }
    }

    let reading = RfidReading::parse(&data);
    assert!(reading.is_none());
  }

  #[test]
  fn test_parse_invalid_character() {
    let data = b"1 2 3 x 5";
    let reading = RfidReading::parse(data);
    assert!(reading.is_none());
  }
}
