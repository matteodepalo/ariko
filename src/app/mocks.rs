use crate::app::traits::{BoardIO, BuzzerIO, ClockDisplayIO, DelayIO, DisplayIO};
use crate::certabo::board::BoardError;
use crate::certabo::simulator::SimulatedBoard;

impl BoardIO for SimulatedBoard {
    fn poll(&mut self) {
        <SimulatedBoard as crate::certabo::board::CertaboBoard>::poll(self);
    }

    fn has_line(&self) -> bool {
        <SimulatedBoard as crate::certabo::board::CertaboBoard>::has_line(self)
    }

    fn read_line(&mut self, out: &mut [u8]) -> Option<usize> {
        <SimulatedBoard as crate::certabo::board::CertaboBoard>::read_line(self, out)
    }

    fn send_leds(&mut self, data: &[u8; 8]) -> Result<(), BoardError> {
        <SimulatedBoard as crate::certabo::board::CertaboBoard>::send_leds(self, data)
    }

    fn clear_buffer(&mut self) {
        <SimulatedBoard as crate::certabo::board::CertaboBoard>::clear_buffer(self);
    }
}

#[derive(Default)]
pub struct MockDisplay {
    pub last_message: Option<DisplayMessage>,
    pub messages: Vec<DisplayMessage>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DisplayMessage {
    CalibrationPrompt,
    CalibrationProgress(u8),
    CalibrationComplete,
    WaitingForSetup,
    Turn { is_white: bool },
    LastMove { from: u8, to: u8 },
    Paused,
    PromotionPrompt,
    InvalidMove,
    GameOver { winner: String, reason: String },
    Draw { reason: String },
    TakebackComplete,
    TakebackCastling,
    TakebackEnPassant,
    TakebackPromotion,
    TakebackCapture,
}

impl MockDisplay {
    pub fn new() -> Self {
        Self::default()
    }

    fn record(&mut self, msg: DisplayMessage) {
        self.last_message = Some(msg.clone());
        self.messages.push(msg);
    }
}

impl DisplayIO for MockDisplay {
    fn show_calibration_prompt(&mut self) {
        self.record(DisplayMessage::CalibrationPrompt);
    }

    fn show_calibration_progress(&mut self, count: u8) {
        self.record(DisplayMessage::CalibrationProgress(count));
    }

    fn show_calibration_complete(&mut self) {
        self.record(DisplayMessage::CalibrationComplete);
    }

    fn show_waiting_for_setup(&mut self) {
        self.record(DisplayMessage::WaitingForSetup);
    }

    fn show_turn(&mut self, is_white: bool) {
        self.record(DisplayMessage::Turn { is_white });
    }

    fn show_last_move(&mut self, from: u8, to: u8) {
        self.record(DisplayMessage::LastMove { from, to });
    }

    fn show_paused(&mut self) {
        self.record(DisplayMessage::Paused);
    }

    fn show_promotion_prompt(&mut self) {
        self.record(DisplayMessage::PromotionPrompt);
    }

    fn show_invalid_move(&mut self) {
        self.record(DisplayMessage::InvalidMove);
    }

    fn show_game_over(&mut self, winner: &str, reason: &str) {
        self.record(DisplayMessage::GameOver {
            winner: winner.into(),
            reason: reason.into(),
        });
    }

    fn show_draw(&mut self, reason: &str) {
        self.record(DisplayMessage::Draw {
            reason: reason.into(),
        });
    }

    fn show_takeback_complete(&mut self) {
        self.record(DisplayMessage::TakebackComplete);
    }

    fn show_takeback_castling(&mut self) {
        self.record(DisplayMessage::TakebackCastling);
    }

    fn show_takeback_en_passant(&mut self) {
        self.record(DisplayMessage::TakebackEnPassant);
    }

    fn show_takeback_promotion(&mut self) {
        self.record(DisplayMessage::TakebackPromotion);
    }

    fn show_takeback_capture(&mut self) {
        self.record(DisplayMessage::TakebackCapture);
    }
}

#[derive(Default)]
pub struct MockBuzzer {
    pub sounds: Vec<BuzzerSound>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuzzerSound {
    CalibrationComplete,
    Move,
    Error,
    GameOver,
    LowTime,
    TimeExpired,
}

impl MockBuzzer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BuzzerIO for MockBuzzer {
    fn calibration_complete(&mut self) {
        self.sounds.push(BuzzerSound::CalibrationComplete);
    }

    fn move_sound(&mut self) {
        self.sounds.push(BuzzerSound::Move);
    }

    fn error_sound(&mut self) {
        self.sounds.push(BuzzerSound::Error);
    }

    fn game_over_sound(&mut self) {
        self.sounds.push(BuzzerSound::GameOver);
    }

    fn low_time_warning(&mut self) {
        self.sounds.push(BuzzerSound::LowTime);
    }

    fn time_expired(&mut self) {
        self.sounds.push(BuzzerSound::TimeExpired);
    }
}

#[derive(Default)]
pub struct MockClockDisplay {
    pub white_time: (u8, u8),
    pub black_time: (u8, u8),
    pub white_active: bool,
    pub black_active: bool,
    pub colon_on: bool,
}

impl MockClockDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ClockDisplayIO for MockClockDisplay {
    fn toggle_colon(&mut self) {
        self.colon_on = !self.colon_on;
    }

    fn update_white(&mut self, minutes: u8, seconds: u8, active: bool) {
        self.white_time = (minutes, seconds);
        self.white_active = active;
    }

    fn update_black(&mut self, minutes: u8, seconds: u8, active: bool) {
        self.black_time = (minutes, seconds);
        self.black_active = active;
    }
}

#[derive(Default)]
pub struct MockDelay {
    pub total_delay_ms: u32,
}

impl MockDelay {
    pub fn new() -> Self {
        Self::default()
    }
}

impl DelayIO for MockDelay {
    fn delay_ms(&mut self, ms: u32) {
        self.total_delay_ms += ms;
    }
}
