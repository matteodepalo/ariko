use crate::certabo::board::BoardError;

pub trait BoardIO {
    fn poll(&mut self);
    fn has_line(&self) -> bool;
    fn read_line(&mut self, out: &mut [u8]) -> Option<usize>;
    fn send_leds(&mut self, data: &[u8; 8]) -> Result<(), BoardError>;
    fn clear_buffer(&mut self);
}

pub trait DisplayIO {
    fn show_calibration_prompt(&mut self);
    fn show_calibration_progress(&mut self, count: u8);
    fn show_calibration_complete(&mut self);
    fn show_waiting_for_setup(&mut self);
    fn show_turn(&mut self, is_white: bool);
    fn show_last_move(&mut self, from: u8, to: u8);
    fn show_paused(&mut self);
    fn show_promotion_prompt(&mut self);
    fn show_invalid_move(&mut self);
    fn show_game_over(&mut self, winner: &str, reason: &str);
    fn show_draw(&mut self, reason: &str);
    fn show_takeback_complete(&mut self);
    fn show_takeback_castling(&mut self);
    fn show_takeback_en_passant(&mut self);
    fn show_takeback_promotion(&mut self);
    fn show_takeback_capture(&mut self);
}

pub trait BuzzerIO {
    fn calibration_complete(&mut self);
    fn move_sound(&mut self);
    fn error_sound(&mut self);
    fn game_over_sound(&mut self);
    fn low_time_warning(&mut self);
    fn time_expired(&mut self);
}

pub trait ClockDisplayIO {
    fn toggle_colon(&mut self);
    fn update_white(&mut self, minutes: u8, seconds: u8, active: bool);
    fn update_black(&mut self, minutes: u8, seconds: u8, active: bool);
}

pub trait DelayIO {
    fn delay_ms(&mut self, ms: u32);
}

pub struct Hardware<B, D, Z, C, L>
where
    B: BoardIO,
    D: DisplayIO,
    Z: BuzzerIO,
    C: ClockDisplayIO,
    L: DelayIO,
{
    pub board: B,
    pub display: D,
    pub buzzer: Z,
    pub clock: C,
    pub delay: L,
}
