use crate::jhd1802::JHD1802;
use core::fmt;
use core::fmt::Write;

static mut S_LCD: Option<Display> = None;

pub struct Display;

impl Display {
  pub fn init() {
    JHD1802::init();
    unsafe { S_LCD = Some(Display) }
  }

  pub fn get() -> &'static mut Self {
    unsafe { S_LCD.as_mut().unwrap() }
  }
}

impl Write for Display {
  fn write_str(&mut self, string: &str) -> fmt::Result {
    JHD1802::get().send_str(string);
    Ok(())
  }
}
