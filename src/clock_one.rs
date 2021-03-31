use crate::peripherals::Peripherals;
use crate::tm1637::TM1637;
use core::fmt;
use core::fmt::Write;
use sam3x8e_hal::gpio::pioc::{PC24, PC25};
use sam3x8e_hal::gpio::{Output, PushPull};

static mut S_CLOCK_ONE: Option<ClockOne> = None;

pub struct ClockOne {
  display: TM1637<PC25<Output<PushPull>>, PC24<Output<PushPull>>>,
}

impl ClockOne {
  pub fn init() {
    let p = Peripherals::get();
    let mut display = TM1637::new(&mut p.clock_one_clk, &mut p.clock_one_data);
    display.clear();
    unsafe { S_CLOCK_ONE = Some(ClockOne { display }) }
  }

  pub fn get() -> &'static mut Self {
    unsafe { S_CLOCK_ONE.as_mut().unwrap() }
  }
}

impl Write for ClockOne {
  fn write_str(&mut self, string: &str) -> fmt::Result {
    self.display.display(string);
    Ok(())
  }
}
