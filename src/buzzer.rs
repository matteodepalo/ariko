use crate::peripherals::Peripherals;
use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

static BUZZER: Mutex<RefCell<Option<Buzzer>>> = Mutex::new(RefCell::new(None));

const RESONANT_FREQ: u32 = 2700;
const CYCLE: u32 = 1000000 / RESONANT_FREQ;

const NOTE_E7: u32 = 2637;
const NOTE_G7: u32 = 3136;
const NOTE_A7: u32 = 3520;
const NOTE_REST: u32 = 0;

pub struct Buzzer;

impl Buzzer {
  pub fn init() {
    critical_section::with(|cs| {
      BUZZER.borrow(cs).replace(Some(Buzzer));
    });
  }

  pub fn with<F, R>(f: F) -> R
  where
    F: FnOnce(&mut Buzzer) -> R,
  {
    critical_section::with(|cs| {
      let mut borrow = BUZZER.borrow(cs).borrow_mut();
      let buzzer = borrow.as_mut().expect("Buzzer not initialized");
      f(buzzer)
    })
  }

  pub fn beep(&self) {
    Peripherals::with(|p| {
      p.buzzer.set_high().unwrap();
      p.delay.delay_us(CYCLE / 2);
      p.buzzer.set_low().unwrap();
      p.delay.delay_us(CYCLE / 2); // run the PMW cycle
    });
  }

  fn beep_cycles(&self, cycles: u32) {
    for _ in 0..cycles {
      self.beep();
    }
  }

  fn play_note(&self, frequency_hz: u32, duration_ms: u32) {
    if frequency_hz == 0 {
      Peripherals::with(|p| p.delay.delay_ms(duration_ms));
      return;
    }

    let cycle_us = 1_000_000 / frequency_hz;
    let half_cycle_us = cycle_us / 2;
    let total_cycles = (duration_ms * 1000) / cycle_us;

    Peripherals::with(|p| {
      for _ in 0..total_cycles {
        p.buzzer.set_high().unwrap();
        p.delay.delay_us(half_cycle_us);
        p.buzzer.set_low().unwrap();
        p.delay.delay_us(half_cycle_us);
      }
    });
  }

  /// Short beep for valid move
  pub fn move_sound(&self) {
    self.beep_cycles(135);
  }

  /// Long beep for invalid move
  pub fn error_sound(&self) {
    self.beep_cycles(540);
  }

  /// Alarm sound for low time warning
  pub fn low_time_warning(&self) {
    self.beep_cycles(80);
    Peripherals::with(|p| p.delay.delay_ms(100));
    self.beep_cycles(80);
  }

  /// Continuous alarm for time expired
  pub fn time_expired(&self) {
    self.beep_cycles(1350);
  }

  /// Game over fanfare using notes near resonant frequency
  pub fn game_over_sound(&self) {
    self.play_note(NOTE_E7, 100);
    self.play_note(NOTE_REST, 30);
    self.play_note(NOTE_E7, 100);
    self.play_note(NOTE_REST, 30);
    self.play_note(NOTE_E7, 100);
    self.play_note(NOTE_REST, 80);
    self.play_note(NOTE_G7, 150);
    self.play_note(NOTE_REST, 50);
    self.play_note(NOTE_E7, 80);
    self.play_note(NOTE_REST, 30);
    self.play_note(NOTE_A7, 300);
  }

  /// Calibration complete confirmation
  pub fn calibration_complete(&self) {
    self.beep_cycles(100);
    Peripherals::with(|p| p.delay.delay_ms(50));
    self.beep_cycles(150);
    Peripherals::with(|p| p.delay.delay_ms(50));
    self.beep_cycles(200);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_resonant_frequency() {
    assert_eq!(RESONANT_FREQ, 2700);
  }

  #[test]
  fn test_pwm_cycle_calculation() {
    assert_eq!(CYCLE, 370);
  }

  #[test]
  fn test_half_cycle_timing() {
    let half_cycle = CYCLE / 2;
    assert_eq!(half_cycle, 185);
  }

  #[test]
  fn test_frequency_to_period_relationship() {
    let expected_cycle = 1_000_000_u32 / RESONANT_FREQ;
    assert_eq!(CYCLE, expected_cycle);
  }

  #[test]
  fn test_note_frequencies_near_resonance() {
    assert!(NOTE_E7 > 2000 && NOTE_E7 < 4000);
    assert!(NOTE_G7 > 2000 && NOTE_G7 < 4000);
    assert!(NOTE_A7 > 2000 && NOTE_A7 < 4000);
  }
}
