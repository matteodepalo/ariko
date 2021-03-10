#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate panic_halt;

use ariko::i2c::I2c;
use ariko::jhd1802::Jhd1802;
use ariko::serial::Serial;
use core::fmt::Write;
use cortex_m_rt::entry;
use sam3x8e_hal::pmc::RcOscillatorSpeed::Speed12Mhz;
use sam3x8e_hal::pmc::{MainOscillator, PeripheralClock};
use sam3x8e_hal::time::Hertz;
use sam3x8e_hal::{pac, pmc::Config, prelude::*};

const LCD_ADDRESS: u8 = 0x3E;

#[entry]
unsafe fn main() -> ! {
  let p = pac::Peripherals::take().unwrap();
  let cp = cortex_m::Peripherals::take().unwrap();

  let mut pmc = p
    .PMC
    .freeze(Config::main_clock(MainOscillator::FastRcOscillator(
      Speed12Mhz,
    )));

  pmc.enable_clock(PeripheralClock::Twi0);

  let _piob = p.PIOB.split(&mut pmc);
  let mut pioa = p.PIOA.split(&mut pmc);

  // SDA1
  pioa
    .pa18
    .disable_pio_line(&mut pioa.pdr)
    .into_peripheral_a(&mut pioa.absr);

  // SCL1
  pioa
    .pa17
    .disable_pio_line(&mut pioa.pdr)
    .into_peripheral_a(&mut pioa.absr);

  // Serial RX
  pioa
    .pa8
    .disable_pio_line(&mut pioa.pdr)
    .into_peripheral_a(&mut pioa.absr);

  // Serial TX
  pioa
    .pa9
    .disable_pio_line(&mut pioa.pdr)
    .into_peripheral_a(&mut pioa.absr);

  let uart = p.UART;
  let mut delay = cp.SYST.delay(pmc.clocks);
  let _serial = Serial::new(Hertz(57600), &mut pmc, uart);
  let i2c = I2c::new(p.TWI0, &pmc.clocks);
  let mut lcd = Jhd1802::new(i2c, LCD_ADDRESS, &mut delay);
  lcd.write_str("Hello, world this is a test").unwrap();
  loop {}
}
