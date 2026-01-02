use crate::certabo::calibration::{Piece, STARTING_LAYOUT};
use crate::certabo::protocol::{RfidReading, RFID_BYTES};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SimPiece {
    pub piece: Piece,
    pub chip_id: [u8; RFID_BYTES],
}

impl SimPiece {
    pub fn new(piece: Piece, chip_id: [u8; RFID_BYTES]) -> Self {
        Self { piece, chip_id }
    }
}

pub struct VirtualBoard {
    squares: [Option<SimPiece>; 64],
}

impl Default for VirtualBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualBoard {
    pub fn new() -> Self {
        Self {
            squares: [None; 64],
        }
    }

    pub fn starting_position() -> Self {
        let mut board = Self::new();

        for (square, piece) in STARTING_LAYOUT {
            let chip_id = Self::generate_chip_id(square, piece);
            board.squares[square as usize] = Some(SimPiece::new(piece, chip_id));
        }

        board
    }

    fn generate_chip_id(square: u8, piece: Piece) -> [u8; RFID_BYTES] {
        let piece_byte = match piece {
            Piece::WhitePawn => 0x01,
            Piece::WhiteKnight => 0x02,
            Piece::WhiteBishop => 0x03,
            Piece::WhiteRook => 0x04,
            Piece::WhiteQueen => 0x05,
            Piece::WhiteKing => 0x06,
            Piece::BlackPawn => 0x11,
            Piece::BlackKnight => 0x12,
            Piece::BlackBishop => 0x13,
            Piece::BlackRook => 0x14,
            Piece::BlackQueen => 0x15,
            Piece::BlackKing => 0x16,
        };

        [0xCE, 0x47, piece_byte, square, 0xAB]
    }

    pub fn get(&self, square: u8) -> Option<&SimPiece> {
        self.squares.get(square as usize).and_then(|s| s.as_ref())
    }

    pub fn set(&mut self, square: u8, piece: Option<SimPiece>) {
        if let Some(slot) = self.squares.get_mut(square as usize) {
            *slot = piece;
        }
    }

    pub fn place_piece(&mut self, square: u8, piece: Piece) {
        let chip_id = Self::generate_chip_id(square, piece);
        self.set(square, Some(SimPiece::new(piece, chip_id)));
    }

    pub fn place_piece_with_chip_id(&mut self, square: u8, piece: Piece, chip_id: [u8; RFID_BYTES]) {
        self.set(square, Some(SimPiece::new(piece, chip_id)));
    }

    pub fn remove_piece(&mut self, square: u8) {
        self.set(square, None);
    }

    pub fn move_piece(&mut self, from: u8, to: u8) -> bool {
        let piece = self.squares.get(from as usize).copied().flatten();
        if piece.is_none() {
            return false;
        }

        self.squares[from as usize] = None;
        self.squares[to as usize] = piece;
        true
    }

    pub fn clear(&mut self) {
        self.squares = [None; 64];
    }

    pub fn to_rfid_reading(&self) -> RfidReading {
        let mut reading = RfidReading::new();

        for (square, slot) in self.squares.iter().enumerate() {
            if let Some(sim_piece) = slot {
                reading.chip_ids[square] = sim_piece.chip_id;
            }
        }

        reading
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_board() {
        let board = VirtualBoard::new();
        for square in 0..64 {
            assert!(board.get(square).is_none());
        }
    }

    #[test]
    fn test_starting_position() {
        let board = VirtualBoard::starting_position();

        assert!(board.get(0).is_some());
        assert_eq!(board.get(0).unwrap().piece, Piece::WhiteRook);
        assert!(board.get(4).is_some());
        assert_eq!(board.get(4).unwrap().piece, Piece::WhiteKing);
        assert!(board.get(60).is_some());
        assert_eq!(board.get(60).unwrap().piece, Piece::BlackKing);
        assert!(board.get(28).is_none());
    }

    #[test]
    fn test_move_piece() {
        let mut board = VirtualBoard::starting_position();

        let e2_piece = board.get(12).cloned();
        assert!(e2_piece.is_some());

        assert!(board.move_piece(12, 28));

        assert!(board.get(12).is_none());
        assert!(board.get(28).is_some());
        assert_eq!(board.get(28).unwrap().piece, Piece::WhitePawn);
    }

    #[test]
    fn test_to_rfid_reading() {
        let board = VirtualBoard::starting_position();
        let reading = board.to_rfid_reading();

        assert!(reading.has_piece(0));
        assert!(reading.has_piece(4));
        assert!(reading.has_piece(60));
        assert!(!reading.has_piece(28));

        assert_ne!(reading.chip_id(0), [0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_place_and_remove() {
        let mut board = VirtualBoard::new();

        board.place_piece(28, Piece::WhitePawn);
        assert!(board.get(28).is_some());
        assert_eq!(board.get(28).unwrap().piece, Piece::WhitePawn);

        board.remove_piece(28);
        assert!(board.get(28).is_none());
    }
}
