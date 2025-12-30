#![allow(dead_code)]

use crate::peripherals::Peripherals;
use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

static BUZZER: Mutex<RefCell<Option<Buzzer>>> = Mutex::new(RefCell::new(None));

const NOTE_D0: u32 = 0;
const NOTE_D1: u32 = 294;
const NOTE_D2: u32 = 330;
const NOTE_D3: u32 = 350;
const NOTE_D4: u32 = 393;
const NOTE_D5: u32 = 441;
const NOTE_D6: u32 = 495;
const NOTE_D7: u32 = 556;

const NOTE_DL1: u32 = 147;
const NOTE_DL2: u32 = 165;
const NOTE_DL3: u32 = 175;
const NOTE_DL4: u32 = 196;
const NOTE_DL5: u32 = 221;
const NOTE_DL6: u32 = 248;
const NOTE_DL7: u32 = 278;

const NOTE_DH1: u32 = 589;
const NOTE_DH2: u32 = 661;
const NOTE_DH3: u32 = 700;
const NOTE_DH4: u32 = 786;
const NOTE_DH5: u32 = 882;
const NOTE_DH6: u32 = 990;
const NOTE_DH7: u32 = 112;

const WHOLE: f32 = 1 as f32;
const HALF: f32 = 0.5;
const QUARTER: f32 = 0.25;
const EIGHTH: f32 = 0.25;
const SIXTEENTH: f32 = 0.625;

//the note part of the whole song
const TUNE: [u32; 98] = [
  NOTE_DH1, NOTE_D6, NOTE_D5, NOTE_D6, NOTE_D0, NOTE_DH1, NOTE_D6, NOTE_D5, NOTE_DH1, NOTE_D6,
  NOTE_D0, NOTE_D6, NOTE_D6, NOTE_D6, NOTE_D5, NOTE_D6, NOTE_D0, NOTE_D6, NOTE_DH1, NOTE_D6,
  NOTE_D5, NOTE_DH1, NOTE_D6, NOTE_D0, NOTE_D1, NOTE_D1, NOTE_D3, NOTE_D1, NOTE_D1, NOTE_D3,
  NOTE_D0, NOTE_D6, NOTE_D6, NOTE_D6, NOTE_D5, NOTE_D6, NOTE_D5, NOTE_D1, NOTE_D3, NOTE_D0,
  NOTE_DH1, NOTE_D6, NOTE_D6, NOTE_D5, NOTE_D6, NOTE_D5, NOTE_D1, NOTE_D2, NOTE_D0, NOTE_D7,
  NOTE_D7, NOTE_D5, NOTE_D3, NOTE_D5, NOTE_DH1, NOTE_D0, NOTE_D6, NOTE_D6, NOTE_D5, NOTE_D5,
  NOTE_D6, NOTE_D6, NOTE_D0, NOTE_D5, NOTE_D1, NOTE_D3, NOTE_D0, NOTE_DH1, NOTE_D0, NOTE_D6,
  NOTE_D6, NOTE_D5, NOTE_D5, NOTE_D6, NOTE_D6, NOTE_D0, NOTE_D5, NOTE_D1, NOTE_D2, NOTE_D0,
  NOTE_D3, NOTE_D3, NOTE_D1, NOTE_DL6, NOTE_D1, NOTE_D3, NOTE_D5, NOTE_D6, NOTE_D6, NOTE_D3,
  NOTE_D5, NOTE_D6, NOTE_D6, NOTE_DH1, NOTE_D0, NOTE_D7, NOTE_D5, NOTE_D6,
];

//the duration time of each note
const DURATION: [f32; 98] = [
  WHOLE,
  WHOLE,
  HALF,
  HALF,
  WHOLE,
  HALF,
  HALF,
  HALF,
  HALF,
  WHOLE,
  HALF,
  HALF,
  HALF,
  WHOLE,
  HALF,
  WHOLE,
  HALF,
  HALF,
  HALF,
  HALF,
  HALF,
  HALF,
  WHOLE,
  WHOLE,
  WHOLE,
  WHOLE,
  WHOLE + WHOLE,
  HALF,
  WHOLE,
  WHOLE + HALF,
  WHOLE,
  WHOLE,
  WHOLE,
  HALF,
  HALF,
  WHOLE,
  HALF,
  WHOLE,
  WHOLE + HALF,
  WHOLE,
  HALF,
  HALF,
  HALF,
  HALF,
  WHOLE + WHOLE,
  HALF,
  WHOLE,
  WHOLE + HALF,
  WHOLE,
  WHOLE + WHOLE,
  HALF,
  HALF,
  WHOLE,
  WHOLE + WHOLE + WHOLE + WHOLE,
  HALF,
  HALF,
  HALF + QUARTER,
  QUARTER,
  HALF + QUARTER,
  QUARTER,
  HALF + QUARTER,
  QUARTER,
  HALF,
  WHOLE,
  HALF,
  WHOLE,
  WHOLE,
  HALF,
  HALF,
  HALF + QUARTER,
  QUARTER,
  HALF + QUARTER,
  QUARTER,
  HALF + QUARTER,
  QUARTER,
  HALF,
  WHOLE,
  HALF,
  WHOLE,
  WHOLE,
  WHOLE + WHOLE,
  HALF,
  HALF,
  WHOLE,
  WHOLE + WHOLE + WHOLE + WHOLE,
  HALF,
  WHOLE,
  HALF,
  WHOLE + WHOLE,
  HALF,
  WHOLE,
  HALF,
  WHOLE + WHOLE,
  WHOLE + WHOLE,
  HALF,
  HALF,
  WHOLE,
  WHOLE + WHOLE + WHOLE + WHOLE,
];

const FREQUENCY: u32 = 2700; //reach the Resonant Frequency
const CYCLE: u32 = 1000000 / FREQUENCY;

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
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_resonant_frequency() {
    // Buzzer resonant frequency is 2700 Hz
    assert_eq!(FREQUENCY, 2700);
  }

  #[test]
  fn test_pwm_cycle_calculation() {
    // CYCLE = 1000000 / 2700 = 370 microseconds (integer division)
    assert_eq!(CYCLE, 370);
  }

  #[test]
  fn test_half_cycle_timing() {
    // Each half of the PWM cycle should be ~185 microseconds
    let half_cycle = CYCLE / 2;
    assert_eq!(half_cycle, 185);
  }

  #[test]
  fn test_note_frequencies_main_octave() {
    // Test main octave notes (D scale)
    assert_eq!(NOTE_D0, 0); // Rest/silence
    assert_eq!(NOTE_D1, 294);
    assert_eq!(NOTE_D2, 330);
    assert_eq!(NOTE_D3, 350);
    assert_eq!(NOTE_D4, 393);
    assert_eq!(NOTE_D5, 441);
    assert_eq!(NOTE_D6, 495);
    assert_eq!(NOTE_D7, 556);
  }

  #[test]
  fn test_note_frequencies_lower_octave() {
    // Lower octave should be roughly half the main octave frequencies
    assert_eq!(NOTE_DL1, 147); // ~294/2
    assert_eq!(NOTE_DL2, 165); // ~330/2
    assert_eq!(NOTE_DL3, 175); // ~350/2
    assert_eq!(NOTE_DL4, 196); // ~393/2
    assert_eq!(NOTE_DL5, 221); // ~441/2
    assert_eq!(NOTE_DL6, 248); // ~495/2
    assert_eq!(NOTE_DL7, 278); // ~556/2
  }

  #[test]
  fn test_note_frequencies_higher_octave() {
    // Higher octave should be roughly double the main octave frequencies
    assert_eq!(NOTE_DH1, 589); // ~294*2
    assert_eq!(NOTE_DH2, 661); // ~330*2
    assert_eq!(NOTE_DH3, 700); // ~350*2
    assert_eq!(NOTE_DH4, 786); // ~393*2
    assert_eq!(NOTE_DH5, 882); // ~441*2
    assert_eq!(NOTE_DH6, 990); // ~495*2
    // NOTE_DH7 appears to be 112 which seems like a typo (should be ~1112)
    assert_eq!(NOTE_DH7, 112);
  }

  #[test]
  fn test_duration_constants() {
    // Test note duration ratios
    assert_eq!(WHOLE, 1.0);
    assert_eq!(HALF, 0.5);
    assert_eq!(QUARTER, 0.25);
    assert_eq!(EIGHTH, 0.25); // Note: EIGHTH equals QUARTER in the source
    assert_eq!(SIXTEENTH, 0.625); // Note: This seems unusual for a sixteenth note
  }

  #[test]
  fn test_tune_and_duration_arrays_same_length() {
    // TUNE and DURATION arrays must have the same length
    assert_eq!(TUNE.len(), DURATION.len());
    assert_eq!(TUNE.len(), 98);
  }

  #[test]
  fn test_frequency_to_period_relationship() {
    // Verify the math: period (us) = 1,000,000 / frequency (Hz)
    // For 2700 Hz: 1000000 / 2700 = 370.37... which truncates to 370
    let expected_cycle = 1_000_000_u32 / FREQUENCY;
    assert_eq!(CYCLE, expected_cycle);
  }
}
