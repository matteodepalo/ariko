//! Game state management
//!
//! Simple board state tracking for MVP. No move validation.

use crate::game::timer::{ChessTimer, Color};

/// Current game status
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameStatus {
  /// Waiting for calibration before game can start
  WaitingForCalibration,
  /// Calibrated, waiting for pieces to be placed in starting position
  WaitingForSetup,
  /// Game in progress
  InProgress,
  /// Game paused
  Paused,
  /// White wins (checkmate or black timeout)
  WhiteWins,
  /// Black wins (checkmate or white timeout)
  BlackWins,
  /// Draw (stalemate, insufficient material, etc.)
  Draw,
}

/// Main game state
pub struct GameState {
  /// Current turn
  turn: Color,
  /// Current game status
  status: GameStatus,
  /// Chess timer
  timer: ChessTimer,
  /// Square index of currently lifted piece (if any)
  lifted_piece: Option<u8>,
  /// Square from which a piece was lifted (for move detection)
  lift_square: Option<u8>,
  /// Move count (half-moves)
  move_count: u16,
}

impl Default for GameState {
  fn default() -> Self {
    Self::new()
  }
}

impl GameState {
  /// Create a new game state
  pub fn new() -> Self {
    Self {
      turn: Color::White,
      status: GameStatus::WaitingForCalibration,
      timer: ChessTimer::new(),
      lifted_piece: None,
      lift_square: None,
      move_count: 0,
    }
  }

  /// Get current game status
  pub fn status(&self) -> GameStatus {
    self.status
  }

  /// Set game status
  pub fn set_status(&mut self, status: GameStatus) {
    self.status = status;

    // Start/stop timer based on status
    match status {
      GameStatus::InProgress => self.timer.start(),
      GameStatus::Paused | GameStatus::WhiteWins | GameStatus::BlackWins | GameStatus::Draw => {
        self.timer.stop()
      }
      _ => {}
    }
  }

  /// Get the current turn color
  pub fn current_turn(&self) -> Color {
    self.turn
  }

  /// Get timer reference
  pub fn timer(&self) -> &ChessTimer {
    &self.timer
  }

  /// Get mutable timer reference
  pub fn timer_mut(&mut self) -> &mut ChessTimer {
    &mut self.timer
  }

  /// Check if a move from one square to another is legal
  /// For MVP, all moves are considered legal (physical board enforces this)
  pub fn is_legal_move(&self, _from: u8, _to: u8) -> bool {
    true
  }

  /// Get all legal destination squares for a piece at the given square
  /// For MVP, returns empty (no highlights)
  pub fn legal_destinations(&self, _from: u8) -> [u8; 0] {
    []
  }

  /// Make a move on the board
  ///
  /// For MVP, just switches turn. Returns `true` always.
  pub fn make_move(&mut self, _from: u8, _to: u8) -> bool {
    // Switch turns
    self.turn = match self.turn {
      Color::White => Color::Black,
      Color::Black => Color::White,
    };
    self.move_count += 1;
    self.lifted_piece = None;
    self.lift_square = None;
    true
  }

  /// Record that a piece was lifted from a square
  pub fn piece_lifted(&mut self, square: u8) {
    self.lifted_piece = Some(square);
    self.lift_square = Some(square);
  }

  /// Record that a piece was placed on a square
  ///
  /// Returns `Some((from, to))` if this completes a move, `None` otherwise.
  pub fn piece_placed(&mut self, square: u8) -> Option<(u8, u8)> {
    if let Some(from) = self.lift_square {
      self.lifted_piece = None;
      self.lift_square = None;

      if from != square {
        return Some((from, square));
      }
    }
    None
  }

  /// Get the currently lifted piece square (if any)
  pub fn lifted_piece(&self) -> Option<u8> {
    self.lifted_piece
  }

  /// Reset the game to starting position
  pub fn reset(&mut self) {
    self.turn = Color::White;
    self.timer.reset();
    self.lifted_piece = None;
    self.lift_square = None;
    self.move_count = 0;
    self.status = GameStatus::WaitingForSetup;
  }

  /// Update timer with elapsed time
  ///
  /// Returns `true` if time expired for the current player.
  pub fn tick_timer(&mut self, elapsed_ms: u32) -> bool {
    if self.status != GameStatus::InProgress {
      return false;
    }

    let expired = self.timer.tick(self.current_turn(), elapsed_ms);

    if expired {
      // Current player loses on time
      match self.current_turn() {
        Color::White => self.set_status(GameStatus::BlackWins),
        Color::Black => self.set_status(GameStatus::WhiteWins),
      }
    }

    expired
  }

  /// Toggle pause state
  pub fn toggle_pause(&mut self) {
    match self.status {
      GameStatus::InProgress => self.set_status(GameStatus::Paused),
      GameStatus::Paused => self.set_status(GameStatus::InProgress),
      _ => {}
    }
  }

  /// Get move count
  pub fn move_count(&self) -> u16 {
    self.move_count
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_game() {
    let game = GameState::new();
    assert_eq!(game.status(), GameStatus::WaitingForCalibration);
    assert_eq!(game.current_turn(), Color::White);
  }

  #[test]
  fn test_make_move_switches_turn() {
    let mut game = GameState::new();
    game.set_status(GameStatus::InProgress);

    assert_eq!(game.current_turn(), Color::White);
    game.make_move(12, 28);
    assert_eq!(game.current_turn(), Color::Black);
    game.make_move(52, 36);
    assert_eq!(game.current_turn(), Color::White);
  }

  #[test]
  fn test_piece_lift_and_place() {
    let mut game = GameState::new();

    game.piece_lifted(12); // e2
    assert_eq!(game.lifted_piece(), Some(12));

    let result = game.piece_placed(28); // e4
    assert_eq!(result, Some((12, 28)));
    assert_eq!(game.lifted_piece(), None);
  }

  #[test]
  fn test_piece_put_back() {
    let mut game = GameState::new();

    game.piece_lifted(12); // e2
    let result = game.piece_placed(12); // Same square
    assert_eq!(result, None);
  }

  #[test]
  fn test_pause_toggle() {
    let mut game = GameState::new();
    game.set_status(GameStatus::InProgress);

    game.toggle_pause();
    assert_eq!(game.status(), GameStatus::Paused);

    game.toggle_pause();
    assert_eq!(game.status(), GameStatus::InProgress);
  }

  #[test]
  fn test_reset() {
    let mut game = GameState::new();
    game.set_status(GameStatus::InProgress);
    game.make_move(12, 28);

    game.reset();
    assert_eq!(game.status(), GameStatus::WaitingForSetup);
    assert_eq!(game.current_turn(), Color::White);
    assert_eq!(game.move_count(), 0);
  }
}
