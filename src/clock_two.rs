use crate::peripherals::Peripherals;
use crate::tm1637::TM1637;
use core::fmt;
use core::fmt::Write;
use sam3x8e_hal::gpio::pioc::{PC22, PC23};
use sam3x8e_hal::gpio::{Output, PushPull};

static mut S_CLOCK_TWO: Option<ClockTwo> = None;

pub struct ClockTwo {
  display: TM1637<PC23<Output<PushPull>>, PC22<Output<PushPull>>>,
}

impl ClockTwo {
  pub fn init() {
    let p = Peripherals::get();
    let mut display = TM1637::new(&mut p.clock_two_clk, &mut p.clock_two_data);
    display.clear();
    unsafe { S_CLOCK_TWO = Some(ClockTwo { display }) }
  }

  pub fn get() -> &'static mut Self {
    unsafe { S_CLOCK_TWO.as_mut().unwrap() }
  }
}

impl Write for ClockTwo {
  fn write_str(&mut self, string: &str) -> fmt::Result {
    self.display.display(string);
    Ok(())
  }
}
