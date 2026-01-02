pub mod virtual_board;

use crate::certabo::board::{BoardError, CertaboBoard};
use crate::certabo::buffer::MAX_LINE_LEN;
use crate::certabo::leds::LedState;
use crate::certabo::protocol::{write_ascii_number, RfidReading, RFID_BYTES, TOTAL_VALUES};
pub use virtual_board::{SimPiece, VirtualBoard};

pub struct SimulatedBoard {
    virtual_board: VirtualBoard,
    output_buffer: [u8; MAX_LINE_LEN],
    output_len: usize,
    output_pos: usize,
    led_state: LedState,
    auto_generate: bool,
}

impl Default for SimulatedBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulatedBoard {
    pub fn new() -> Self {
        Self {
            virtual_board: VirtualBoard::new(),
            output_buffer: [0; MAX_LINE_LEN],
            output_len: 0,
            output_pos: 0,
            led_state: LedState::new(),
            auto_generate: true,
        }
    }

    pub fn with_starting_position() -> Self {
        Self {
            virtual_board: VirtualBoard::starting_position(),
            output_buffer: [0; MAX_LINE_LEN],
            output_len: 0,
            output_pos: 0,
            led_state: LedState::new(),
            auto_generate: true,
        }
    }

    pub fn virtual_board(&self) -> &VirtualBoard {
        &self.virtual_board
    }

    pub fn virtual_board_mut(&mut self) -> &mut VirtualBoard {
        &mut self.virtual_board
    }

    pub fn led_state(&self) -> &LedState {
        &self.led_state
    }

    pub fn set_auto_generate(&mut self, auto: bool) {
        self.auto_generate = auto;
    }

    pub fn generate_reading(&mut self) {
        let reading = self.virtual_board.to_rfid_reading();
        self.encode_reading(&reading);
    }

    pub fn inject_reading(&mut self, reading: &RfidReading) {
        self.encode_reading(reading);
    }

    fn encode_reading(&mut self, reading: &RfidReading) {
        self.output_pos = 0;
        self.output_len = 0;

        self.output_buffer[self.output_len] = b':';
        self.output_len += 1;

        for i in 0..TOTAL_VALUES {
            let square_idx = i / RFID_BYTES;
            let byte_idx = i % RFID_BYTES;
            let value = reading.chip_ids[square_idx][byte_idx];

            if i > 0 {
                self.output_buffer[self.output_len] = b' ';
                self.output_len += 1;
            }

            self.write_number(value);
        }

        self.output_buffer[self.output_len] = b'\r';
        self.output_len += 1;
        self.output_buffer[self.output_len] = b'\n';
        self.output_len += 1;
    }

    fn write_number(&mut self, value: u8) {
        write_ascii_number(&mut self.output_buffer, &mut self.output_len, value);
    }

    fn has_pending_output(&self) -> bool {
        self.output_pos < self.output_len
    }

    fn consume_output(&mut self, out: &mut [u8]) -> Option<usize> {
        if !self.has_pending_output() {
            return None;
        }

        let newline_pos = self.output_buffer[self.output_pos..self.output_len]
            .iter()
            .position(|&b| b == b'\n');

        let Some(rel_pos) = newline_pos else {
            return None;
        };

        let end_pos = self.output_pos + rel_pos + 1;
        let len = end_pos - self.output_pos;

        if len > out.len() {
            return None;
        }

        out[..len].copy_from_slice(&self.output_buffer[self.output_pos..end_pos]);
        self.output_pos = end_pos;

        if self.output_pos >= self.output_len {
            self.output_pos = 0;
            self.output_len = 0;
        }

        Some(len)
    }
}

impl CertaboBoard for SimulatedBoard {
    fn poll(&mut self) {
        if self.auto_generate && !self.has_pending_output() {
            self.generate_reading();
        }
    }

    fn has_line(&self) -> bool {
        self.output_buffer[self.output_pos..self.output_len]
            .iter()
            .any(|&b| b == b'\n')
    }

    fn read_line(&mut self, out: &mut [u8]) -> Option<usize> {
        self.consume_output(out)
    }

    fn send_leds(&mut self, data: &[u8; 8]) -> Result<(), BoardError> {
        self.led_state = LedState::from_bytes(data);
        Ok(())
    }

    fn clear_buffer(&mut self) {
        self.output_pos = 0;
        self.output_len = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::certabo::calibration::Piece;

    #[test]
    fn test_simulated_board_new() {
        let board = SimulatedBoard::new();
        assert!(!board.has_line());
    }

    #[test]
    fn test_generate_reading_empty_board() {
        let mut board = SimulatedBoard::new();
        board.generate_reading();

        assert!(board.has_line());

        let mut buf = [0u8; 2048];
        let len = board.read_line(&mut buf).unwrap();

        assert!(buf[0] == b':');
        assert!(buf[len - 1] == b'\n');
        assert!(buf[len - 2] == b'\r');
    }

    #[test]
    fn test_generate_reading_starting_position() {
        let mut board = SimulatedBoard::with_starting_position();
        board.generate_reading();

        let mut buf = [0u8; 2048];
        let len = board.read_line(&mut buf).unwrap();

        let trimmed = &buf[1..len - 1];
        let reading = RfidReading::parse(trimmed);
        assert!(reading.is_some());

        let reading = reading.unwrap();

        assert!(reading.has_piece(0));
        assert!(reading.has_piece(4));
        assert!(reading.has_piece(60));
        assert!(!reading.has_piece(28));
    }

    #[test]
    fn test_led_state_capture() {
        let mut board = SimulatedBoard::new();

        let leds: [u8; 8] = [0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF];
        board.send_leds(&leds).unwrap();

        assert!(board.led_state().is_on(0));
        assert!(board.led_state().is_on(63));
        assert!(!board.led_state().is_on(28));
    }

    #[test]
    fn test_poll_auto_generates() {
        let mut board = SimulatedBoard::with_starting_position();

        assert!(!board.has_line());

        board.poll();

        assert!(board.has_line());
    }

    #[test]
    fn test_move_piece_and_read() {
        let mut board = SimulatedBoard::with_starting_position();

        board.virtual_board_mut().move_piece(12, 28);
        board.generate_reading();

        let mut buf = [0u8; 2048];
        let len = board.read_line(&mut buf).unwrap();

        let trimmed = &buf[1..len - 1];
        let reading = RfidReading::parse(trimmed).unwrap();

        assert!(!reading.has_piece(12));
        assert!(reading.has_piece(28));
    }

    #[test]
    fn test_clear_buffer() {
        let mut board = SimulatedBoard::with_starting_position();
        board.generate_reading();

        assert!(board.has_line());

        board.clear_buffer();

        assert!(!board.has_line());
    }

    #[test]
    fn test_inject_custom_reading() {
        let mut board = SimulatedBoard::new();

        let mut reading = RfidReading::new();
        reading.chip_ids[0] = [0xDE, 0xAD, 0xBE, 0xEF, 0x00];

        board.inject_reading(&reading);

        assert!(board.has_line());

        let mut buf = [0u8; 2048];
        let len = board.read_line(&mut buf).unwrap();
        let trimmed = &buf[1..len - 1];
        let parsed = RfidReading::parse(trimmed).unwrap();

        assert_eq!(parsed.chip_id(0), [0xDE, 0xAD, 0xBE, 0xEF, 0x00]);
    }

    #[test]
    fn test_calibration_compatible() {
        use crate::certabo::calibration::CalibrationData;

        let mut board = SimulatedBoard::with_starting_position();
        board.generate_reading();

        let mut buf = [0u8; 2048];
        let len = board.read_line(&mut buf).unwrap();
        let trimmed = &buf[1..len - 1];
        let reading = RfidReading::parse(trimmed).unwrap();

        let mut calibration = CalibrationData::new();
        let count = calibration.calibrate_from_starting_position(&reading);

        assert_eq!(count, 32);
        assert!(calibration.is_complete());

        let board_state = calibration.reading_to_board(&reading);
        assert_eq!(board_state[0], Some(Piece::WhiteRook));
        assert_eq!(board_state[4], Some(Piece::WhiteKing));
        assert_eq!(board_state[60], Some(Piece::BlackKing));
        assert_eq!(board_state[28], None);
    }
}
