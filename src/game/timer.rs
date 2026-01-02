//! Chess clock implementation
//!
//! Fixed 10+0 time control (10 minutes per side, no increment).

/// Chess timer for both players
#[derive(Clone, Copy, Debug)]
pub struct ChessTimer {
  /// White's remaining time in milliseconds
  white_time_ms: u32,
  /// Black's remaining time in milliseconds
  black_time_ms: u32,
  /// Whether the timer is running
  running: bool,
}

/// Initial time per player (10 minutes = 600,000 ms)
pub const INITIAL_TIME_MS: u32 = 10 * 60 * 1000;

/// Low time warning threshold (30 seconds)
pub const LOW_TIME_THRESHOLD_MS: u32 = 30 * 1000;

/// Player color for timer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
  White,
  Black,
}

impl Default for ChessTimer {
  fn default() -> Self {
    Self::new()
  }
}

impl ChessTimer {
  /// Create a new timer with default time control (10+0)
  pub fn new() -> Self {
    Self {
      white_time_ms: INITIAL_TIME_MS,
      black_time_ms: INITIAL_TIME_MS,
      running: false,
    }
  }

  /// Start the timer
  pub fn start(&mut self) {
    self.running = true;
  }

  /// Stop/pause the timer
  pub fn stop(&mut self) {
    self.running = false;
  }


  /// Reset both clocks to initial time
  pub fn reset(&mut self) {
    self.white_time_ms = INITIAL_TIME_MS;
    self.black_time_ms = INITIAL_TIME_MS;
    self.running = false;
  }

  /// Update the timer, decrementing the active player's time
  ///
  /// Returns `true` if time ran out for the active player.
  pub fn tick(&mut self, active_color: Color, elapsed_ms: u32) -> bool {
    if !self.running {
      return false;
    }

    match active_color {
      Color::White => {
        self.white_time_ms = self.white_time_ms.saturating_sub(elapsed_ms);
        self.white_time_ms == 0
      }
      Color::Black => {
        self.black_time_ms = self.black_time_ms.saturating_sub(elapsed_ms);
        self.black_time_ms == 0
      }
    }
  }

  /// Get remaining time for a player in milliseconds
  pub fn time_remaining(&self, color: Color) -> u32 {
    match color {
      Color::White => self.white_time_ms,
      Color::Black => self.black_time_ms,
    }
  }

  /// Check if a player is in low time (< 30 seconds)
  pub fn is_low_time(&self, color: Color) -> bool {
    self.time_remaining(color) < LOW_TIME_THRESHOLD_MS
  }

  fn format_time(time_ms: u32) -> (u8, u8) {
    let total_seconds = time_ms / 1000;
    let minutes = (total_seconds / 60) as u8;
    let seconds = (total_seconds % 60) as u8;
    (minutes, seconds)
  }

  /// Get formatted time for a player (minutes, seconds)
  pub fn formatted_time(&self, color: Color) -> (u8, u8) {
    Self::format_time(self.time_remaining(color))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_timer() {
    let timer = ChessTimer::new();
    assert_eq!(timer.time_remaining(Color::White), INITIAL_TIME_MS);
    assert_eq!(timer.time_remaining(Color::Black), INITIAL_TIME_MS);
    assert!(!timer.running);
  }

  #[test]
  fn test_start_stop() {
    let mut timer = ChessTimer::new();
    assert!(!timer.running);

    timer.start();
    assert!(timer.running);

    timer.stop();
    assert!(!timer.running);
  }

  #[test]
  fn test_tick_decrements_active_player() {
    let mut timer = ChessTimer::new();
    timer.start();

    // Tick for white
    timer.tick(Color::White, 1000);
    assert_eq!(timer.time_remaining(Color::White), INITIAL_TIME_MS - 1000);
    assert_eq!(timer.time_remaining(Color::Black), INITIAL_TIME_MS);

    // Tick for black
    timer.tick(Color::Black, 2000);
    assert_eq!(timer.time_remaining(Color::White), INITIAL_TIME_MS - 1000);
    assert_eq!(timer.time_remaining(Color::Black), INITIAL_TIME_MS - 2000);
  }

  #[test]
  fn test_tick_when_stopped() {
    let mut timer = ChessTimer::new();
    // Timer not started, tick should do nothing
    timer.tick(Color::White, 1000);
    assert_eq!(timer.time_remaining(Color::White), INITIAL_TIME_MS);
  }

  #[test]
  fn test_time_expiry() {
    let mut timer = ChessTimer::new();
    timer.start();

    timer.tick(Color::White, INITIAL_TIME_MS - 100);
    assert!(timer.time_remaining(Color::White) > 0);

    let expired = timer.tick(Color::White, 100);
    assert!(expired);
    assert_eq!(timer.time_remaining(Color::White), 0);
    assert!(timer.time_remaining(Color::Black) > 0);
  }

  #[test]
  fn test_low_time_warning() {
    let mut timer = ChessTimer::new();
    timer.start();

    // Not low time yet
    assert!(!timer.is_low_time(Color::White));

    // Reduce to just below threshold
    timer.tick(Color::White, INITIAL_TIME_MS - LOW_TIME_THRESHOLD_MS + 1000);
    assert!(timer.is_low_time(Color::White));
  }

  #[test]
  fn test_formatted_time() {
    let mut timer = ChessTimer::new();
    assert_eq!(timer.formatted_time(Color::White), (10, 0)); // 10:00

    timer.start();
    timer.tick(Color::White, 535_000);
    assert_eq!(timer.formatted_time(Color::White), (1, 5)); // 1:05

    timer.tick(Color::White, 35_000);
    assert_eq!(timer.formatted_time(Color::White), (0, 30)); // 0:30

    timer.tick(Color::White, 30_000);
    assert_eq!(timer.formatted_time(Color::White), (0, 0)); // 0:00
  }

  #[test]
  fn test_reset() {
    let mut timer = ChessTimer::new();
    timer.start();
    timer.tick(Color::White, 60_000);

    timer.reset();
    assert_eq!(timer.time_remaining(Color::White), INITIAL_TIME_MS);
    assert_eq!(timer.time_remaining(Color::Black), INITIAL_TIME_MS);
    assert!(!timer.running);
  }

  #[test]
  fn test_saturating_sub() {
    let mut timer = ChessTimer::new();
    timer.start();

    // Try to subtract more than available
    timer.tick(Color::White, INITIAL_TIME_MS + 10000);
    assert_eq!(timer.time_remaining(Color::White), 0);
  }
}
