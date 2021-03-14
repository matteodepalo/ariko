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
use embedded_hal::blocking::delay::DelayMs;

#[entry]
unsafe fn main() -> ! {
  Peripherals::init();
  Serial::init(57600);
  USB::init();
  I2C::init();
  Display::init();

  let lcd = Display::get();
  lcd.write_str("hello").unwrap();

  loop {}
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  loop {
    Serial::get().write_fmt(format_args!("{}\n", info.message().unwrap()));
    Peripherals::get().delay.try_delay_ms(1000);
  }
}
