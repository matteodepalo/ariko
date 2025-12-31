//! Lightweight chess board representation and move validation
//!
//! Implements basic move rules without heavy lookup tables.
//! Generates pseudo-legal moves (doesn't verify king safety).

/// Chess piece types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King,
}

/// Piece color
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceColor {
  White,
  Black,
}

impl PieceColor {
  pub fn opponent(self) -> Self {
    match self {
      PieceColor::White => PieceColor::Black,
      PieceColor::Black => PieceColor::White,
    }
  }
}

/// A chess piece with type and color
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
  pub piece_type: PieceType,
  pub color: PieceColor,
}

impl Piece {
  pub const fn new(piece_type: PieceType, color: PieceColor) -> Self {
    Self { piece_type, color }
  }
}

/// Maximum destinations for any piece
pub const MAX_DESTINATIONS: usize = 28;

/// Collection of destination squares
pub struct Destinations {
  squares: [u8; MAX_DESTINATIONS],
  count: usize,
}

impl Destinations {
  pub const fn new() -> Self {
    Self {
      squares: [0; MAX_DESTINATIONS],
      count: 0,
    }
  }

  pub fn push(&mut self, sq: u8) {
    if self.count < MAX_DESTINATIONS {
      self.squares[self.count] = sq;
      self.count += 1;
    }
  }

  pub fn contains(&self, sq: u8) -> bool {
    self.squares[..self.count].contains(&sq)
  }

  pub fn len(&self) -> usize {
    self.count
  }

  pub fn is_empty(&self) -> bool {
    self.count == 0
  }
}

impl IntoIterator for Destinations {
  type Item = u8;
  type IntoIter = DestinationsIter;

  fn into_iter(self) -> Self::IntoIter {
    DestinationsIter {
      squares: self.squares,
      count: self.count,
      index: 0,
    }
  }
}

pub struct DestinationsIter {
  squares: [u8; MAX_DESTINATIONS],
  count: usize,
  index: usize,
}

impl Iterator for DestinationsIter {
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index < self.count {
      let sq = self.squares[self.index];
      self.index += 1;
      Some(sq)
    } else {
      None
    }
  }
}

/// Game status from board perspective
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardStatus {
  /// Game ongoing
  Ongoing,
  /// Checkmate - side to move has lost
  Checkmate,
  /// Stalemate - draw
  Stalemate,
}

/// Chess board state
#[derive(Clone)]
pub struct ChessBoard {
  squares: [Option<Piece>; 64],
  side_to_move: PieceColor,
}

impl Default for ChessBoard {
  fn default() -> Self {
    Self::starting_position()
  }
}

impl ChessBoard {
  /// Create empty board
  pub const fn empty() -> Self {
    Self {
      squares: [None; 64],
      side_to_move: PieceColor::White,
    }
  }

  /// Create board with starting position
  pub fn starting_position() -> Self {
    let mut board = Self::empty();

    // White pieces (ranks 1-2)
    board.squares[0] = Some(Piece::new(PieceType::Rook, PieceColor::White));
    board.squares[1] = Some(Piece::new(PieceType::Knight, PieceColor::White));
    board.squares[2] = Some(Piece::new(PieceType::Bishop, PieceColor::White));
    board.squares[3] = Some(Piece::new(PieceType::Queen, PieceColor::White));
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[5] = Some(Piece::new(PieceType::Bishop, PieceColor::White));
    board.squares[6] = Some(Piece::new(PieceType::Knight, PieceColor::White));
    board.squares[7] = Some(Piece::new(PieceType::Rook, PieceColor::White));
    for sq in 8..16 {
      board.squares[sq] = Some(Piece::new(PieceType::Pawn, PieceColor::White));
    }

    // Black pieces (ranks 7-8)
    for sq in 48..56 {
      board.squares[sq] = Some(Piece::new(PieceType::Pawn, PieceColor::Black));
    }
    board.squares[56] = Some(Piece::new(PieceType::Rook, PieceColor::Black));
    board.squares[57] = Some(Piece::new(PieceType::Knight, PieceColor::Black));
    board.squares[58] = Some(Piece::new(PieceType::Bishop, PieceColor::Black));
    board.squares[59] = Some(Piece::new(PieceType::Queen, PieceColor::Black));
    board.squares[60] = Some(Piece::new(PieceType::King, PieceColor::Black));
    board.squares[61] = Some(Piece::new(PieceType::Bishop, PieceColor::Black));
    board.squares[62] = Some(Piece::new(PieceType::Knight, PieceColor::Black));
    board.squares[63] = Some(Piece::new(PieceType::Rook, PieceColor::Black));

    board
  }

  /// Get piece at square
  pub fn get(&self, sq: u8) -> Option<Piece> {
    self.squares[sq as usize]
  }

  /// Get side to move
  pub fn side_to_move(&self) -> PieceColor {
    self.side_to_move
  }

  /// Make a move (updates board state)
  pub fn make_move(&mut self, from: u8, to: u8) {
    let piece = self.squares[from as usize].take();
    self.squares[to as usize] = piece;
    self.side_to_move = self.side_to_move.opponent();
  }

  /// Check if a move is pseudo-legal (follows piece movement rules)
  pub fn is_pseudo_legal(&self, from: u8, to: u8) -> bool {
    let piece = match self.get(from) {
      Some(p) => p,
      None => return false,
    };

    // Must be our piece
    if piece.color != self.side_to_move {
      return false;
    }

    // Can't capture our own piece
    if let Some(target) = self.get(to) {
      if target.color == piece.color {
        return false;
      }
    }

    self.legal_destinations(from).contains(to)
  }

  /// Get all pseudo-legal destinations for a piece
  pub fn legal_destinations(&self, from: u8) -> Destinations {
    let mut dests = Destinations::new();

    let piece = match self.get(from) {
      Some(p) => p,
      None => return dests,
    };

    match piece.piece_type {
      PieceType::Pawn => self.pawn_moves(from, piece.color, &mut dests),
      PieceType::Knight => self.knight_moves(from, piece.color, &mut dests),
      PieceType::Bishop => self.sliding_moves(from, piece.color, &BISHOP_DIRS, &mut dests),
      PieceType::Rook => self.sliding_moves(from, piece.color, &ROOK_DIRS, &mut dests),
      PieceType::Queen => {
        self.sliding_moves(from, piece.color, &BISHOP_DIRS, &mut dests);
        self.sliding_moves(from, piece.color, &ROOK_DIRS, &mut dests);
      }
      PieceType::King => self.king_moves(from, piece.color, &mut dests),
    }

    dests
  }

  /// Find the king's square for a given color
  pub fn find_king(&self, color: PieceColor) -> Option<u8> {
    for sq in 0..64 {
      if let Some(piece) = self.squares[sq] {
        if piece.piece_type == PieceType::King && piece.color == color {
          return Some(sq as u8);
        }
      }
    }
    None
  }

  /// Check if a square is attacked by the given color
  pub fn is_square_attacked(&self, sq: u8, by_color: PieceColor) -> bool {
    let sq_file = (sq % 8) as i8;
    let sq_rank = (sq / 8) as i8;

    // Check pawn attacks
    let pawn_dir: i8 = match by_color {
      PieceColor::White => -8, // White pawns attack upward, so look down for attackers
      PieceColor::Black => 8,  // Black pawns attack downward, so look up for attackers
    };
    for &df in &[-1i8, 1i8] {
      if let Some(attacker_sq) = add_offset(sq, pawn_dir + df) {
        let att_file = (attacker_sq % 8) as i8;
        if (att_file - sq_file).abs() == 1 {
          if let Some(piece) = self.get(attacker_sq) {
            if piece.piece_type == PieceType::Pawn && piece.color == by_color {
              return true;
            }
          }
        }
      }
    }

    // Check knight attacks
    let knight_offsets: [i8; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
    for &offset in &knight_offsets {
      if let Some(attacker_sq) = add_offset(sq, offset) {
        let att_file = (attacker_sq % 8) as i8;
        let att_rank = (attacker_sq / 8) as i8;
        let df = (att_file - sq_file).abs();
        let dr = (att_rank - sq_rank).abs();
        if (df == 1 && dr == 2) || (df == 2 && dr == 1) {
          if let Some(piece) = self.get(attacker_sq) {
            if piece.piece_type == PieceType::Knight && piece.color == by_color {
              return true;
            }
          }
        }
      }
    }

    // Check king attacks (for adjacent squares)
    for &(df, dr) in &ALL_DIRS {
      if let Some(attacker_sq) = add_offset(sq, dr * 8 + df) {
        let att_file = (attacker_sq % 8) as i8;
        if (att_file - sq_file).abs() <= 1 {
          if let Some(piece) = self.get(attacker_sq) {
            if piece.piece_type == PieceType::King && piece.color == by_color {
              return true;
            }
          }
        }
      }
    }

    // Check sliding piece attacks (bishop/queen diagonals)
    for &(df, dr) in &BISHOP_DIRS {
      if self.is_attacked_from_direction(sq, by_color, df, dr, &[PieceType::Bishop, PieceType::Queen]) {
        return true;
      }
    }

    // Check sliding piece attacks (rook/queen lines)
    for &(df, dr) in &ROOK_DIRS {
      if self.is_attacked_from_direction(sq, by_color, df, dr, &[PieceType::Rook, PieceType::Queen]) {
        return true;
      }
    }

    false
  }

  /// Helper: check if square is attacked from a sliding direction
  fn is_attacked_from_direction(&self, sq: u8, by_color: PieceColor, df: i8, dr: i8, piece_types: &[PieceType]) -> bool {
    let mut curr_sq = sq;
    let mut prev_file = (sq % 8) as i8;

    loop {
      let offset = dr * 8 + df;
      match add_offset(curr_sq, offset) {
        Some(next_sq) => {
          let next_file = (next_sq % 8) as i8;
          if (next_file - prev_file).abs() > 1 {
            break;
          }

          if let Some(piece) = self.get(next_sq) {
            if piece.color == by_color && piece_types.contains(&piece.piece_type) {
              return true;
            }
            break; // Blocked by a piece
          }

          curr_sq = next_sq;
          prev_file = next_file;
        }
        None => break,
      }
    }

    false
  }

  /// Check if the given color's king is in check
  pub fn is_in_check(&self, color: PieceColor) -> bool {
    if let Some(king_sq) = self.find_king(color) {
      self.is_square_attacked(king_sq, color.opponent())
    } else {
      false // No king found (shouldn't happen in valid game)
    }
  }

  /// Check if a move is fully legal (doesn't leave own king in check)
  pub fn is_legal(&self, from: u8, to: u8) -> bool {
    if !self.is_pseudo_legal(from, to) {
      return false;
    }

    // Make the move on a copy and check if king is in check
    let mut test_board = self.clone();
    test_board.make_move(from, to);

    // After the move, check if the moving side's king is in check
    // Note: make_move switches side_to_move, so we check the opponent (original mover)
    !test_board.is_in_check(self.side_to_move)
  }

  /// Check if the side to move has any legal moves
  pub fn has_legal_moves(&self) -> bool {
    for from in 0..64u8 {
      if let Some(piece) = self.get(from) {
        if piece.color == self.side_to_move {
          let dests = self.legal_destinations(from);
          for to in dests {
            if self.is_legal(from, to) {
              return true;
            }
          }
        }
      }
    }
    false
  }

  /// Get the current board status (checkmate, stalemate, or ongoing)
  pub fn status(&self) -> BoardStatus {
    if self.has_legal_moves() {
      BoardStatus::Ongoing
    } else if self.is_in_check(self.side_to_move) {
      BoardStatus::Checkmate
    } else {
      BoardStatus::Stalemate
    }
  }

  fn pawn_moves(&self, from: u8, color: PieceColor, dests: &mut Destinations) {
    let (dir, start_rank, _promo_rank): (i8, u8, u8) = match color {
      PieceColor::White => (8, 1, 6),
      PieceColor::Black => (-8, 6, 1),
    };

    let rank = from / 8;
    let file = from % 8;

    // Forward one
    if let Some(to) = add_offset(from, dir) {
      if self.get(to).is_none() {
        dests.push(to);

        // Forward two from start
        if rank == start_rank {
          if let Some(to2) = add_offset(to, dir) {
            if self.get(to2).is_none() {
              dests.push(to2);
            }
          }
        }
      }
    }

    // Captures
    for &cap_dir in &[dir - 1, dir + 1] {
      if let Some(to) = add_offset(from, cap_dir) {
        let to_file = to % 8;
        // Ensure we didn't wrap around files
        if (to_file as i8 - file as i8).abs() == 1 {
          if let Some(target) = self.get(to) {
            if target.color != color {
              dests.push(to);
            }
          }
        }
      }
    }
  }

  fn knight_moves(&self, from: u8, color: PieceColor, dests: &mut Destinations) {
    let offsets: [i8; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
    let from_file = (from % 8) as i8;
    let from_rank = (from / 8) as i8;

    for &offset in &offsets {
      if let Some(to) = add_offset(from, offset) {
        let to_file = (to % 8) as i8;
        let to_rank = (to / 8) as i8;

        // Knight moves 2+1 squares, check valid L-shape
        let df = (to_file - from_file).abs();
        let dr = (to_rank - from_rank).abs();
        if (df == 1 && dr == 2) || (df == 2 && dr == 1) {
          if let Some(target) = self.get(to) {
            if target.color == color {
              continue;
            }
          }
          dests.push(to);
        }
      }
    }
  }

  fn king_moves(&self, from: u8, color: PieceColor, dests: &mut Destinations) {
    let from_file = (from % 8) as i8;

    for &(df, dr) in &ALL_DIRS {
      if let Some(to) = add_offset(from, dr * 8 + df) {
        let to_file = (to % 8) as i8;

        // King moves one square, check not wrapping
        if (to_file - from_file).abs() <= 1 {
          if let Some(target) = self.get(to) {
            if target.color == color {
              continue;
            }
          }
          dests.push(to);
        }
      }
    }
  }

  fn sliding_moves(&self, from: u8, color: PieceColor, dirs: &[(i8, i8)], dests: &mut Destinations) {
    let from_file = (from % 8) as i8;

    for &(df, dr) in dirs {
      let mut sq = from;
      let mut prev_file = from_file;

      loop {
        let offset = dr * 8 + df;
        match add_offset(sq, offset) {
          Some(to) => {
            let to_file = (to % 8) as i8;
            // Check for file wrap
            if (to_file - prev_file).abs() > 1 {
              break;
            }

            if let Some(target) = self.get(to) {
              if target.color != color {
                dests.push(to); // Can capture
              }
              break; // Blocked
            }

            dests.push(to);
            sq = to;
            prev_file = to_file;
          }
          None => break,
        }
      }
    }
  }
}

// Direction offsets
const ROOK_DIRS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const BISHOP_DIRS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const ALL_DIRS: [(i8, i8); 8] = [
  (1, 0), (-1, 0), (0, 1), (0, -1),
  (1, 1), (1, -1), (-1, 1), (-1, -1),
];

/// Add offset to square, returning None if out of bounds
fn add_offset(sq: u8, offset: i8) -> Option<u8> {
  let result = sq as i8 + offset;
  if result >= 0 && result < 64 {
    Some(result as u8)
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_starting_position() {
    let board = ChessBoard::starting_position();

    // White rooks
    assert_eq!(board.get(0), Some(Piece::new(PieceType::Rook, PieceColor::White)));
    assert_eq!(board.get(7), Some(Piece::new(PieceType::Rook, PieceColor::White)));

    // White king
    assert_eq!(board.get(4), Some(Piece::new(PieceType::King, PieceColor::White)));

    // Black king
    assert_eq!(board.get(60), Some(Piece::new(PieceType::King, PieceColor::Black)));

    // Pawns
    assert_eq!(board.get(8), Some(Piece::new(PieceType::Pawn, PieceColor::White)));
    assert_eq!(board.get(48), Some(Piece::new(PieceType::Pawn, PieceColor::Black)));

    // Empty squares
    assert_eq!(board.get(32), None);
  }

  #[test]
  fn test_pawn_moves_from_start() {
    let board = ChessBoard::starting_position();

    // e2 pawn can move to e3 and e4
    let dests = board.legal_destinations(12); // e2
    assert_eq!(dests.len(), 2);
    assert!(dests.contains(20)); // e3
    assert!(dests.contains(28)); // e4
  }

  #[test]
  fn test_knight_moves_from_start() {
    let board = ChessBoard::starting_position();

    // b1 knight can move to a3 and c3
    let dests = board.legal_destinations(1); // b1
    assert_eq!(dests.len(), 2);
    assert!(dests.contains(16)); // a3
    assert!(dests.contains(18)); // c3
  }

  #[test]
  fn test_make_move() {
    let mut board = ChessBoard::starting_position();

    assert_eq!(board.side_to_move(), PieceColor::White);
    board.make_move(12, 28); // e2-e4
    assert_eq!(board.side_to_move(), PieceColor::Black);
    assert_eq!(board.get(12), None);
    assert_eq!(board.get(28), Some(Piece::new(PieceType::Pawn, PieceColor::White)));
  }

  #[test]
  fn test_pseudo_legal() {
    let board = ChessBoard::starting_position();

    // Legal pawn move
    assert!(board.is_pseudo_legal(12, 28)); // e2-e4

    // Illegal - not our piece
    assert!(!board.is_pseudo_legal(52, 36)); // e7-e5 (black's turn)

    // Illegal - no piece
    assert!(!board.is_pseudo_legal(32, 40));
  }

  #[test]
  fn test_find_king() {
    let board = ChessBoard::starting_position();
    assert_eq!(board.find_king(PieceColor::White), Some(4)); // e1
    assert_eq!(board.find_king(PieceColor::Black), Some(60)); // e8
  }

  #[test]
  fn test_not_in_check_at_start() {
    let board = ChessBoard::starting_position();
    assert!(!board.is_in_check(PieceColor::White));
    assert!(!board.is_in_check(PieceColor::Black));
  }

  #[test]
  fn test_check_by_rook() {
    // Set up position: White King on e1, Black Rook on e8
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[60] = Some(Piece::new(PieceType::Rook, PieceColor::Black));

    assert!(board.is_in_check(PieceColor::White));
    assert!(!board.is_in_check(PieceColor::Black));
  }

  #[test]
  fn test_check_by_bishop() {
    // White King on e1, Black Bishop on h4 (diagonal)
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[31] = Some(Piece::new(PieceType::Bishop, PieceColor::Black)); // h4

    assert!(board.is_in_check(PieceColor::White));
  }

  #[test]
  fn test_check_by_knight() {
    // White King on e1 (4), Black Knight on f3 (21)
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[21] = Some(Piece::new(PieceType::Knight, PieceColor::Black)); // f3

    assert!(board.is_in_check(PieceColor::White));
  }

  #[test]
  fn test_check_by_pawn() {
    // White King on e4 (28), Black Pawn on d5 (35)
    let mut board = ChessBoard::empty();
    board.squares[28] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[35] = Some(Piece::new(PieceType::Pawn, PieceColor::Black)); // d5

    assert!(board.is_in_check(PieceColor::White));
  }

  #[test]
  fn test_blocked_check() {
    // White King on e1, Black Rook on e8, White Pawn on e2 (blocks)
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[60] = Some(Piece::new(PieceType::Rook, PieceColor::Black));
    board.squares[12] = Some(Piece::new(PieceType::Pawn, PieceColor::White)); // e2

    assert!(!board.is_in_check(PieceColor::White)); // Pawn blocks
  }

  #[test]
  fn test_is_legal_blocks_self_check() {
    // White King on e1, Black Rook on e8, White Pawn on e2
    // Moving the pawn would expose king to check - should be illegal
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[60] = Some(Piece::new(PieceType::Rook, PieceColor::Black));
    board.squares[12] = Some(Piece::new(PieceType::Pawn, PieceColor::White));

    // Pawn moving forward is pseudo-legal but not fully legal (exposes king)
    assert!(board.is_pseudo_legal(12, 20)); // e2-e3
    assert!(!board.is_legal(12, 20)); // Can't move - would expose king
  }

  #[test]
  fn test_must_escape_check() {
    // White King on e1, Black Rook on e8, White's turn
    // King must move, other pieces can't move
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[60] = Some(Piece::new(PieceType::Rook, PieceColor::Black));
    board.squares[0] = Some(Piece::new(PieceType::Rook, PieceColor::White)); // a1

    assert!(board.is_in_check(PieceColor::White));

    // King can escape to d1, d2, f1, f2
    assert!(board.is_legal(4, 3)); // e1-d1
    assert!(board.is_legal(4, 5)); // e1-f1

    // White rook can't move - doesn't help with check
    assert!(!board.is_legal(0, 8)); // a1-a2 doesn't help
  }

  #[test]
  fn test_checkmate() {
    // Fool's mate position: Black queen on h4, bishop on c5, White king on e1
    // Actually, let's use a simpler back-rank mate
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White)); // e1
    board.squares[8] = Some(Piece::new(PieceType::Pawn, PieceColor::White)); // a2
    board.squares[9] = Some(Piece::new(PieceType::Pawn, PieceColor::White)); // b2
    board.squares[12] = Some(Piece::new(PieceType::Pawn, PieceColor::White)); // e2
    board.squares[13] = Some(Piece::new(PieceType::Pawn, PieceColor::White)); // f2
    board.squares[14] = Some(Piece::new(PieceType::Pawn, PieceColor::White)); // g2
    board.squares[60] = Some(Piece::new(PieceType::Rook, PieceColor::Black)); // e8

    assert!(board.is_in_check(PieceColor::White));
    assert!(!board.has_legal_moves());
    assert_eq!(board.status(), BoardStatus::Checkmate);
  }

  #[test]
  fn test_stalemate() {
    // King on a1 with no legal moves, not in check
    let mut board = ChessBoard::empty();
    board.squares[0] = Some(Piece::new(PieceType::King, PieceColor::White)); // a1
    board.squares[17] = Some(Piece::new(PieceType::Queen, PieceColor::Black)); // b3
    board.squares[10] = Some(Piece::new(PieceType::King, PieceColor::Black)); // c2

    assert!(!board.is_in_check(PieceColor::White));
    assert!(!board.has_legal_moves());
    assert_eq!(board.status(), BoardStatus::Stalemate);
  }

  #[test]
  fn test_ongoing_game() {
    let board = ChessBoard::starting_position();
    assert_eq!(board.status(), BoardStatus::Ongoing);
    assert!(board.has_legal_moves());
  }
}
