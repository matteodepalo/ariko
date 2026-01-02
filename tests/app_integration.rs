#![cfg(feature = "simulator")]

use certabo::app::{
    App, AppState, BoardIO, BuzzerSound, DisplayMessage, Hardware, MockBuzzer, MockClockDisplay,
    MockDelay, MockDisplay,
};
use certabo::certabo::buffer::MAX_LINE_LEN;
use certabo::certabo::calibration::Piece;
use certabo::certabo::protocol::RfidReading;
use certabo::certabo::simulator::SimulatedBoard;

fn create_test_hardware() -> Hardware<SimulatedBoard, MockDisplay, MockBuzzer, MockClockDisplay, MockDelay>
{
    Hardware {
        board: SimulatedBoard::with_starting_position(),
        display: MockDisplay::new(),
        buzzer: MockBuzzer::new(),
        clock: MockClockDisplay::new(),
        delay: MockDelay::new(),
    }
}

fn poll_reading(board: &mut SimulatedBoard) -> RfidReading {
    board.poll();
    let mut buf = [0u8; MAX_LINE_LEN];
    let len = board.read_line(&mut buf).expect("should have line");
    let trimmed = &buf[1..len - 1];
    RfidReading::parse(trimmed).expect("should parse")
}

struct TestGame {
    app: App,
    hw: Hardware<SimulatedBoard, MockDisplay, MockBuzzer, MockClockDisplay, MockDelay>,
}

impl TestGame {
    fn new() -> Self {
        let mut app = App::new();
        let mut hw = create_test_hardware();

        app.start(&mut hw);
        app.on_blue_button(&mut hw.display);

        let reading = poll_reading(&mut hw.board);
        app.on_board_reading(reading, &mut hw.display, &mut hw.buzzer, &mut hw.delay);

        let reading = poll_reading(&mut hw.board);
        app.on_board_reading(reading, &mut hw.display, &mut hw.buzzer, &mut hw.delay);

        assert_eq!(app.state(), AppState::GameInProgress);

        Self { app, hw }
    }

    fn make_move(&mut self, from: u8, to: u8) {
        let piece = self.hw.board.virtual_board().get(from).cloned().unwrap();
        self.hw.board.virtual_board_mut().remove_piece(from);
        let reading = poll_reading(&mut self.hw.board);
        self.app
            .on_board_reading(reading, &mut self.hw.display, &mut self.hw.buzzer, &mut self.hw.delay);

        self.hw
            .board
            .virtual_board_mut()
            .place_piece_with_chip_id(to, piece.piece, piece.chip_id);
        let reading = poll_reading(&mut self.hw.board);
        self.app
            .on_board_reading(reading, &mut self.hw.display, &mut self.hw.buzzer, &mut self.hw.delay);
    }

    fn make_capture(&mut self, from: u8, to: u8) {
        let piece = self.hw.board.virtual_board().get(from).cloned().unwrap();
        self.hw.board.virtual_board_mut().remove_piece(from);
        let reading = poll_reading(&mut self.hw.board);
        self.app
            .on_board_reading(reading, &mut self.hw.display, &mut self.hw.buzzer, &mut self.hw.delay);

        self.hw.board.virtual_board_mut().remove_piece(to);
        self.hw
            .board
            .virtual_board_mut()
            .place_piece_with_chip_id(to, piece.piece, piece.chip_id);
        let reading = poll_reading(&mut self.hw.board);
        self.app
            .on_board_reading(reading, &mut self.hw.display, &mut self.hw.buzzer, &mut self.hw.delay);
    }

    fn make_castling(&mut self, king_from: u8, king_to: u8, rook_from: u8, rook_to: u8) {
        let king = self.hw.board.virtual_board().get(king_from).cloned().unwrap();
        let rook = self.hw.board.virtual_board().get(rook_from).cloned().unwrap();

        self.hw.board.virtual_board_mut().remove_piece(king_from);
        let reading = poll_reading(&mut self.hw.board);
        self.app
            .on_board_reading(reading, &mut self.hw.display, &mut self.hw.buzzer, &mut self.hw.delay);

        self.hw
            .board
            .virtual_board_mut()
            .place_piece_with_chip_id(king_to, king.piece, king.chip_id);
        let reading = poll_reading(&mut self.hw.board);
        self.app
            .on_board_reading(reading, &mut self.hw.display, &mut self.hw.buzzer, &mut self.hw.delay);

        self.hw.board.virtual_board_mut().remove_piece(rook_from);
        let reading = poll_reading(&mut self.hw.board);
        self.app
            .on_board_reading(reading, &mut self.hw.display, &mut self.hw.buzzer, &mut self.hw.delay);

        self.hw
            .board
            .virtual_board_mut()
            .place_piece_with_chip_id(rook_to, rook.piece, rook.chip_id);
        let reading = poll_reading(&mut self.hw.board);
        self.app
            .on_board_reading(reading, &mut self.hw.display, &mut self.hw.buzzer, &mut self.hw.delay);
    }

    fn lift_piece(&mut self, square: u8) {
        self.hw.board.virtual_board_mut().remove_piece(square);
        let reading = poll_reading(&mut self.hw.board);
        self.app
            .on_board_reading(reading, &mut self.hw.display, &mut self.hw.buzzer, &mut self.hw.delay);
    }

    fn place_piece(&mut self, square: u8, piece: Piece, chip_id: [u8; 5]) {
        self.hw
            .board
            .virtual_board_mut()
            .place_piece_with_chip_id(square, piece, chip_id);
        let reading = poll_reading(&mut self.hw.board);
        self.app
            .on_board_reading(reading, &mut self.hw.display, &mut self.hw.buzzer, &mut self.hw.delay);
    }

    fn tick(&mut self, ms: u32) {
        self.app.tick(ms, &mut self.hw.display, &mut self.hw.buzzer, &mut self.hw.clock);
    }
}

#[test]
fn test_app_initialization() {
    let app = App::new();
    assert_eq!(app.state(), AppState::Initializing);
}

#[test]
fn test_app_start() {
    let mut app = App::new();
    let mut hw = create_test_hardware();

    app.start(&mut hw);

    assert_eq!(app.state(), AppState::WaitingForCalibration);
    assert_eq!(
        hw.display.last_message,
        Some(DisplayMessage::CalibrationPrompt)
    );
}

#[test]
fn test_blue_button_starts_calibration() {
    let mut app = App::new();
    let mut hw = create_test_hardware();

    app.start(&mut hw);
    app.on_blue_button(&mut hw.display);

    assert_eq!(app.state(), AppState::Calibrating);
}

#[test]
fn test_calibration_flow() {
    let mut app = App::new();
    let mut hw = create_test_hardware();

    app.start(&mut hw);
    app.on_blue_button(&mut hw.display);

    let reading = poll_reading(&mut hw.board);
    app.on_board_reading(reading, &mut hw.display, &mut hw.buzzer, &mut hw.delay);

    assert_eq!(app.state(), AppState::WaitingForSetup);
    assert!(hw
        .display
        .messages
        .contains(&DisplayMessage::CalibrationComplete));
    assert!(hw.buzzer.sounds.contains(&BuzzerSound::CalibrationComplete));
}

#[test]
fn test_game_starts_when_pieces_detected() {
    let mut app = App::new();
    let mut hw = create_test_hardware();

    app.start(&mut hw);
    app.on_blue_button(&mut hw.display);

    let reading = poll_reading(&mut hw.board);
    app.on_board_reading(reading, &mut hw.display, &mut hw.buzzer, &mut hw.delay);

    let reading = poll_reading(&mut hw.board);
    app.on_board_reading(reading, &mut hw.display, &mut hw.buzzer, &mut hw.delay);

    assert_eq!(app.state(), AppState::GameInProgress);
}

#[test]
fn test_move_detection_e2_e4() {
    let mut game = TestGame::new();

    game.hw.buzzer.sounds.clear();
    game.make_move(12, 28);

    assert!(game.hw.buzzer.sounds.contains(&BuzzerSound::Move));
}

#[test]
fn test_pause_resume() {
    let mut game = TestGame::new();

    game.app.on_white_button(&mut game.hw.display);
    assert_eq!(game.app.state(), AppState::GamePaused);
    assert!(game.hw.display.messages.contains(&DisplayMessage::Paused));

    game.app.on_white_button(&mut game.hw.display);
    assert_eq!(game.app.state(), AppState::GameInProgress);
}

#[test]
fn test_led_animation_during_calibration() {
    let mut app = App::new();
    let mut hw = create_test_hardware();

    app.start(&mut hw);

    for _ in 0..10 {
        app.tick(100, &mut hw.display, &mut hw.buzzer, &mut hw.clock);
    }

    assert!(app.should_send_leds());
}

#[test]
fn test_clock_updates_during_game() {
    let mut game = TestGame::new();

    game.tick(100);

    assert_eq!(game.hw.clock.white_time, (10, 0));
    assert!(game.hw.clock.white_active);
    assert!(!game.hw.clock.black_active);
}

#[test]
fn test_full_game_scenario() {
    let mut game = TestGame::new();

    game.make_move(12, 28);

    assert!(game
        .hw
        .display
        .messages
        .iter()
        .any(|m| matches!(m, DisplayMessage::Turn { is_white: false })));

    game.app.on_white_button(&mut game.hw.display);
    assert_eq!(game.app.state(), AppState::GamePaused);
}

#[test]
fn test_black_piece_leds_after_white_move() {
    let mut game = TestGame::new();

    game.make_move(12, 28);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Move),
        "White's e2-e4 move should complete"
    );
    assert!(
        !game.app.led_state().is_on(12),
        "e2 LED should be off after move completes"
    );
    assert!(
        !game.app.led_state().is_on(28),
        "e4 LED should be off after move completes"
    );

    let e7_piece = game.hw.board.virtual_board().get(52).cloned().unwrap();
    game.lift_piece(52);

    assert!(
        game.app.led_state().is_on(52),
        "e7 LED should be on (lifted piece)"
    );
    assert!(
        game.app.led_state().is_on(44),
        "e6 LED should be on (legal destination)"
    );
    assert!(
        game.app.led_state().is_on(36),
        "e5 LED should be on (legal destination - 2 square initial move)"
    );

    assert!(
        !game.app.led_state().is_on(12),
        "e2 LED should NOT be on when Black lifts e7"
    );
    assert!(
        !game.app.led_state().is_on(28),
        "e4 LED should NOT be on when Black lifts e7"
    );

    game.place_piece(36, e7_piece.piece, e7_piece.chip_id);

    assert!(
        game.hw
            .display
            .messages
            .iter()
            .any(|m| matches!(m, DisplayMessage::Turn { is_white: true })),
        "After Black's move, it should be White's turn"
    );
}

#[test]
fn test_white_kingside_castling() {
    let mut game = TestGame::new();

    game.make_move(12, 28);
    game.make_move(52, 36);
    game.make_move(6, 21);
    game.make_move(57, 42);
    game.make_move(5, 12);
    game.make_move(62, 45);

    game.hw.buzzer.sounds.clear();
    game.make_castling(4, 6, 7, 5);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Move),
        "Kingside castling should be valid"
    );
    assert!(
        game.hw
            .display
            .messages
            .iter()
            .any(|m| matches!(m, DisplayMessage::Turn { is_white: false })),
        "Turn should switch to Black after castling"
    );
}

#[test]
fn test_white_queenside_castling() {
    let mut game = TestGame::new();

    game.make_move(11, 27);
    game.make_move(52, 36);
    game.make_move(3, 19);
    game.make_move(57, 42);
    game.make_move(2, 29);
    game.make_move(62, 45);
    game.make_move(1, 18);
    game.make_move(61, 52);

    game.hw.buzzer.sounds.clear();
    game.make_castling(4, 2, 0, 3);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Move),
        "Queenside castling should be valid"
    );
}

#[test]
fn test_black_kingside_castling() {
    let mut game = TestGame::new();

    game.make_move(12, 28);
    game.make_move(52, 36);
    game.make_move(6, 21);
    game.make_move(62, 45);
    game.make_move(5, 12);
    game.make_move(61, 52);
    game.make_move(4, 5);

    game.hw.buzzer.sounds.clear();
    game.make_castling(60, 62, 63, 61);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Move),
        "Black kingside castling should be valid"
    );
}

#[test]
fn test_en_passant_capture() {
    let mut game = TestGame::new();

    game.make_move(12, 28);
    game.make_move(48, 40);
    game.make_move(28, 36);
    game.make_move(51, 35);

    let pawn = game.hw.board.virtual_board().get(36).cloned().unwrap();
    game.hw.board.virtual_board_mut().remove_piece(36);
    game.hw.board.virtual_board_mut().remove_piece(35);
    let reading = poll_reading(&mut game.hw.board);
    game.app
        .on_board_reading(reading, &mut game.hw.display, &mut game.hw.buzzer, &mut game.hw.delay);

    game.hw
        .board
        .virtual_board_mut()
        .place_piece_with_chip_id(43, pawn.piece, pawn.chip_id);
    let reading = poll_reading(&mut game.hw.board);
    game.app
        .on_board_reading(reading, &mut game.hw.display, &mut game.hw.buzzer, &mut game.hw.delay);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Move),
        "En passant capture should be valid"
    );
}

#[test]
fn test_pawn_promotion_to_queen() {
    let mut game = TestGame::new();

    let white_queen_chip = game.hw.board.virtual_board().get(3).cloned().unwrap().chip_id;

    game.make_move(15, 31);
    game.make_move(54, 38);
    game.make_capture(31, 38);
    game.make_move(62, 45);
    game.make_move(38, 46);
    game.make_move(45, 28);
    game.make_move(46, 54);
    game.make_move(28, 43);

    game.hw.buzzer.sounds.clear();

    game.lift_piece(54);
    game.hw.board.virtual_board_mut().remove_piece(3);
    game.place_piece(62, Piece::WhiteQueen, white_queen_chip);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Move),
        "Pawn promotion to queen should be valid"
    );
}

#[test]
fn test_pawn_promotion_to_knight() {
    let mut game = TestGame::new();

    let white_knight_chip = game.hw.board.virtual_board().get(1).cloned().unwrap().chip_id;

    game.make_move(15, 31);
    game.make_move(54, 38);
    game.make_capture(31, 38);
    game.make_move(62, 45);
    game.make_move(38, 46);
    game.make_move(45, 28);
    game.make_move(46, 54);
    game.make_move(28, 43);

    game.hw.buzzer.sounds.clear();

    game.lift_piece(54);
    game.hw.board.virtual_board_mut().remove_piece(1);
    game.place_piece(62, Piece::WhiteKnight, white_knight_chip);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Move),
        "Pawn promotion to knight should be valid"
    );
}

#[test]
fn test_invalid_move_rejected() {
    let mut game = TestGame::new();

    let piece = game.hw.board.virtual_board().get(12).cloned().unwrap();
    game.lift_piece(12);
    game.place_piece(21, piece.piece, piece.chip_id);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Error),
        "Invalid diagonal pawn move (no capture) should be rejected"
    );
    assert!(
        game.hw.display.messages.contains(&DisplayMessage::InvalidMove),
        "Display should show invalid move message"
    );
}

#[test]
fn test_capture_move() {
    let mut game = TestGame::new();

    game.make_move(12, 28);
    game.make_move(51, 35);
    game.make_capture(28, 35);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Move),
        "Capture should be valid"
    );
}

#[test]
fn test_simple_takeback() {
    let mut game = TestGame::new();

    game.make_move(12, 28);
    game.make_move(52, 36);

    let piece = game.hw.board.virtual_board().get(36).cloned().unwrap();
    game.hw.board.virtual_board_mut().remove_piece(36);
    let reading = poll_reading(&mut game.hw.board);
    game.app
        .on_board_reading(reading, &mut game.hw.display, &mut game.hw.buzzer, &mut game.hw.delay);

    game.hw
        .board
        .virtual_board_mut()
        .place_piece_with_chip_id(52, piece.piece, piece.chip_id);
    let reading = poll_reading(&mut game.hw.board);
    game.app
        .on_board_reading(reading, &mut game.hw.display, &mut game.hw.buzzer, &mut game.hw.delay);

    assert!(
        game.hw
            .display
            .messages
            .iter()
            .any(|m| matches!(m, DisplayMessage::Turn { is_white: false })),
        "After takeback, it should be Black's turn again"
    );
}

#[test]
fn test_timer_countdown() {
    let mut game = TestGame::new();

    for _ in 0..100 {
        game.tick(1000);
    }

    let (min, sec) = game.hw.clock.white_time;
    assert!(
        min == 8 && (sec == 20 || sec == 21),
        "Timer should be around 8:20 after 100 seconds, got {}:{:02}", min, sec
    );
}

#[test]
fn test_timer_switches_on_move() {
    let mut game = TestGame::new();

    game.tick(1000);
    assert!(game.hw.clock.white_active);
    assert!(!game.hw.clock.black_active);

    game.make_move(12, 28);
    game.tick(1000);

    assert!(!game.hw.clock.white_active);
    assert!(game.hw.clock.black_active);
}

#[test]
fn test_led_shows_legal_moves_for_pawn() {
    let mut game = TestGame::new();

    game.lift_piece(12);

    assert!(game.app.led_state().is_on(12), "Source square should be lit");
    assert!(game.app.led_state().is_on(20), "e3 should be lit (one square forward)");
    assert!(game.app.led_state().is_on(28), "e4 should be lit (two squares forward)");
    assert!(!game.app.led_state().is_on(21), "f3 should NOT be lit (no capture available)");
}

#[test]
fn test_led_shows_legal_moves_for_knight() {
    let mut game = TestGame::new();

    game.lift_piece(6);

    assert!(game.app.led_state().is_on(6), "Source square should be lit");
    assert!(game.app.led_state().is_on(21), "f3 should be lit");
    assert!(game.app.led_state().is_on(23), "h3 should be lit");
    assert!(!game.app.led_state().is_on(12), "e2 should NOT be lit (blocked by own pawn)");
}

#[test]
fn test_knight_cannot_move_to_own_piece() {
    let mut game = TestGame::new();

    let piece = game.hw.board.virtual_board().get(6).cloned().unwrap();
    game.lift_piece(6);
    game.place_piece(15, piece.piece, piece.chip_id);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Error),
        "Knight moving to own piece square should be rejected"
    );
}

#[test]
fn test_opening_sequence_italian_game() {
    let mut game = TestGame::new();

    game.make_move(12, 28);
    game.make_move(52, 36);
    game.make_move(6, 21);
    game.make_move(57, 42);
    game.make_move(5, 26);

    assert_eq!(game.app.state(), AppState::GameInProgress);
    assert!(
        game.hw
            .display
            .messages
            .iter()
            .any(|m| matches!(m, DisplayMessage::Turn { is_white: false })),
        "After Bc4, it should be Black's turn"
    );
}

#[test]
fn test_scholars_mate_checkmate() {
    let mut game = TestGame::new();

    game.make_move(12, 28);
    game.make_move(52, 36);
    game.make_move(3, 21);
    game.make_move(57, 42);
    game.make_move(5, 26);
    game.make_move(48, 40);
    game.make_capture(21, 53);

    assert_eq!(
        game.app.state(),
        AppState::GameEnded,
        "Game should end after Scholar's mate"
    );
    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::GameOver),
        "Game over sound should play"
    );
    assert!(
        game.hw
            .display
            .messages
            .iter()
            .any(|m| matches!(m, DisplayMessage::GameOver { winner, reason } if winner == "White" && reason == "Checkmate")),
        "Display should show White wins by checkmate"
    );
}

#[test]
fn test_cannot_move_into_check() {
    let mut game = TestGame::new();

    game.make_move(11, 27);
    game.make_move(52, 36);
    game.make_move(10, 26);
    game.make_move(59, 32);

    let piece = game.hw.board.virtual_board().get(4).cloned().unwrap();
    game.lift_piece(4);
    game.place_piece(11, piece.piece, piece.chip_id);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Error),
        "Moving king into check should be rejected (queen on a5 attacks d2)"
    );
}

#[test]
fn test_game_reset_after_end() {
    let mut game = TestGame::new();

    game.make_move(12, 28);
    game.make_move(52, 36);
    game.make_move(3, 21);
    game.make_move(57, 42);
    game.make_move(5, 26);
    game.make_move(48, 40);
    game.make_capture(21, 53);

    assert_eq!(game.app.state(), AppState::GameEnded);

    game.app.on_white_button(&mut game.hw.display);

    assert_eq!(game.app.state(), AppState::WaitingForSetup);
    assert!(
        game.hw
            .display
            .messages
            .contains(&DisplayMessage::WaitingForSetup),
        "Display should show waiting for setup"
    );
}

#[test]
fn test_multiple_moves_alternating_turns() {
    let mut game = TestGame::new();

    game.make_move(12, 28);
    assert!(game
        .hw
        .display
        .messages
        .iter()
        .any(|m| matches!(m, DisplayMessage::Turn { is_white: false })));

    game.make_move(52, 36);
    assert!(game
        .hw
        .display
        .messages
        .iter()
        .any(|m| matches!(m, DisplayMessage::Turn { is_white: true })));

    game.make_move(6, 21);
    assert!(game
        .hw
        .display
        .messages
        .iter()
        .any(|m| matches!(m, DisplayMessage::Turn { is_white: false })));
}

#[test]
fn test_rook_move() {
    let mut game = TestGame::new();

    game.make_move(8, 16);
    game.make_move(52, 36);
    game.make_move(0, 8);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Move),
        "Rook move should be valid"
    );
}

#[test]
fn test_bishop_move() {
    let mut game = TestGame::new();

    game.make_move(12, 28);
    game.make_move(52, 36);
    game.make_move(5, 26);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Move),
        "Bishop move should be valid"
    );
}

#[test]
fn test_queen_move() {
    let mut game = TestGame::new();

    game.make_move(11, 27);
    game.make_move(52, 36);
    game.make_move(3, 11);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Move),
        "Queen move should be valid"
    );
}

#[test]
fn test_king_move() {
    let mut game = TestGame::new();

    game.make_move(12, 28);
    game.make_move(52, 36);
    game.make_move(4, 12);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Move),
        "King move should be valid"
    );
}

#[test]
fn test_clock_display_format() {
    let mut game = TestGame::new();

    game.tick(100);

    let (min, sec) = game.hw.clock.white_time;
    assert_eq!(min, 10);
    assert_eq!(sec, 0);
}

#[test]
fn test_pawn_cannot_move_backwards() {
    let mut game = TestGame::new();

    game.make_move(12, 28);
    game.make_move(52, 36);

    let piece = game.hw.board.virtual_board().get(28).cloned().unwrap();
    game.lift_piece(28);
    game.place_piece(20, piece.piece, piece.chip_id);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Error),
        "Pawn moving backwards should be rejected"
    );
}

#[test]
fn test_cannot_castle_after_king_moves() {
    let mut game = TestGame::new();

    game.make_move(12, 28);
    game.make_move(52, 36);
    game.make_move(6, 21);
    game.make_move(57, 42);
    game.make_move(5, 12);
    game.make_move(62, 45);
    game.make_move(4, 5);
    game.make_move(60, 61);
    game.make_move(5, 4);
    game.make_move(61, 60);

    game.hw.buzzer.sounds.clear();
    game.make_castling(4, 6, 7, 5);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Error),
        "Castling after king has moved should be rejected"
    );
}

#[test]
fn test_display_shows_last_move() {
    let mut game = TestGame::new();

    game.make_move(12, 28);

    assert!(
        game.hw
            .display
            .messages
            .iter()
            .any(|m| matches!(m, DisplayMessage::LastMove { from: 12, to: 28 })),
        "Display should show last move e2-e4"
    );
}

#[test]
fn test_black_queenside_castling() {
    let mut game = TestGame::new();

    game.make_move(12, 28);
    game.make_move(51, 35);
    game.make_move(3, 39);
    game.make_move(58, 37);
    game.make_move(6, 21);
    game.make_move(57, 42);
    game.make_move(5, 12);
    game.make_move(59, 51);
    game.make_move(8, 16);

    game.hw.buzzer.sounds.clear();
    game.make_castling(60, 58, 56, 59);

    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::Move),
        "Black queenside castling should be valid"
    );
}

#[test]
fn test_timer_expiration_ends_game() {
    let mut game = TestGame::new();

    for _ in 0..601 {
        game.tick(1000);
    }

    assert_eq!(
        game.app.state(),
        AppState::GameEnded,
        "Game should end when timer expires"
    );
    assert!(
        game.hw.buzzer.sounds.contains(&BuzzerSound::TimeExpired),
        "Time expired sound should play"
    );
}

#[test]
fn test_leds_clear_after_checkmate() {
    let mut game = TestGame::new();

    game.make_move(12, 28);
    game.make_move(52, 36);
    game.make_move(3, 21);
    game.make_move(57, 42);
    game.make_move(5, 26);
    game.make_move(48, 40);
    game.make_capture(21, 53);

    assert_eq!(game.app.state(), AppState::GameEnded);
    assert!(
        !game.app.led_state().is_on(53),
        "LEDs should be clear after checkmate"
    );
    assert!(
        !game.app.led_state().is_on(21),
        "LEDs should be clear after checkmate"
    );
}
