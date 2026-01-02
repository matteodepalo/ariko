use crate::app::traits::{BoardIO, BuzzerIO, ClockDisplayIO, DelayIO, DisplayIO, Hardware};
use crate::certabo::calibration::{CalibrationData, Piece as CalibrationPiece};
use crate::certabo::leds::LedState;
use crate::certabo::protocol::RfidReading;
use crate::game::chess::{BoardStatus, PieceType, UndoInfo};
use crate::game::state::{GameState, GameStatus};
use crate::game::timer::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AppState {
    Initializing,
    WaitingForCalibration,
    Calibrating,
    WaitingForSetup,
    GameInProgress,
    GamePaused,
    GameEnded,
}

fn calibration_piece_to_type(piece: CalibrationPiece) -> PieceType {
    match piece {
        CalibrationPiece::WhitePawn | CalibrationPiece::BlackPawn => PieceType::Pawn,
        CalibrationPiece::WhiteKnight | CalibrationPiece::BlackKnight => PieceType::Knight,
        CalibrationPiece::WhiteBishop | CalibrationPiece::BlackBishop => PieceType::Bishop,
        CalibrationPiece::WhiteRook | CalibrationPiece::BlackRook => PieceType::Rook,
        CalibrationPiece::WhiteQueen | CalibrationPiece::BlackQueen => PieceType::Queen,
        CalibrationPiece::WhiteKing | CalibrationPiece::BlackKing => PieceType::King,
    }
}

fn is_promotion_rank(square: u8, is_white_pawn: bool) -> bool {
    let rank = square / 8;
    if is_white_pawn {
        rank == 7
    } else {
        rank == 0
    }
}

struct PendingPromotion {
    from: u8,
    to: u8,
}

struct PendingTakeback {
    from: u8,
    to: u8,
    captured_square: u8,
    has_capture: bool,
    castling_rook_from: Option<u8>,
    castling_rook_to: Option<u8>,
    is_promotion: bool,
    side_is_white: bool,
}

impl PendingTakeback {
    fn from_undo_info(info: &UndoInfo, side_is_white: bool) -> Self {
        let (castling_rook_from, castling_rook_to) = if let Some(kingside) = info.castling {
            if side_is_white {
                if kingside {
                    (Some(5u8), Some(7u8))
                } else {
                    (Some(3u8), Some(0u8))
                }
            } else {
                if kingside {
                    (Some(61u8), Some(63u8))
                } else {
                    (Some(59u8), Some(56u8))
                }
            }
        } else {
            (None, None)
        };

        Self {
            from: info.from,
            to: info.to,
            captured_square: info.captured_square,
            has_capture: info.captured.is_some(),
            castling_rook_from,
            castling_rook_to,
            is_promotion: info.promotion.is_some(),
            side_is_white,
        }
    }

    fn squares_to_highlight(&self) -> [Option<u8>; 4] {
        let mut squares = [None; 4];
        squares[0] = Some(self.from);
        if self.has_capture {
            squares[1] = Some(self.captured_square);
        }
        if let Some(rook_to) = self.castling_rook_to {
            squares[2] = Some(rook_to);
        }
        squares
    }

    fn is_complete(&self, board: &[Option<CalibrationPiece>; 64]) -> bool {
        let Some(from_piece) = board[self.from as usize] else {
            return false;
        };

        let from_piece_correct_color = from_piece.is_white() == self.side_is_white;
        if !from_piece_correct_color {
            return false;
        }

        if self.has_capture {
            let Some(captured_piece) = board[self.captured_square as usize] else {
                return false;
            };
            let captured_piece_opponent = captured_piece.is_white() != self.side_is_white;
            if !captured_piece_opponent {
                return false;
            }
        } else if board[self.to as usize].is_some() {
            return false;
        }

        if self.is_promotion {
            let correct_pawn = if self.side_is_white {
                matches!(from_piece, CalibrationPiece::WhitePawn)
            } else {
                matches!(from_piece, CalibrationPiece::BlackPawn)
            };
            if !correct_pawn {
                return false;
            }
        }

        if let (Some(rook_from), Some(rook_to)) = (self.castling_rook_from, self.castling_rook_to) {
            if board[rook_from as usize].is_some() {
                return false;
            }
            let Some(rook_piece) = board[rook_to as usize] else {
                return false;
            };
            let correct_rook = if self.side_is_white {
                matches!(rook_piece, CalibrationPiece::WhiteRook)
            } else {
                matches!(rook_piece, CalibrationPiece::BlackRook)
            };
            if !correct_rook {
                return false;
            }
        }

        true
    }
}

pub struct App {
    state: AppState,
    calibration: CalibrationData,
    game: GameState,
    last_reading: Option<RfidReading>,
    led_state: LedState,
    led_dirty: bool,
    tick_count: u8,
    last_move: Option<(u8, u8)>,
    spinner_frame: u8,
    pending_promotion: Option<PendingPromotion>,
    pending_takeback: Option<PendingTakeback>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Initializing,
            calibration: CalibrationData::new(),
            game: GameState::new(),
            last_reading: None,
            led_state: LedState::new(),
            led_dirty: false,
            tick_count: 0,
            last_move: None,
            spinner_frame: 0,
            pending_promotion: None,
            pending_takeback: None,
        }
    }

    pub fn state(&self) -> AppState {
        self.state
    }

    pub fn led_state(&self) -> &LedState {
        &self.led_state
    }

    pub fn game(&self) -> &GameState {
        &self.game
    }

    pub fn start<B, D, Z, C, L>(&mut self, hw: &mut Hardware<B, D, Z, C, L>)
    where
        B: BoardIO,
        D: DisplayIO,
        Z: BuzzerIO,
        C: ClockDisplayIO,
        L: DelayIO,
    {
        self.state = AppState::WaitingForCalibration;
        hw.display.show_calibration_prompt();
    }

    pub fn on_blue_button<D: DisplayIO>(&mut self, display: &mut D) {
        match self.state {
            AppState::WaitingForCalibration | AppState::WaitingForSetup | AppState::GameEnded => {
                self.state = AppState::Calibrating;
                display.show_calibration_prompt();
            }
            _ => {}
        }
    }

    pub fn on_white_button<D: DisplayIO>(&mut self, display: &mut D) {
        match self.state {
            AppState::GameInProgress | AppState::GamePaused => {
                self.game.toggle_pause();
                match self.game.status() {
                    GameStatus::Paused => {
                        self.state = AppState::GamePaused;
                        display.show_paused();
                    }
                    GameStatus::InProgress => {
                        self.state = AppState::GameInProgress;
                    }
                    _ => {}
                }
            }
            AppState::GameEnded => {
                self.game.reset();
                self.state = AppState::WaitingForSetup;
                display.show_waiting_for_setup();
            }
            _ => {}
        }
    }

    pub fn on_board_reading<D: DisplayIO, Z: BuzzerIO, L: DelayIO>(
        &mut self,
        reading: RfidReading,
        display: &mut D,
        buzzer: &mut Z,
        delay: &mut L,
    ) {
        match self.state {
            AppState::Calibrating => {
                self.do_calibration(&reading, display, buzzer, delay);
            }
            AppState::WaitingForSetup => {
                self.check_starting_position(&reading, display, buzzer);
            }
            AppState::GameInProgress => {
                self.process_game_move(&reading, display, buzzer);
            }
            _ => {}
        }

        self.last_reading = Some(reading);
    }

    fn do_calibration<D: DisplayIO, Z: BuzzerIO, L: DelayIO>(
        &mut self,
        reading: &RfidReading,
        display: &mut D,
        buzzer: &mut Z,
        delay: &mut L,
    ) {
        let count = self.calibration.calibrate_from_starting_position(reading);
        display.show_calibration_progress(count as u8);

        if self.calibration.is_complete() {
            self.state = AppState::WaitingForSetup;
            self.led_state.clear_all();
            self.led_dirty = true;
            buzzer.calibration_complete();
            display.show_calibration_complete();
            delay.delay_ms(1500);
            display.show_waiting_for_setup();
        }
    }

    fn check_starting_position<D: DisplayIO, Z: BuzzerIO>(
        &mut self,
        reading: &RfidReading,
        display: &mut D,
        buzzer: &mut Z,
    ) {
        let board = self.calibration.reading_to_board(reading);

        let mut correct = 0;
        for square in 0..64u8 {
            if board[square as usize].is_some() {
                correct += 1;
            }
        }

        if correct >= 32 {
            self.state = AppState::GameInProgress;
            self.game.set_status(GameStatus::InProgress);
            buzzer.move_sound();
            self.update_display(display);
        }
    }

    fn process_game_move<D: DisplayIO, Z: BuzzerIO>(
        &mut self,
        reading: &RfidReading,
        display: &mut D,
        buzzer: &mut Z,
    ) {
        let current_board = self.calibration.reading_to_board(reading);

        if self.handle_pending_takeback(&current_board, display, buzzer) {
            return;
        }

        if self.handle_pending_promotion(&current_board, display, buzzer) {
            return;
        }

        let Some(ref last_reading) = self.last_reading else {
            return;
        };
        let previous_board = self.calibration.reading_to_board(last_reading);

        let mut lifted_from: Option<u8> = None;
        let mut placed_to: Option<u8> = None;

        for square in 0..64u8 {
            let prev = previous_board[square as usize];
            let curr = current_board[square as usize];

            if prev.is_some() && curr.is_none() {
                lifted_from = Some(square);
            } else if prev.is_none() && curr.is_some() {
                placed_to = Some(square);
            } else if prev.is_some() && curr.is_some() && prev != curr {
                placed_to = Some(square);
            }
        }

        if self.detect_takeback_start(lifted_from, placed_to, &current_board, display, buzzer) {
            return;
        }

        if let Some(from) = lifted_from {
            if placed_to.is_none() {
                self.game.piece_lifted(from);
                let destinations = self.game.legal_destinations(from);
                self.led_state.clear_all();
                self.led_state.set(from);
                for dest in destinations {
                    self.led_state.set(dest);
                }
                self.led_dirty = true;
            }
        }

        if let (Some(from), Some(to)) = (self.game.lifted_piece(), placed_to) {
            if self.game.is_legal_move(from, to) {
                if let Some(piece) = previous_board[from as usize] {
                    let is_pawn = matches!(
                        piece,
                        CalibrationPiece::WhitePawn | CalibrationPiece::BlackPawn
                    );
                    let is_white = piece.is_white();

                    if is_pawn && is_promotion_rank(to, is_white) {
                        if let Some(new_piece) = current_board[to as usize] {
                            let piece_type = calibration_piece_to_type(new_piece);

                            if matches!(
                                piece_type,
                                PieceType::Queen | PieceType::Rook | PieceType::Bishop | PieceType::Knight
                            ) {
                                self.game.make_move_with_promotion(from, to, Some(piece_type));
                                self.last_move = Some((from, to));
                                buzzer.move_sound();
                                self.led_state.clear_all();
                                self.led_dirty = true;
                                self.check_game_end(display, buzzer);
                                self.update_display(display);
                                return;
                            }
                        }

                        self.pending_promotion = Some(PendingPromotion { from, to });
                        display.show_promotion_prompt();
                        self.led_state.clear_all();
                        self.led_state.set(to);
                        self.led_dirty = true;
                        return;
                    }
                }

                self.game.make_move(from, to);
                self.last_move = Some((from, to));
                buzzer.move_sound();
                self.led_state.clear_all();
                self.led_dirty = true;
                self.check_game_end(display, buzzer);
                self.update_display(display);
            } else {
                buzzer.error_sound();
                display.show_invalid_move();
            }
        }
    }

    fn handle_pending_takeback<D: DisplayIO, Z: BuzzerIO>(
        &mut self,
        current_board: &[Option<CalibrationPiece>; 64],
        display: &mut D,
        buzzer: &mut Z,
    ) -> bool {
        let Some(ref takeback) = self.pending_takeback else {
            return false;
        };

        if takeback.is_complete(current_board) {
            self.game.undo_move();
            self.last_move = None;
            self.pending_takeback = None;
            buzzer.move_sound();
            self.led_state.clear_all();
            self.led_dirty = true;
            display.show_takeback_complete();
            self.update_display(display);
            return true;
        }

        self.led_state.clear_all();
        for sq in takeback.squares_to_highlight() {
            if let Some(s) = sq {
                self.led_state.set(s);
            }
        }
        self.led_dirty = true;
        true
    }

    fn handle_pending_promotion<D: DisplayIO, Z: BuzzerIO>(
        &mut self,
        current_board: &[Option<CalibrationPiece>; 64],
        display: &mut D,
        buzzer: &mut Z,
    ) -> bool {
        let Some(ref promotion) = self.pending_promotion else {
            return false;
        };

        let to = promotion.to;
        let from = promotion.from;

        if let Some(new_piece) = current_board[to as usize] {
            let piece_type = calibration_piece_to_type(new_piece);

            if matches!(
                piece_type,
                PieceType::Queen | PieceType::Rook | PieceType::Bishop | PieceType::Knight
            ) {
                self.game.make_move_with_promotion(from, to, Some(piece_type));
                self.last_move = Some((from, to));
                self.pending_promotion = None;
                buzzer.move_sound();
                self.led_state.clear_all();
                self.led_dirty = true;
                self.check_game_end(display, buzzer);
                self.update_display(display);
                return true;
            }
        }
        true
    }

    fn detect_takeback_start<D: DisplayIO, Z: BuzzerIO>(
        &mut self,
        lifted_from: Option<u8>,
        placed_to: Option<u8>,
        current_board: &[Option<CalibrationPiece>; 64],
        display: &mut D,
        buzzer: &mut Z,
    ) -> bool {
        let Some(undo_info) = self.game.last_undo_info() else {
            return false;
        };

        let undo_from = undo_info.from;
        let undo_to = undo_info.to;
        let has_capture = undo_info.captured.is_some();
        let is_castling = undo_info.castling.is_some();
        let is_promotion = undo_info.promotion.is_some();
        let is_en_passant = has_capture && undo_info.captured_square != undo_to;

        let side_is_white = match self.game.current_turn() {
            Color::White => false,
            Color::Black => true,
        };

        let is_simple_takeback = !has_capture && !is_castling && !is_promotion;

        if let (Some(from), Some(to)) = (lifted_from, placed_to) {
            if from == undo_to && to == undo_from {
                if is_simple_takeback {
                    self.game.undo_move();
                    self.last_move = None;
                    buzzer.move_sound();
                    self.led_state.clear_all();
                    self.led_dirty = true;
                    self.update_display(display);
                    return true;
                }

                let pending = PendingTakeback::from_undo_info(undo_info, side_is_white);

                if pending.is_complete(current_board) {
                    self.game.undo_move();
                    self.last_move = None;
                    buzzer.move_sound();
                    self.led_state.clear_all();
                    self.led_dirty = true;
                    self.update_display(display);
                    return true;
                }

                self.pending_takeback = Some(pending);
                if is_castling {
                    display.show_takeback_castling();
                } else if is_en_passant {
                    display.show_takeback_en_passant();
                } else if is_promotion {
                    display.show_takeback_promotion();
                } else {
                    display.show_takeback_capture();
                }
                self.led_state.clear_all();
                if let Some(ref tb) = self.pending_takeback {
                    for sq in tb.squares_to_highlight() {
                        if let Some(s) = sq {
                            self.led_state.set(s);
                        }
                    }
                }
                self.led_dirty = true;
                return true;
            }
        }

        false
    }

    fn check_game_end<D: DisplayIO, Z: BuzzerIO>(&mut self, display: &mut D, buzzer: &mut Z) {
        match self.game.status() {
            GameStatus::WhiteWins => {
                self.state = AppState::GameEnded;
                buzzer.game_over_sound();
                display.show_game_over("White", "Timeout");
                return;
            }
            GameStatus::BlackWins => {
                self.state = AppState::GameEnded;
                buzzer.game_over_sound();
                display.show_game_over("Black", "Timeout");
                return;
            }
            _ => {}
        }

        match self.game.board_status() {
            BoardStatus::Checkmate => {
                self.state = AppState::GameEnded;
                buzzer.game_over_sound();
                let winner = match self.game.current_turn() {
                    Color::White => "Black",
                    Color::Black => "White",
                };
                display.show_game_over(winner, "Checkmate");
            }
            BoardStatus::Stalemate => {
                self.state = AppState::GameEnded;
                buzzer.game_over_sound();
                display.show_draw("Stalemate");
            }
            BoardStatus::FiftyMoveRule => {
                self.state = AppState::GameEnded;
                buzzer.game_over_sound();
                display.show_draw("50-move");
            }
            BoardStatus::InsufficientMaterial => {
                self.state = AppState::GameEnded;
                buzzer.game_over_sound();
                display.show_draw("No material");
            }
            BoardStatus::Ongoing => {}
        }
    }

    pub fn tick<D: DisplayIO, Z: BuzzerIO, C: ClockDisplayIO>(
        &mut self,
        elapsed_ms: u32,
        display: &mut D,
        buzzer: &mut Z,
        clock: &mut C,
    ) {
        self.tick_count = (self.tick_count + 1) % 5;
        if self.tick_count == 0 {
            clock.toggle_colon();
        }

        self.update_clock_displays(clock);

        if self.state == AppState::WaitingForCalibration {
            self.spinner_frame = self.spinner_frame.wrapping_add(1);
            self.led_state = LedState::wave(self.spinner_frame);
            self.led_dirty = true;
            return;
        }

        if self.state == AppState::Calibrating {
            self.spinner_frame = self.spinner_frame.wrapping_add(1);
            self.led_state = LedState::spinner(self.spinner_frame, 4);
            self.led_dirty = true;
            return;
        }

        if self.state != AppState::GameInProgress {
            return;
        }

        let current_turn = self.game.current_turn();

        if self.game.timer().is_low_time(current_turn) && elapsed_ms % 1000 < 50 {
            buzzer.low_time_warning();
        }

        if self.game.tick_timer(elapsed_ms) {
            self.state = AppState::GameEnded;
            buzzer.time_expired();
            let winner = match current_turn {
                Color::White => "Black",
                Color::Black => "White",
            };
            display.show_game_over(winner, "Time");
        }
    }

    fn update_clock_displays<C: ClockDisplayIO>(&self, clock: &mut C) {
        let timer = self.game.timer();
        let current_turn = self.game.current_turn();
        let game_active = self.state == AppState::GameInProgress;

        let (white_min, white_sec) = timer.formatted_time(Color::White);
        let (black_min, black_sec) = timer.formatted_time(Color::Black);

        clock.update_white(
            white_min,
            white_sec,
            game_active && matches!(current_turn, Color::White),
        );
        clock.update_black(
            black_min,
            black_sec,
            game_active && matches!(current_turn, Color::Black),
        );
    }

    pub fn update_display<D: DisplayIO>(&self, display: &mut D) {
        if self.state != AppState::GameInProgress {
            return;
        }

        let turn = self.game.current_turn();
        let is_white = matches!(turn, Color::White);

        display.show_turn(is_white);
        if let Some((from, to)) = self.last_move {
            display.show_last_move(from, to);
        }
    }

    pub fn should_send_leds(&self) -> bool {
        self.led_dirty
    }

    pub fn leds_sent(&mut self) {
        self.led_dirty = false;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
