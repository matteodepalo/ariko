use crate::jhd1802::JHD1802;
use core::cell::RefCell;
use core::fmt;
use core::fmt::Write;
use critical_section::Mutex;

static DISPLAY: Mutex<RefCell<Option<Display>>> = Mutex::new(RefCell::new(None));

pub struct Display;

impl Display {
  pub fn init() {
    JHD1802::init();
    critical_section::with(|cs| {
      DISPLAY.borrow(cs).replace(Some(Display));
    });
  }

  pub fn with<F, R>(f: F) -> R
  where
    F: FnOnce(&mut Display) -> R,
  {
    critical_section::with(|cs| {
      let mut borrow = DISPLAY.borrow(cs).borrow_mut();
      let display = borrow.as_mut().expect("Display not initialized");
      f(display)
    })
  }
}

impl Write for Display {
  fn write_str(&mut self, string: &str) -> fmt::Result {
    JHD1802::with(|jhd1802| {
      jhd1802.clear();
      jhd1802.set_cursor(0, 0);
      jhd1802.send_str(string);
    });
    Ok(())
  }
}
