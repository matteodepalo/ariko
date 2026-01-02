#![cfg_attr(not(any(test, feature = "simulator")), no_std)]

pub mod app;
pub mod certabo;
pub mod game;

#[cfg(target_arch = "arm")]
pub mod buzzer;
#[cfg(target_arch = "arm")]
pub mod events;
#[cfg(target_arch = "arm")]
pub mod display;
#[cfg(target_arch = "arm")]
pub mod i2c;
#[cfg(target_arch = "arm")]
pub mod jhd1802;
#[cfg(target_arch = "arm")]
pub mod logger;
#[cfg(target_arch = "arm")]
pub mod peripherals;
#[cfg(target_arch = "arm")]
pub mod serial;
#[cfg(target_arch = "arm")]
pub mod tm1637;
#[cfg(target_arch = "arm")]
pub mod usb;
#[cfg(target_arch = "arm")]
pub mod arm_io;

#[cfg(feature = "simulator")]
pub use certabo::board::CertaboBoard;
#[cfg(feature = "simulator")]
pub use certabo::simulator::{SimPiece, SimulatedBoard, VirtualBoard};
