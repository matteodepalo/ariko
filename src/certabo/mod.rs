//! Certabo chess board communication module
//!
//! Handles RFID data parsing, piece calibration, and LED control
//! for the Certabo electronic chess board.

pub mod board;
pub mod buffer;
pub mod calibration;
pub mod leds;
pub mod protocol;

#[cfg(feature = "simulator")]
pub mod simulator;
