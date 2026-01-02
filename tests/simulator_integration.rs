#![cfg(feature = "simulator")]

use certabo::certabo::board::CertaboBoard;
use certabo::certabo::buffer::MAX_LINE_LEN;
use certabo::certabo::calibration::{CalibrationData, Piece};
use certabo::certabo::protocol::RfidReading;
use certabo::certabo::simulator::{SimulatedBoard, VirtualBoard};
use certabo::game::state::{GameState, GameStatus};

fn parse_board_reading(board: &mut SimulatedBoard) -> RfidReading {
    board.poll();
    let mut buf = [0u8; MAX_LINE_LEN];
    let len = board.read_line(&mut buf).expect("should have line");
    let trimmed = &buf[1..len - 1];
    RfidReading::parse(trimmed).expect("should parse")
}

#[test]
fn test_full_calibration_flow() {
    let mut board = SimulatedBoard::with_starting_position();

    let reading = parse_board_reading(&mut board);

    let mut calibration = CalibrationData::new();
    let count = calibration.calibrate_from_starting_position(&reading);

    assert_eq!(count, 32);
    assert!(calibration.is_complete());

    let board_state = calibration.reading_to_board(&reading);
    assert_eq!(board_state[0], Some(Piece::WhiteRook));
    assert_eq!(board_state[4], Some(Piece::WhiteKing));
    assert_eq!(board_state[60], Some(Piece::BlackKing));
}

#[test]
fn test_e2_e4_opening() {
    let mut board = SimulatedBoard::with_starting_position();

    let initial_reading = parse_board_reading(&mut board);
    let mut calibration = CalibrationData::new();
    calibration.calibrate_from_starting_position(&initial_reading);

    board.virtual_board_mut().move_piece(12, 28);

    let after_move = parse_board_reading(&mut board);
    let board_state = calibration.reading_to_board(&after_move);

    assert_eq!(board_state[12], None);
    assert_eq!(board_state[28], Some(Piece::WhitePawn));
}

#[test]
fn test_capture_detection() {
    let mut board = SimulatedBoard::with_starting_position();

    let reading = parse_board_reading(&mut board);
    let mut calibration = CalibrationData::new();
    calibration.calibrate_from_starting_position(&reading);

    board.virtual_board_mut().move_piece(12, 28);
    board.virtual_board_mut().move_piece(51, 35);
    board.virtual_board_mut().move_piece(28, 35);

    let after_capture = parse_board_reading(&mut board);
    let board_state = calibration.reading_to_board(&after_capture);

    assert_eq!(board_state[12], None);
    assert_eq!(board_state[28], None);
    assert_eq!(board_state[51], None);
    assert_eq!(board_state[35], Some(Piece::WhitePawn));
}

#[test]
fn test_led_feedback() {
    let mut board = SimulatedBoard::new();

    let leds: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x10, 0x00];
    board.send_leds(&leds).unwrap();

    assert!(board.led_state().is_on(12));
    assert!(board.led_state().is_on(28));
    assert!(!board.led_state().is_on(0));
}

#[test]
fn test_multiple_readings() {
    let mut board = SimulatedBoard::with_starting_position();
    board.set_auto_generate(false);

    assert!(!board.has_line());

    board.generate_reading();
    assert!(board.has_line());

    let mut buf = [0u8; MAX_LINE_LEN];
    let _ = board.read_line(&mut buf);
    assert!(!board.has_line());

    board.generate_reading();
    assert!(board.has_line());
}

#[test]
fn test_custom_position() {
    let mut virtual_board = VirtualBoard::new();
    virtual_board.place_piece(4, Piece::WhiteKing);
    virtual_board.place_piece(60, Piece::BlackKing);
    virtual_board.place_piece(28, Piece::WhiteQueen);

    let mut board = SimulatedBoard::new();
    *board.virtual_board_mut() = virtual_board;

    let reading = parse_board_reading(&mut board);

    assert!(reading.has_piece(4));
    assert!(reading.has_piece(60));
    assert!(reading.has_piece(28));
    assert!(!reading.has_piece(0));
}

#[test]
fn test_game_state_with_simulator() {
    let mut game = GameState::new();
    game.set_status(GameStatus::InProgress);

    let mut board = SimulatedBoard::with_starting_position();
    let reading = parse_board_reading(&mut board);
    let mut calibration = CalibrationData::new();
    calibration.calibrate_from_starting_position(&reading);

    board.virtual_board_mut().move_piece(12, 28);
    let after_e4 = parse_board_reading(&mut board);
    let board_state = calibration.reading_to_board(&after_e4);

    let mut pieces_found = 0;
    for piece in board_state.iter() {
        if piece.is_some() {
            pieces_found += 1;
        }
    }
    assert_eq!(pieces_found, 32);
}

#[test]
fn test_castling_setup() {
    let mut virtual_board = VirtualBoard::new();

    virtual_board.place_piece(4, Piece::WhiteKing);
    virtual_board.place_piece(7, Piece::WhiteRook);
    virtual_board.place_piece(0, Piece::WhiteRook);
    virtual_board.place_piece(60, Piece::BlackKing);

    let mut board = SimulatedBoard::new();
    *board.virtual_board_mut() = virtual_board;

    let reading = parse_board_reading(&mut board);

    assert!(reading.has_piece(4));
    assert!(reading.has_piece(7));
    assert!(reading.has_piece(0));
    assert!(reading.has_piece(60));
    assert!(!reading.has_piece(5));
    assert!(!reading.has_piece(6));
}

#[test]
fn test_promotion_scenario() {
    let mut virtual_board = VirtualBoard::new();

    virtual_board.place_piece(4, Piece::WhiteKing);
    virtual_board.place_piece(60, Piece::BlackKing);
    virtual_board.place_piece(48, Piece::WhitePawn);

    let mut board = SimulatedBoard::new();
    *board.virtual_board_mut() = virtual_board;

    let before = parse_board_reading(&mut board);
    assert!(before.has_piece(48));

    board.virtual_board_mut().remove_piece(48);
    board.virtual_board_mut().place_piece(56, Piece::WhiteQueen);

    let after = parse_board_reading(&mut board);
    assert!(!after.has_piece(48));
    assert!(after.has_piece(56));
}
