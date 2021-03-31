#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![allow(dead_code)]

extern crate cortex_m_rt;
use core::panic::PanicInfo;

use ariko::buzzer::Buzzer;
use ariko::clock_one::ClockOne;
use ariko::clock_two::ClockTwo;
use ariko::display::Display;
use ariko::i2c::I2C;
use ariko::peripherals::Peripherals;
use ariko::serial::Serial;
use ariko::usb::USB;
use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;

#[entry]
unsafe fn main() -> ! {
  Peripherals::init();
  Serial::init(57600);
  I2C::init();
  Display::init();
  USB::init();
  Buzzer::init();
  ClockOne::init();
  ClockTwo::init();

  let usb = USB::get();
  let lcd = Display::get();
  let _buzzer = Buzzer::get();
  let clock_one = ClockOne::get();
  let clock_two = ClockTwo::get();

  lcd.write_str("Started!").unwrap();
  clock_one.write_str("1");
  clock_two.write_str("2");

  let _p = Peripherals::get();

  loop {
    // if p.white_button.try_is_low().unwrap() {
    //   buzzer.beep()
    // };

    // usb.poll()
  }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  let location = info.location().unwrap();

  loop {
    Serial::get()
      .write_fmt(format_args!(
        "Panic at {} ({}, {}): {}\n",
        location.file(),
        location.line(),
        location.column(),
        info.message().unwrap()
      ))
      .unwrap();

    Peripherals::get().delay.try_delay_ms(1000_u32).unwrap();
  }
}
