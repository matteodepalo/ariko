//! Atomic event flags for ISR → main loop communication.
//!
//! Interrupt handlers set these flags; the main loop consumes them.

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Blue button (calibrate) was pressed.
pub static BLUE_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

/// White button (pause/resume) was pressed.
pub static WHITE_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

/// Timer tick occurred (100ms interval).
pub static TIMER_TICK: AtomicBool = AtomicBool::new(false);

/// Monotonic millisecond counter, incremented by timer ISR.
pub static MILLIS: AtomicU32 = AtomicU32::new(0);

/// Consume a flag atomically, returning true if it was set.
#[inline]
pub fn consume(flag: &AtomicBool) -> bool {
    flag.swap(false, Ordering::SeqCst)
}
