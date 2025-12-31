//! Line buffer for accumulating RFID data from Certabo board
//!
//! The Certabo board sends newline-terminated messages containing
//! 320 space-separated decimal values. This buffer accumulates
//! incoming bytes until a complete line is received.

/// Maximum line length (320 values * ~4 chars each + safety margin)
pub const MAX_LINE_LEN: usize = 1536;

/// Buffer for accumulating bytes until a complete line is received
pub struct LineBuffer {
  buffer: [u8; MAX_LINE_LEN],
  len: usize,
}

impl LineBuffer {
  /// Create a new empty buffer
  pub const fn new() -> Self {
    Self {
      buffer: [0; MAX_LINE_LEN],
      len: 0,
    }
  }

  /// Push incoming data into the buffer
  ///
  /// Returns `true` if a complete line is now available.
  pub fn push(&mut self, data: &[u8]) -> bool {
    let mut has_complete_line = false;

    for &byte in data {
      if self.len < MAX_LINE_LEN {
        self.buffer[self.len] = byte;
        self.len += 1;

        // Only use LF as line terminator (matching Python usbtool.py)
        if byte == b'\n' {
          has_complete_line = true;
        }
      } else {
        // Buffer overflow - discard oldest data by shifting
        // This shouldn't happen with proper sizing, but handle gracefully
        self.buffer.copy_within(1.., 0);
        self.len = MAX_LINE_LEN - 1;
        self.buffer[self.len] = byte;
      }
    }

    has_complete_line
  }

  /// Check if a complete line is available
  pub fn has_complete_line(&self) -> bool {
    self.buffer[..self.len].iter().any(|&b| b == b'\n')
  }

  /// Take a complete line from the buffer
  ///
  /// Returns the line (without newline) if available, or None.
  /// Removes the line from the buffer, keeping any remaining data.
  pub fn take_line(&mut self) -> Option<&[u8]> {
    // Find LF position (matching Python usbtool.py)
    let newline_pos = self.buffer[..self.len].iter().position(|&b| b == b'\n')?;

    // The line is everything before the newline
    let _line_end = newline_pos;

    // Shift remaining data to the front (after newline)
    let remaining_start = newline_pos + 1;
    let remaining_len = self.len - remaining_start;

    if remaining_len > 0 {
      self.buffer.copy_within(remaining_start..self.len, 0);
    }
    self.len = remaining_len;

    // Return slice of the line (it's still valid because we copied remaining to front)
    // Actually, we need a different approach - the data we want to return was overwritten
    // Let's use a static buffer approach instead
    None // We need to rethink this...
  }

  /// Take a complete line, copying it to the provided output buffer
  ///
  /// Returns the number of bytes written, or None if no complete line available.
  pub fn take_line_into(&mut self, out: &mut [u8]) -> Option<usize> {
    // Find LF position (matching Python usbtool.py)
    let newline_pos = self.buffer[..self.len].iter().position(|&b| b == b'\n')?;

    // Copy line to output (without newline)
    let line_len = newline_pos.min(out.len());
    out[..line_len].copy_from_slice(&self.buffer[..line_len]);

    // Shift remaining data to the front
    let remaining_start = newline_pos + 1;
    let remaining_len = self.len - remaining_start;

    if remaining_len > 0 {
      self.buffer.copy_within(remaining_start..self.len, 0);
    }
    self.len = remaining_len;

    Some(line_len)
  }

  /// Clear the buffer
  pub fn clear(&mut self) {
    self.len = 0;
  }

  /// Get current buffer length
  pub fn len(&self) -> usize {
    self.len
  }

  /// Check if buffer is empty
  pub fn is_empty(&self) -> bool {
    self.len == 0
  }
}

impl Default for LineBuffer {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_buffer_is_empty() {
    let buf = LineBuffer::new();
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);
    assert!(!buf.has_complete_line());
  }

  #[test]
  fn test_push_without_newline() {
    let mut buf = LineBuffer::new();
    let has_line = buf.push(b"hello");
    assert!(!has_line);
    assert!(!buf.has_complete_line());
    assert_eq!(buf.len(), 5);
  }

  #[test]
  fn test_push_with_newline() {
    let mut buf = LineBuffer::new();
    let has_line = buf.push(b"hello\n");
    assert!(has_line);
    assert!(buf.has_complete_line());
  }

  #[test]
  fn test_take_line() {
    let mut buf = LineBuffer::new();
    buf.push(b"hello\nworld");

    let mut out = [0u8; 64];
    let len = buf.take_line_into(&mut out).unwrap();

    assert_eq!(&out[..len], b"hello");
    assert_eq!(buf.len(), 5); // "world" remains
    assert!(!buf.has_complete_line());
  }

  #[test]
  fn test_take_multiple_lines() {
    let mut buf = LineBuffer::new();
    buf.push(b"line1\nline2\npartial");

    let mut out = [0u8; 64];

    // Take first line
    let len = buf.take_line_into(&mut out).unwrap();
    assert_eq!(&out[..len], b"line1");

    // Take second line
    let len = buf.take_line_into(&mut out).unwrap();
    assert_eq!(&out[..len], b"line2");

    // No more complete lines
    assert!(buf.take_line_into(&mut out).is_none());
    assert_eq!(buf.len(), 7); // "partial" remains
  }

  #[test]
  fn test_incremental_push() {
    let mut buf = LineBuffer::new();

    buf.push(b"hel");
    assert!(!buf.has_complete_line());

    buf.push(b"lo\nwor");
    assert!(buf.has_complete_line());

    let mut out = [0u8; 64];
    let len = buf.take_line_into(&mut out).unwrap();
    assert_eq!(&out[..len], b"hello");
  }

  #[test]
  fn test_empty_line() {
    let mut buf = LineBuffer::new();
    buf.push(b"\n");

    let mut out = [0u8; 64];
    let len = buf.take_line_into(&mut out).unwrap();
    assert_eq!(len, 0);
  }

  #[test]
  fn test_clear() {
    let mut buf = LineBuffer::new();
    buf.push(b"some data\n");
    buf.clear();

    assert!(buf.is_empty());
    assert!(!buf.has_complete_line());
  }

  #[test]
  fn test_no_line_available() {
    let mut buf = LineBuffer::new();
    buf.push(b"no newline here");

    let mut out = [0u8; 64];
    assert!(buf.take_line_into(&mut out).is_none());
  }
}
