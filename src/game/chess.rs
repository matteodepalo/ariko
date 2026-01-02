//! Chess board representation and move validation
//!
//! Implements full chess rules including castling, en passant, promotion,
//! and draw conditions (50-move rule, insufficient material).

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

/// Castling rights for one side
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CastlingRights {
  /// Can castle kingside (O-O)
  pub kingside: bool,
  /// Can castle queenside (O-O-O)
  pub queenside: bool,
}

impl CastlingRights {
  pub const fn new() -> Self {
    Self {
      kingside: true,
      queenside: true,
    }
  }

  pub const fn none() -> Self {
    Self {
      kingside: false,
      queenside: false,
    }
  }
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
  /// Stalemate - draw (no legal moves, not in check)
  Stalemate,
  /// Draw by fifty-move rule
  FiftyMoveRule,
  /// Draw by insufficient material
  InsufficientMaterial,
}

/// Maximum number of moves that can be undone
pub const MAX_UNDO_HISTORY: usize = 100;

/// Information needed to undo a move
#[derive(Clone, Copy, Debug)]
pub struct UndoInfo {
  /// From square
  pub from: u8,
  /// To square
  pub to: u8,
  /// Captured piece (if any)
  pub captured: Option<Piece>,
  /// Captured piece square (different from `to` for en passant)
  pub captured_square: u8,
  /// En passant square before the move
  pub prev_en_passant: Option<u8>,
  /// Castling rights before the move
  pub prev_white_castling: CastlingRights,
  pub prev_black_castling: CastlingRights,
  /// Halfmove clock before the move
  pub prev_halfmove_clock: u8,
  /// Was this a castling move (kingside = true, queenside = false)
  pub castling: Option<bool>,
  /// Promotion piece (if pawn promoted)
  pub promotion: Option<PieceType>,
  /// Original piece type (pawn before promotion)
  pub original_piece: PieceType,
}

/// Chess board state
#[derive(Clone)]
pub struct ChessBoard {
  squares: [Option<Piece>; 64],
  side_to_move: PieceColor,
  /// Castling rights for white
  white_castling: CastlingRights,
  /// Castling rights for black
  black_castling: CastlingRights,
  /// En passant target square (the square where the capturing pawn lands)
  en_passant: Option<u8>,
  /// Halfmove clock for 50-move rule (resets on pawn move or capture)
  halfmove_clock: u8,
  /// Full move number (increments after black's move)
  fullmove_number: u16,
  /// Undo history for takebacks
  undo_history: [Option<UndoInfo>; MAX_UNDO_HISTORY],
  /// Number of moves in undo history
  undo_count: usize,
}

impl Default for ChessBoard {
  fn default() -> Self {
    Self::starting_position()
  }
}

/// Square indices for castling
const WHITE_KING_START: u8 = 4;   // e1
const WHITE_ROOK_KINGSIDE: u8 = 7;  // h1
const WHITE_ROOK_QUEENSIDE: u8 = 0; // a1
const BLACK_KING_START: u8 = 60;  // e8
const BLACK_ROOK_KINGSIDE: u8 = 63; // h8
const BLACK_ROOK_QUEENSIDE: u8 = 56; // a8

impl ChessBoard {
  pub(crate) fn empty() -> Self {
    Self {
      squares: [None; 64],
      side_to_move: PieceColor::White,
      white_castling: CastlingRights::none(),
      black_castling: CastlingRights::none(),
      en_passant: None,
      halfmove_clock: 0,
      fullmove_number: 1,
      undo_history: [None; MAX_UNDO_HISTORY],
      undo_count: 0,
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

    // Set castling rights for starting position
    board.white_castling = CastlingRights::new();
    board.black_castling = CastlingRights::new();

    board
  }

  pub fn get(&self, sq: u8) -> Option<Piece> {
    self.squares[sq as usize]
  }

  pub fn castling_rights(&self, color: PieceColor) -> CastlingRights {
    match color {
      PieceColor::White => self.white_castling,
      PieceColor::Black => self.black_castling,
    }
  }

  /// Get information about the last move (for takeback detection)
  pub fn last_undo_info(&self) -> Option<&UndoInfo> {
    if self.undo_count > 0 {
      self.undo_history[self.undo_count - 1].as_ref()
    } else {
      None
    }
  }

  /// Make a move with optional promotion piece (updates board state)
  /// For pawn promotion, if `promotion` is None, auto-promotes to Queen.
  pub fn make_move(&mut self, from: u8, to: u8) {
    self.make_move_with_promotion(from, to, None);
  }

  /// Make a move with explicit promotion piece choice
  pub fn make_move_with_promotion(&mut self, from: u8, to: u8, promotion: Option<PieceType>) {
    let piece = match self.get(from) {
      Some(p) => p,
      None => return,
    };

    // Save state for undo
    let prev_en_passant = self.en_passant;
    let prev_white_castling = self.white_castling;
    let prev_black_castling = self.black_castling;
    let prev_halfmove_clock = self.halfmove_clock;

    let mut captured_square = to;
    let mut castling_move: Option<bool> = None;
    let mut actual_promotion: Option<PieceType> = None;

    // Determine if this is a capture
    let mut captured = self.get(to);

    // Handle en passant capture
    if piece.piece_type == PieceType::Pawn && Some(to) == self.en_passant {
      // En passant capture - the captured pawn is on a different square
      let captured_pawn_sq = match piece.color {
        PieceColor::White => to - 8, // Captured pawn is below the landing square
        PieceColor::Black => to + 8, // Captured pawn is above the landing square
      };
      captured = self.squares[captured_pawn_sq as usize].take();
      captured_square = captured_pawn_sq;
    }

    // Handle castling
    if piece.piece_type == PieceType::King {
      let from_file = from % 8;
      let to_file = to % 8;

      // Kingside castling (king moves 2 squares right)
      if from_file == 4 && to_file == 6 {
        castling_move = Some(true);
        // Move the rook
        let rook_from = if piece.color == PieceColor::White { WHITE_ROOK_KINGSIDE } else { BLACK_ROOK_KINGSIDE };
        let rook_to = to - 1; // f1 or f8
        let rook = self.squares[rook_from as usize].take();
        self.squares[rook_to as usize] = rook;
      }
      // Queenside castling (king moves 2 squares left)
      else if from_file == 4 && to_file == 2 {
        castling_move = Some(false);
        // Move the rook
        let rook_from = if piece.color == PieceColor::White { WHITE_ROOK_QUEENSIDE } else { BLACK_ROOK_QUEENSIDE };
        let rook_to = to + 1; // d1 or d8
        let rook = self.squares[rook_from as usize].take();
        self.squares[rook_to as usize] = rook;
      }
    }

    // Move the piece
    let mut moving_piece = self.squares[from as usize].take().unwrap();

    // Handle pawn promotion
    if piece.piece_type == PieceType::Pawn {
      let promo_rank = match piece.color {
        PieceColor::White => 7,
        PieceColor::Black => 0,
      };
      if to / 8 == promo_rank {
        // Promote to the specified piece, or Queen by default
        actual_promotion = Some(promotion.unwrap_or(PieceType::Queen));
        moving_piece.piece_type = actual_promotion.unwrap();
      }
    }

    self.squares[to as usize] = Some(moving_piece);

    // Update en passant square
    self.en_passant = None;
    if piece.piece_type == PieceType::Pawn {
      let from_rank = from / 8;
      let to_rank = to / 8;
      // Double pawn push
      if (from_rank == 1 && to_rank == 3) || (from_rank == 6 && to_rank == 4) {
        // Check if there's an adjacent enemy pawn that could capture
        let ep_square = (from + to) / 2; // Square the pawn passed through
        let to_file = to % 8;

        // Check for adjacent enemy pawns
        let mut has_adjacent_pawn = false;
        if to_file > 0 {
          if let Some(adj) = self.get(to - 1) {
            if adj.piece_type == PieceType::Pawn && adj.color != piece.color {
              has_adjacent_pawn = true;
            }
          }
        }
        if to_file < 7 {
          if let Some(adj) = self.get(to + 1) {
            if adj.piece_type == PieceType::Pawn && adj.color != piece.color {
              has_adjacent_pawn = true;
            }
          }
        }

        if has_adjacent_pawn {
          self.en_passant = Some(ep_square);
        }
      }
    }

    // Update castling rights
    // If king moves, lose all castling rights for that side
    if piece.piece_type == PieceType::King {
      match piece.color {
        PieceColor::White => self.white_castling = CastlingRights::none(),
        PieceColor::Black => self.black_castling = CastlingRights::none(),
      }
    }

    // If rook moves from starting square, lose that castling right
    if piece.piece_type == PieceType::Rook {
      match (piece.color, from) {
        (PieceColor::White, 0) => self.white_castling.queenside = false,
        (PieceColor::White, 7) => self.white_castling.kingside = false,
        (PieceColor::Black, 56) => self.black_castling.queenside = false,
        (PieceColor::Black, 63) => self.black_castling.kingside = false,
        _ => {}
      }
    }

    // If rook is captured on starting square, lose that castling right
    if captured.is_some() {
      match captured_square {
        0 => self.white_castling.queenside = false,
        7 => self.white_castling.kingside = false,
        56 => self.black_castling.queenside = false,
        63 => self.black_castling.kingside = false,
        _ => {}
      }
    }

    // Update halfmove clock
    if piece.piece_type == PieceType::Pawn || captured.is_some() {
      self.halfmove_clock = 0;
    } else {
      self.halfmove_clock = self.halfmove_clock.saturating_add(1);
    }

    // Update fullmove number (increment after black's move)
    if piece.color == PieceColor::Black {
      self.fullmove_number = self.fullmove_number.saturating_add(1);
    }

    // Save undo info
    if self.undo_count < MAX_UNDO_HISTORY {
      self.undo_history[self.undo_count] = Some(UndoInfo {
        from,
        to,
        captured,
        captured_square,
        prev_en_passant,
        prev_white_castling,
        prev_black_castling,
        prev_halfmove_clock,
        castling: castling_move,
        promotion: actual_promotion,
        original_piece: piece.piece_type,
      });
      self.undo_count += 1;
    }

    // Switch sides
    self.side_to_move = self.side_to_move.opponent();
  }

  /// Undo the last move (takeback)
  /// Returns true if a move was undone, false if no move to undo
  pub fn undo_move(&mut self) -> bool {
    if self.undo_count == 0 {
      return false;
    }

    self.undo_count -= 1;
    let undo = match self.undo_history[self.undo_count].take() {
      Some(u) => u,
      None => return false,
    };

    // Switch back side to move
    self.side_to_move = self.side_to_move.opponent();

    // Get the piece that was moved (it's now on the 'to' square)
    let mut piece = self.squares[undo.to as usize].take().unwrap();

    // If it was a promotion, revert to pawn
    if undo.promotion.is_some() {
      piece.piece_type = PieceType::Pawn;
    }

    // Put the piece back on the 'from' square
    self.squares[undo.from as usize] = Some(piece);

    // Restore captured piece (if any)
    if let Some(captured) = undo.captured {
      self.squares[undo.captured_square as usize] = Some(captured);
    }

    // If it was castling, move the rook back
    if let Some(kingside) = undo.castling {
      let color = piece.color;
      if kingside {
        // Kingside - rook went from h-file to f-file
        let rook_from = undo.to - 1; // f1 or f8
        let rook_to = if color == PieceColor::White { WHITE_ROOK_KINGSIDE } else { BLACK_ROOK_KINGSIDE };
        let rook = self.squares[rook_from as usize].take();
        self.squares[rook_to as usize] = rook;
      } else {
        // Queenside - rook went from a-file to d-file
        let rook_from = undo.to + 1; // d1 or d8
        let rook_to = if color == PieceColor::White { WHITE_ROOK_QUEENSIDE } else { BLACK_ROOK_QUEENSIDE };
        let rook = self.squares[rook_from as usize].take();
        self.squares[rook_to as usize] = rook;
      }
    }

    // Restore previous state
    self.en_passant = undo.prev_en_passant;
    self.white_castling = undo.prev_white_castling;
    self.black_castling = undo.prev_black_castling;
    self.halfmove_clock = undo.prev_halfmove_clock;

    // Update fullmove number if we're undoing black's move
    if self.side_to_move == PieceColor::Black {
      self.fullmove_number = self.fullmove_number.saturating_sub(1);
    }

    true
  }

  fn is_pseudo_legal(&self, from: u8, to: u8) -> bool {
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

  fn find_king(&self, color: PieceColor) -> Option<u8> {
    for sq in 0..64 {
      if let Some(piece) = self.squares[sq] {
        if piece.piece_type == PieceType::King && piece.color == color {
          return Some(sq as u8);
        }
      }
    }
    None
  }

  fn is_square_attacked(&self, sq: u8, by_color: PieceColor) -> bool {
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

  /// Get the current board status (checkmate, stalemate, draw conditions, or ongoing)
  pub fn status(&self) -> BoardStatus {
    // Check 50-move rule first (100 half-moves = 50 full moves)
    // But if checkmate is delivered on move 50, checkmate takes precedence
    if self.halfmove_clock >= 100 {
      // Still need to check for checkmate
      if !self.has_legal_moves() && self.is_in_check(self.side_to_move) {
        return BoardStatus::Checkmate;
      }
      return BoardStatus::FiftyMoveRule;
    }

    // Check insufficient material
    if self.is_insufficient_material() {
      return BoardStatus::InsufficientMaterial;
    }

    // Check for checkmate/stalemate
    if self.has_legal_moves() {
      BoardStatus::Ongoing
    } else if self.is_in_check(self.side_to_move) {
      BoardStatus::Checkmate
    } else {
      BoardStatus::Stalemate
    }
  }

  fn is_insufficient_material(&self) -> bool {
    let mut white_knights = 0u8;
    let mut white_bishops = 0u8;
    let mut white_bishop_sq_color = 0u8; // 0 = none, 1 = light, 2 = dark
    let mut black_knights = 0u8;
    let mut black_bishops = 0u8;
    let mut black_bishop_sq_color = 0u8;

    for sq in 0..64u8 {
      if let Some(piece) = self.get(sq) {
        match (piece.piece_type, piece.color) {
          // Any pawn, rook, or queen means sufficient material
          (PieceType::Pawn, _) | (PieceType::Rook, _) | (PieceType::Queen, _) => {
            return false;
          }
          (PieceType::Knight, PieceColor::White) => white_knights += 1,
          (PieceType::Knight, PieceColor::Black) => black_knights += 1,
          (PieceType::Bishop, PieceColor::White) => {
            white_bishops += 1;
            // Determine bishop square color: (file + rank) % 2
            let sq_color = if (sq % 8 + sq / 8) % 2 == 0 { 1 } else { 2 };
            white_bishop_sq_color = sq_color;
          }
          (PieceType::Bishop, PieceColor::Black) => {
            black_bishops += 1;
            let sq_color = if (sq % 8 + sq / 8) % 2 == 0 { 1 } else { 2 };
            black_bishop_sq_color = sq_color;
          }
          (PieceType::King, _) => {} // Kings are always present
        }
      }
    }

    let white_minor = white_knights + white_bishops;
    let black_minor = black_knights + black_bishops;

    // King vs King
    if white_minor == 0 && black_minor == 0 {
      return true;
    }

    // King + minor vs King
    if (white_minor == 1 && black_minor == 0) || (white_minor == 0 && black_minor == 1) {
      // K+N vs K or K+B vs K
      return true;
    }

    // King + Bishop vs King + Bishop (same color squares)
    if white_bishops == 1
      && black_bishops == 1
      && white_knights == 0
      && black_knights == 0
      && white_bishop_sq_color == black_bishop_sq_color
    {
      return true;
    }

    false
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

    // Captures (including en passant)
    for &cap_dir in &[dir - 1, dir + 1] {
      if let Some(to) = add_offset(from, cap_dir) {
        let to_file = to % 8;
        // Ensure we didn't wrap around files
        if (to_file as i8 - file as i8).abs() == 1 {
          // Regular capture
          if let Some(target) = self.get(to) {
            if target.color != color {
              dests.push(to);
            }
          }
          // En passant capture
          else if Some(to) == self.en_passant {
            dests.push(to);
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

    // Normal king moves (one square in any direction)
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

    // Castling moves
    let castling = self.castling_rights(color);
    let (king_start, rank) = match color {
      PieceColor::White => (WHITE_KING_START, 0u8),
      PieceColor::Black => (BLACK_KING_START, 7u8),
    };

    // Only if king is on starting square
    if from != king_start {
      return;
    }

    let opponent = color.opponent();

    // Kingside castling (O-O)
    if castling.kingside {
      let f_sq = rank * 8 + 5; // f1 or f8
      let g_sq = rank * 8 + 6; // g1 or g8

      // Squares between king and rook must be empty
      let path_clear = self.get(f_sq).is_none() && self.get(g_sq).is_none();

      // King must not be in check, and must not pass through or land on attacked squares
      let not_in_check = !self.is_square_attacked(king_start, opponent);
      let f_safe = !self.is_square_attacked(f_sq, opponent);
      let g_safe = !self.is_square_attacked(g_sq, opponent);

      if path_clear && not_in_check && f_safe && g_safe {
        dests.push(g_sq); // King lands on g1/g8
      }
    }

    // Queenside castling (O-O-O)
    if castling.queenside {
      let b_sq = rank * 8 + 1; // b1 or b8
      let c_sq = rank * 8 + 2; // c1 or c8
      let d_sq = rank * 8 + 3; // d1 or d8

      // Squares between king and rook must be empty (b, c, d)
      let path_clear = self.get(b_sq).is_none()
        && self.get(c_sq).is_none()
        && self.get(d_sq).is_none();

      // King must not be in check, and must not pass through or land on attacked squares
      // Note: b-square doesn't need to be safe (rook passes through it, not king)
      let not_in_check = !self.is_square_attacked(king_start, opponent);
      let c_safe = !self.is_square_attacked(c_sq, opponent);
      let d_safe = !self.is_square_attacked(d_sq, opponent);

      if path_clear && not_in_check && c_safe && d_safe {
        dests.push(c_sq); // King lands on c1/c8
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

    assert_eq!(board.side_to_move, PieceColor::White);
    board.make_move(12, 28); // e2-e4
    assert_eq!(board.side_to_move, PieceColor::Black);
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
    // White King on e1, Black Rook on a1, White Bishop on c1
    // Moving the bishop would expose king to check along the 1st rank
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White)); // e1
    board.squares[0] = Some(Piece::new(PieceType::Rook, PieceColor::Black)); // a1
    board.squares[2] = Some(Piece::new(PieceType::Bishop, PieceColor::White)); // c1

    // Bishop moving diagonally is pseudo-legal but would expose king to check
    assert!(board.is_pseudo_legal(2, 11)); // c1-d2
    assert!(!board.is_legal(2, 11)); // Can't move - would expose king to rook on a1
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
    // Corridor mate: Queen delivers mate with rook support
    // King on a1, pawns on a2 and b2, black queen on d1 gives check
    // Black rook on d8 prevents queen capture
    let mut board = ChessBoard::empty();
    board.squares[0] = Some(Piece::new(PieceType::King, PieceColor::White)); // a1
    board.squares[8] = Some(Piece::new(PieceType::Pawn, PieceColor::White)); // a2
    board.squares[9] = Some(Piece::new(PieceType::Pawn, PieceColor::White)); // b2
    board.squares[3] = Some(Piece::new(PieceType::Queen, PieceColor::Black)); // d1 - gives check
    board.squares[60] = Some(Piece::new(PieceType::King, PieceColor::Black)); // e8 - black king (needed for valid position)

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

  // ========== Tests for new features ==========

  #[test]
  fn test_castling_rights_at_start() {
    let board = ChessBoard::starting_position();
    let white_rights = board.castling_rights(PieceColor::White);
    let black_rights = board.castling_rights(PieceColor::Black);

    assert!(white_rights.kingside);
    assert!(white_rights.queenside);
    assert!(black_rights.kingside);
    assert!(black_rights.queenside);
  }

  #[test]
  fn test_castling_kingside() {
    let mut board = ChessBoard::starting_position();
    // Clear squares between king and rook
    board.squares[5] = None; // f1
    board.squares[6] = None; // g1

    // King should be able to castle kingside
    let dests = board.legal_destinations(4); // e1
    assert!(dests.contains(6)); // g1 - castling destination

    // Make the castling move
    board.make_move(4, 6);

    // King should be on g1, rook on f1
    assert_eq!(board.get(6), Some(Piece::new(PieceType::King, PieceColor::White)));
    assert_eq!(board.get(5), Some(Piece::new(PieceType::Rook, PieceColor::White)));
    assert_eq!(board.get(4), None); // e1 empty
    assert_eq!(board.get(7), None); // h1 empty
  }

  #[test]
  fn test_castling_queenside() {
    let mut board = ChessBoard::starting_position();
    // Clear squares between king and rook
    board.squares[1] = None; // b1
    board.squares[2] = None; // c1
    board.squares[3] = None; // d1

    // King should be able to castle queenside
    let dests = board.legal_destinations(4); // e1
    assert!(dests.contains(2)); // c1 - castling destination

    // Make the castling move
    board.make_move(4, 2);

    // King should be on c1, rook on d1
    assert_eq!(board.get(2), Some(Piece::new(PieceType::King, PieceColor::White)));
    assert_eq!(board.get(3), Some(Piece::new(PieceType::Rook, PieceColor::White)));
    assert_eq!(board.get(4), None); // e1 empty
    assert_eq!(board.get(0), None); // a1 empty
  }

  #[test]
  fn test_castling_blocked_by_check() {
    // Set up position where king is in check and can't castle
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White)); // e1
    board.squares[7] = Some(Piece::new(PieceType::Rook, PieceColor::White)); // h1
    board.squares[60] = Some(Piece::new(PieceType::Rook, PieceColor::Black)); // e8 attacks e1
    board.white_castling = CastlingRights::new(); // Can castle

    // King is in check
    assert!(board.is_in_check(PieceColor::White));

    // King can't castle while in check
    let dests = board.legal_destinations(4);
    assert!(!dests.contains(6)); // g1 - castling destination
  }

  #[test]
  fn test_castling_rights_lost_after_king_move() {
    let mut board = ChessBoard::starting_position();
    // Clear f1
    board.squares[5] = None;

    // Move king to f1
    board.make_move(4, 5);
    // Move king back (black moves first, let's just set up the position)
    board.side_to_move = PieceColor::White;
    board.make_move(5, 4);

    // Castling rights should be lost
    let rights = board.castling_rights(PieceColor::White);
    assert!(!rights.kingside);
    assert!(!rights.queenside);
  }

  #[test]
  fn test_en_passant_capture() {
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[60] = Some(Piece::new(PieceType::King, PieceColor::Black));
    // White pawn on e5
    board.squares[36] = Some(Piece::new(PieceType::Pawn, PieceColor::White));
    // Black pawn on d7
    board.squares[51] = Some(Piece::new(PieceType::Pawn, PieceColor::Black));
    board.side_to_move = PieceColor::Black;

    // Black plays d7-d5 (double pawn push)
    board.make_move(51, 35);

    // En passant should be available
    assert_eq!(board.en_passant, Some(43)); // d6

    // White pawn on e5 should be able to capture en passant
    let dests = board.legal_destinations(36);
    assert!(dests.contains(43)); // e5xd6 e.p.

    // Make the en passant capture
    board.make_move(36, 43);

    // White pawn should be on d6, black pawn on d5 should be gone
    assert_eq!(board.get(43), Some(Piece::new(PieceType::Pawn, PieceColor::White)));
    assert_eq!(board.get(35), None); // d5 - captured pawn removed
  }

  #[test]
  fn test_pawn_promotion() {
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[60] = Some(Piece::new(PieceType::King, PieceColor::Black));
    // White pawn on e7
    board.squares[52] = Some(Piece::new(PieceType::Pawn, PieceColor::White));

    // Move pawn to e8 (promotion)
    board.make_move(52, 60);

    // Pawn should have promoted to Queen (default)
    assert_eq!(board.get(60), Some(Piece::new(PieceType::Queen, PieceColor::White)));
  }

  #[test]
  fn test_pawn_promotion_to_knight() {
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[60] = Some(Piece::new(PieceType::King, PieceColor::Black));
    // White pawn on e7
    board.squares[52] = Some(Piece::new(PieceType::Pawn, PieceColor::White));

    // Move pawn to e8 with knight promotion
    board.make_move_with_promotion(52, 60, Some(PieceType::Knight));

    // Pawn should have promoted to Knight
    assert_eq!(board.get(60), Some(Piece::new(PieceType::Knight, PieceColor::White)));
  }

  #[test]
  fn test_undo_simple_move() {
    let mut board = ChessBoard::starting_position();

    // Make a move
    board.make_move(12, 28); // e2-e4

    assert_eq!(board.get(12), None);
    assert_eq!(board.get(28), Some(Piece::new(PieceType::Pawn, PieceColor::White)));
    assert_eq!(board.side_to_move, PieceColor::Black);

    assert!(board.undo_move());

    assert_eq!(board.get(12), Some(Piece::new(PieceType::Pawn, PieceColor::White)));
    assert_eq!(board.get(28), None);
    assert_eq!(board.side_to_move, PieceColor::White);
  }

  #[test]
  fn test_undo_capture() {
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[60] = Some(Piece::new(PieceType::King, PieceColor::Black));
    board.squares[28] = Some(Piece::new(PieceType::Pawn, PieceColor::White)); // e4
    board.squares[35] = Some(Piece::new(PieceType::Pawn, PieceColor::Black)); // d5

    // Capture
    board.make_move(28, 35); // exd5

    assert_eq!(board.get(35), Some(Piece::new(PieceType::Pawn, PieceColor::White)));
    assert_eq!(board.get(28), None);

    // Undo
    assert!(board.undo_move());

    assert_eq!(board.get(28), Some(Piece::new(PieceType::Pawn, PieceColor::White)));
    assert_eq!(board.get(35), Some(Piece::new(PieceType::Pawn, PieceColor::Black)));
  }

  #[test]
  fn test_undo_castling() {
    let mut board = ChessBoard::starting_position();
    board.squares[5] = None; // f1
    board.squares[6] = None; // g1

    // Castle kingside
    board.make_move(4, 6);

    assert_eq!(board.get(6), Some(Piece::new(PieceType::King, PieceColor::White)));
    assert_eq!(board.get(5), Some(Piece::new(PieceType::Rook, PieceColor::White)));

    // Undo
    assert!(board.undo_move());

    assert_eq!(board.get(4), Some(Piece::new(PieceType::King, PieceColor::White)));
    assert_eq!(board.get(7), Some(Piece::new(PieceType::Rook, PieceColor::White)));
    assert_eq!(board.get(5), None);
    assert_eq!(board.get(6), None);

    // Castling rights should be restored
    let rights = board.castling_rights(PieceColor::White);
    assert!(rights.kingside);
    assert!(rights.queenside);
  }

  #[test]
  fn test_undo_en_passant() {
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[60] = Some(Piece::new(PieceType::King, PieceColor::Black));
    board.squares[36] = Some(Piece::new(PieceType::Pawn, PieceColor::White)); // e5
    board.squares[51] = Some(Piece::new(PieceType::Pawn, PieceColor::Black)); // d7
    board.side_to_move = PieceColor::Black;

    // Black plays d7-d5
    board.make_move(51, 35);
    // White captures en passant
    board.make_move(36, 43);

    assert_eq!(board.get(43), Some(Piece::new(PieceType::Pawn, PieceColor::White)));
    assert_eq!(board.get(35), None);

    // Undo en passant
    assert!(board.undo_move());

    assert_eq!(board.get(36), Some(Piece::new(PieceType::Pawn, PieceColor::White)));
    assert_eq!(board.get(35), Some(Piece::new(PieceType::Pawn, PieceColor::Black)));
    assert_eq!(board.get(43), None);
  }

  #[test]
  fn test_undo_promotion() {
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White)); // e1
    board.squares[60] = Some(Piece::new(PieceType::King, PieceColor::Black)); // e8
    board.squares[55] = Some(Piece::new(PieceType::Pawn, PieceColor::White)); // h7

    // Promote (h7-h8)
    board.make_move(55, 63);

    assert_eq!(board.get(63), Some(Piece::new(PieceType::Queen, PieceColor::White)));
    assert_eq!(board.get(55), None);

    // Undo
    assert!(board.undo_move());

    assert_eq!(board.get(55), Some(Piece::new(PieceType::Pawn, PieceColor::White)));
    assert_eq!(board.get(63), None);
  }

  #[test]
  fn test_halfmove_clock() {
    let mut board = ChessBoard::starting_position();

    assert_eq!(board.halfmove_clock, 0);

    board.make_move(12, 28); // e2-e4
    assert_eq!(board.halfmove_clock, 0);

    board.make_move(57, 42); // Nb8-c6
    assert_eq!(board.halfmove_clock, 1);

    board.make_move(6, 21); // Ng1-f3
    assert_eq!(board.halfmove_clock, 2);

    board.make_move(52, 36); // e7-e5
    assert_eq!(board.halfmove_clock, 0);
  }

  #[test]
  fn test_insufficient_material_king_vs_king() {
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[60] = Some(Piece::new(PieceType::King, PieceColor::Black));

    assert!(board.is_insufficient_material());
    assert_eq!(board.status(), BoardStatus::InsufficientMaterial);
  }

  #[test]
  fn test_insufficient_material_king_bishop_vs_king() {
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[60] = Some(Piece::new(PieceType::King, PieceColor::Black));
    board.squares[20] = Some(Piece::new(PieceType::Bishop, PieceColor::White));

    assert!(board.is_insufficient_material());
  }

  #[test]
  fn test_insufficient_material_king_knight_vs_king() {
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[60] = Some(Piece::new(PieceType::King, PieceColor::Black));
    board.squares[20] = Some(Piece::new(PieceType::Knight, PieceColor::White));

    assert!(board.is_insufficient_material());
  }

  #[test]
  fn test_sufficient_material_king_rook_vs_king() {
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[60] = Some(Piece::new(PieceType::King, PieceColor::Black));
    board.squares[0] = Some(Piece::new(PieceType::Rook, PieceColor::White));

    assert!(!board.is_insufficient_material());
  }

  #[test]
  fn test_sufficient_material_with_pawn() {
    let mut board = ChessBoard::empty();
    board.squares[4] = Some(Piece::new(PieceType::King, PieceColor::White));
    board.squares[60] = Some(Piece::new(PieceType::King, PieceColor::Black));
    board.squares[12] = Some(Piece::new(PieceType::Pawn, PieceColor::White));

    assert!(!board.is_insufficient_material());
  }
}
