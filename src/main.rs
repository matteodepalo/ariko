#![no_std]
#![no_main]
#![allow(dead_code)]

extern crate cortex_m_rt;
use core::panic::PanicInfo;

use certabo::buzzer::Buzzer;
use certabo::display::Display;
use certabo::i2c::I2C;
use certabo::logger::Logger;
use certabo::peripherals::Peripherals;
use certabo::serial::Serial;
use certabo::usb::USB;
use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;

#[entry]
fn main() -> ! {
  Peripherals::init();
  Serial::init(57600);
  Logger::init();
  I2C::init();
  Display::init();
  USB::init();
  Buzzer::init();

  Display::with(|lcd| {
    lcd.write_str("Started!").unwrap();
  });

  loop {
    // Peripherals::with(|p| {
    //   if p.white_button.try_is_low().unwrap() {
    //     Buzzer::with(|buzzer| buzzer.beep());
    //   }
    // });

    USB::with(|usb| usb.poll());
  }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  let location = info.location().unwrap();

  loop {
    Serial::with(|serial| {
      serial
        .write_fmt(format_args!(
          "Panic at {} ({}, {}): {}\n\r",
          location.file(),
          location.line(),
          location.column(),
          info.message()
        ))
        .unwrap();
    });

    Peripherals::with(|p| p.delay.delay_ms(1000_u32));
  }
}
