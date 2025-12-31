//! Game state management
//!
//! Wraps the chess-engine crate for move validation and game state tracking.

use alloc::vec::Vec;
use chess_engine::{Board, Color as ChessColor, GameResult, Move, Position};

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
  /// Chess board state (from chess-engine crate)
  board: Board,
  /// Current game status
  status: GameStatus,
  /// Chess timer
  timer: ChessTimer,
  /// Square index of currently lifted piece (if any)
  lifted_piece: Option<u8>,
  /// Square from which a piece was lifted (for move detection)
  lift_square: Option<u8>,
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
      board: Board::default(),
      status: GameStatus::WaitingForCalibration,
      timer: ChessTimer::new(),
      lifted_piece: None,
      lift_square: None,
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
    match self.board.get_turn_color() {
      ChessColor::White => Color::White,
      ChessColor::Black => Color::Black,
    }
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
  pub fn is_legal_move(&self, from: u8, to: u8) -> bool {
    let from_pos = index_to_position(from);
    let to_pos = index_to_position(to);

    self
      .board
      .get_legal_moves()
      .iter()
      .any(|m| m.get_from() == from_pos && m.get_to() == to_pos)
  }

  /// Get all legal destination squares for a piece at the given square
  pub fn legal_destinations(&self, from: u8) -> Vec<u8> {
    let from_pos = index_to_position(from);

    self
      .board
      .get_legal_moves()
      .iter()
      .filter(|m| m.get_from() == from_pos)
      .map(|m| position_to_index(m.get_to()))
      .collect()
  }

  /// Make a move on the board
  ///
  /// Returns `true` if the move was legal and made, `false` otherwise.
  pub fn make_move(&mut self, from: u8, to: u8) -> bool {
    let from_pos = index_to_position(from);
    let to_pos = index_to_position(to);

    // Find the matching legal move
    let legal_moves = self.board.get_legal_moves();
    let chess_move = legal_moves
      .iter()
      .find(|m| m.get_from() == from_pos && m.get_to() == to_pos);

    if let Some(m) = chess_move {
      self.board = self.board.play_move(*m);

      // Check for game end conditions
      self.check_game_result();

      true
    } else {
      false
    }
  }

  /// Check game result and update status
  fn check_game_result(&mut self) {
    match self.board.get_game_result() {
      Some(GameResult::WhiteWins) => {
        self.set_status(GameStatus::WhiteWins);
      }
      Some(GameResult::BlackWins) => {
        self.set_status(GameStatus::BlackWins);
      }
      Some(GameResult::Draw) => {
        self.set_status(GameStatus::Draw);
      }
      None => {
        // Game continues
      }
    }
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
    self.board = Board::default();
    self.timer.reset();
    self.lifted_piece = None;
    self.lift_square = None;
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
}

/// Convert square index (0-63) to chess-engine Position
fn index_to_position(index: u8) -> Position {
  let file = index % 8;
  let rank = index / 8;
  Position::new(file as i32, rank as i32)
}

/// Convert chess-engine Position to square index (0-63)
fn position_to_index(pos: Position) -> u8 {
  let file = pos.get_col() as u8;
  let rank = pos.get_row() as u8;
  rank * 8 + file
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
  fn test_index_position_conversion() {
    // a1 = 0
    assert_eq!(position_to_index(index_to_position(0)), 0);
    // h1 = 7
    assert_eq!(position_to_index(index_to_position(7)), 7);
    // a8 = 56
    assert_eq!(position_to_index(index_to_position(56)), 56);
    // h8 = 63
    assert_eq!(position_to_index(index_to_position(63)), 63);
    // e4 = 28
    assert_eq!(position_to_index(index_to_position(28)), 28);
  }

  #[test]
  fn test_legal_move_e2e4() {
    let game = GameState::new();
    // e2 = 12, e4 = 28
    assert!(game.is_legal_move(12, 28));
  }

  #[test]
  fn test_illegal_move() {
    let game = GameState::new();
    // e2 = 12, e5 = 36 (pawn can't move 3 squares)
    assert!(!game.is_legal_move(12, 36));
  }

  #[test]
  fn test_make_move() {
    let mut game = GameState::new();
    game.set_status(GameStatus::InProgress);

    // 1. e4
    assert!(game.make_move(12, 28));
    assert_eq!(game.current_turn(), Color::Black);

    // 1... e5
    assert!(game.make_move(52, 36));
    assert_eq!(game.current_turn(), Color::White);
  }

  #[test]
  fn test_legal_destinations() {
    let game = GameState::new();
    // e2 pawn can go to e3 (20) or e4 (28)
    let dests = game.legal_destinations(12);
    assert!(dests.contains(&20));
    assert!(dests.contains(&28));
    assert_eq!(dests.len(), 2);
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
  }
}
