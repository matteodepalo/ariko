#![allow(dead_code)]

use crate::peripherals::Peripherals;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::OutputPin;

static mut S_BUZZER: Option<Buzzer> = None;

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
    unsafe { S_BUZZER = Some(Buzzer) }
  }

  pub fn get() -> &'static mut Self {
    unsafe { S_BUZZER.as_mut().unwrap() }
  }
}

impl Buzzer {
  // fn play_tune() {
  //   let p = Peripherals::get();
  //
  //   for (i, x) in TUNE.iter().enumerate() {
  //     tone(tonePin, x); //output the "x" note
  //     p.delay.try_delay_ms(400 * DURATION[i]); // rhythm of the music,it can be tuned fast and slow by change the number"400"
  //     noTone(tonePin); //stop the current note and go to the next note
  //   }
  // }

  pub fn beep(&self) {
    let p = unsafe { Peripherals::get() };

    p.buzzer.try_set_high().unwrap();
    p.delay.try_delay_us(CYCLE / 2).unwrap();
    p.buzzer.try_set_low().unwrap();
    p.delay.try_delay_us(CYCLE / 2).unwrap(); // run the PMW cycle
  }
}
