use crate::app::traits::{BoardIO, BuzzerIO, ClockDisplayIO, DelayIO, DisplayIO};
use crate::buzzer::Buzzer;
use crate::certabo::board::BoardError;
use crate::display::Display;
use crate::peripherals::Peripherals;
use crate::tm1637::ChessClockDisplays;
use crate::usb::{CP210xDevice, USB};
use embedded_hal::delay::DelayNs;

pub struct ArmBoard;

impl BoardIO for ArmBoard {
    fn poll(&mut self) {
        USB::with(|usb| usb.poll());
    }

    fn has_line(&self) -> bool {
        CP210xDevice::has_line()
    }

    fn read_line(&mut self, out: &mut [u8]) -> Option<usize> {
        CP210xDevice::read_line(out)
    }

    fn send_leds(&mut self, data: &[u8; 8]) -> Result<(), BoardError> {
        CP210xDevice::send_leds(data)
    }

    fn clear_buffer(&mut self) {
        CP210xDevice::clear_buffer();
    }
}

pub struct ArmDisplay;

impl DisplayIO for ArmDisplay {
    fn show_calibration_prompt(&mut self) {
        Display::with(|d| d.show_calibration_prompt());
    }

    fn show_calibration_progress(&mut self, count: u8) {
        Display::with(|d| d.show_calibration_progress(count));
    }

    fn show_calibration_complete(&mut self) {
        Display::with(|d| d.show_calibration_complete());
    }

    fn show_waiting_for_setup(&mut self) {
        Display::with(|d| d.show_waiting_for_setup());
    }

    fn show_turn(&mut self, is_white: bool) {
        Display::with(|d| d.show_turn(is_white));
    }

    fn show_last_move(&mut self, from: u8, to: u8) {
        Display::with(|d| d.show_last_move(from, to));
    }

    fn show_paused(&mut self) {
        Display::with(|d| d.show_paused());
    }

    fn show_promotion_prompt(&mut self) {
        Display::with(|d| d.show_promotion_prompt());
    }

    fn show_invalid_move(&mut self) {
        Display::with(|d| d.show_invalid_move());
    }

    fn show_game_over(&mut self, winner: &str, reason: &str) {
        Display::with(|d| d.show_game_over(winner, reason));
    }

    fn show_draw(&mut self, reason: &str) {
        Display::with(|d| d.show_draw(reason));
    }

    fn show_takeback_complete(&mut self) {
        Display::with(|d| d.show_takeback_complete());
    }

    fn show_takeback_castling(&mut self) {
        Display::with(|d| d.show_takeback_castling());
    }

    fn show_takeback_en_passant(&mut self) {
        Display::with(|d| d.show_takeback_en_passant());
    }

    fn show_takeback_promotion(&mut self) {
        Display::with(|d| d.show_takeback_promotion());
    }

    fn show_takeback_capture(&mut self) {
        Display::with(|d| d.show_takeback_capture());
    }
}

pub struct ArmBuzzer;

impl BuzzerIO for ArmBuzzer {
    fn calibration_complete(&mut self) {
        Buzzer::with(|b| b.calibration_complete());
    }

    fn move_sound(&mut self) {
        Buzzer::with(|b| b.move_sound());
    }

    fn error_sound(&mut self) {
        Buzzer::with(|b| b.error_sound());
    }

    fn game_over_sound(&mut self) {
        Buzzer::with(|b| b.game_over_sound());
    }

    fn low_time_warning(&mut self) {
        Buzzer::with(|b| b.low_time_warning());
    }

    fn time_expired(&mut self) {
        Buzzer::with(|b| b.time_expired());
    }
}

pub struct ArmClockDisplay;

impl ClockDisplayIO for ArmClockDisplay {
    fn toggle_colon(&mut self) {
        ChessClockDisplays::with(|d| d.toggle_colon());
    }

    fn update_white(&mut self, minutes: u8, seconds: u8, active: bool) {
        ChessClockDisplays::with(|d| d.update_white(minutes, seconds, active));
    }

    fn update_black(&mut self, minutes: u8, seconds: u8, active: bool) {
        ChessClockDisplays::with(|d| d.update_black(minutes, seconds, active));
    }
}

pub struct ArmDelay;

impl DelayIO for ArmDelay {
    fn delay_ms(&mut self, ms: u32) {
        Peripherals::with(|p| p.delay.delay_ms(ms));
    }
}
