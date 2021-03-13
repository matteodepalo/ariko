#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate panic_halt;

use ariko::i2c::I2c;
use ariko::jhd1802::Jhd1802;
use ariko::serial::Serial;
use ariko::usb::Usb;
use core::fmt::Write;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use sam3x8e_hal::pac::Interrupt::UOTGHS;
use sam3x8e_hal::pmc::RcOscillatorSpeed::Speed12Mhz;
use sam3x8e_hal::pmc::{MainOscillator, PeripheralClock};
use sam3x8e_hal::time::Hertz;
use sam3x8e_hal::{pac, pmc::Config, prelude::*};

const LCD_ADDRESS: u8 = 0x3E;

#[entry]
unsafe fn main() -> ! {
  let p = pac::Peripherals::take().unwrap();
  let mut cp = cortex_m::Peripherals::take().unwrap();

  let mut pmc = p
    .PMC
    .freeze(Config::main_clock(MainOscillator::FastRcOscillator(
      Speed12Mhz,
    )));

  pmc.enable_clock(PeripheralClock::Twi0);
  pmc.enable_clock(PeripheralClock::UOtgHs);

  &cp.NVIC.set_priority(UOTGHS, 0);
  NVIC::unmask(UOTGHS);

  let ctrl = &p.UOTGHS.ctrl;

  ctrl.write_with_zero(|w| w.uide().clear_bit());
  ctrl.write_with_zero(|w| w.uimod().clear_bit());
  ctrl.write_with_zero(|w| w.vbuspo().set_bit());
  ctrl.write_with_zero(|w| w.otgpade().set_bit());
  ctrl.write_with_zero(|w| w.usbe().set_bit());
  ctrl.write_with_zero(|w| w.frzclk().clear_bit());

  while *&p.UOTGHS.sr.read().clkusable().bit_is_set() {}

  &p.UOTGHS.hsticr.write_with_zero(|w| {
    w.dconnic()
      .set_bit()
      .ddiscic()
      .set_bit()
      .hsofic()
      .set_bit()
      .hwupic()
      .set_bit()
      .rsmedic()
      .set_bit()
      .rstic()
      .set_bit()
      .rxrsmic()
      .set_bit()
  });

  &p.UOTGHS.scr.write_with_zero(|w| w.bits(0x1_u32 << 1));

  ctrl.write_with_zero(|w| w.vbushwc().set_bit().vbuste().set_bit().vberre().set_bit());

  &p.UOTGHS.sfr.write_with_zero(|w| w.vbusrqs().set_bit());

  if p.UOTGHS.sr.read().vbus().bit_is_set() {
    &p.UOTGHS.sfr.write_with_zero(|w| w.bits(0x1_u32 << 1));
  }

  &p.UOTGHS.hstier.write_with_zero(|w| w.bits(0x1_u32 << 0));

  ctrl.write_with_zero(|w| w.frzclk().set_bit());

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
  let serial = Serial::new(Hertz(57600), &mut pmc, uart);
  let i2c = I2c::new(p.TWI0, &pmc.clocks);
  let mut lcd = Jhd1802::new(i2c, LCD_ADDRESS, &mut delay);
  Usb::init(p.UOTGHS, serial);
  loop {}
}
