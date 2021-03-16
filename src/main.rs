#![no_std]
#![no_main]
#![feature(panic_info_message)]

extern crate cortex_m_rt;
use core::panic::PanicInfo;

use ariko::buzzer::Buzzer;
use ariko::display::Display;
use ariko::i2c::I2C;
use ariko::peripherals::Peripherals;
use ariko::serial::Serial;
use ariko::usb::USB;
use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::InputPin;

#[entry]
unsafe fn main() -> ! {
  Peripherals::init();
  Serial::init(57600);
  I2C::init();
  Display::init();
  USB::init();
  Buzzer::init();

  let lcd = Display::get();
  let serial = Serial::get();
  let buzzer = Buzzer::get();
  lcd.write_str("Started!").unwrap();

  let p = Peripherals::get();

  loop {
    // if p.blue_button.try_is_low().unwrap() {
    //   serial.write_str("Blue button pressed\n");
    // }
    //
    if p.white_button.try_is_low().unwrap() {
      buzzer.beep();
    }

    serial
      .write_fmt(format_args!(
        "SR:     {:#034b}\n\r",
        p.uotghs.sr.read().bits()
      ))
      .unwrap();
    serial
      .write_str("        0b1|987654321|987654321|987654321|\n\r")
      .unwrap();
    serial
      .write_fmt(format_args!(
        "CTRL:   {:#034b}\n\r",
        p.uotghs.ctrl.read().bits()
      ))
      .unwrap();
    serial
      .write_str("        0b1|987654321|987654321|987654321|\n\r")
      .unwrap();
    serial
      .write_fmt(format_args!(
        "HSTISR: {:#034b}\n\r",
        p.uotghs.hstisr.read().bits()
      ))
      .unwrap();
    serial
      .write_str("        0b1|987654321|987654321|987654321|\n\r")
      .unwrap();
    serial
      .write_fmt(format_args!(
        "DEVISR: {:#034b}\n\r",
        p.uotghs.devisr.read().bits()
      ))
      .unwrap();
    serial
      .write_str("        0b1|987654321|987654321|987654321|\n\r")
      .unwrap();
    serial.write_str("\n\r").unwrap();

    p.delay.try_delay_ms(1000_u32).unwrap();
    p.delay.try_delay_ms(1000_u32).unwrap();
    p.delay.try_delay_ms(1000_u32).unwrap();
    p.delay.try_delay_ms(1000_u32).unwrap();
    p.delay.try_delay_ms(1000_u32).unwrap();
  }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  loop {
    Serial::get()
      .write_fmt(format_args!("{}\n", info.message().unwrap()))
      .unwrap();

    Peripherals::get().delay.try_delay_ms(1000_u32).unwrap();
  }
}
