#![no_std]
#![no_main]
#![feature(panic_info_message)]

extern crate cortex_m_rt;
use core::panic::PanicInfo;

use ariko::display::Display;
use ariko::i2c::I2C;
use ariko::jhd1802::JHD1802;
use ariko::peripherals::Peripherals;
use ariko::serial::Serial;
use ariko::usb::USB;
use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::digital::InputPin;

#[entry]
unsafe fn main() -> ! {
  cortex_m::interrupt::enable();

  Peripherals::init();
  Serial::init(57600);
  I2C::init();
  Display::init();
  USB::init();

  let lcd = Display::get();
  lcd.write_str("hello").unwrap();

  let p = Peripherals::get();

  loop {
    if p.button.try_is_low().unwrap() {
      Serial::get().write_str("pressed");
    }
  }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  loop {
    Serial::get().write_fmt(format_args!("{}\n", info.message().unwrap()));
    Peripherals::get().delay.try_delay_ms(1000_u32);
  }
}
