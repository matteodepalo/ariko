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

/// Chess board state
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
}
