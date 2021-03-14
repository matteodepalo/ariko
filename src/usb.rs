use crate::display::Display;
use crate::peripherals::Peripherals;
use core::fmt::Write;
use cortex_m::peripheral::NVIC;
use sam3x8e_hal::pac::interrupt;
use sam3x8e_hal::pac::Interrupt::UOTGHS as I_UOTGHS;
use sam3x8e_hal::pmc::PeripheralClock;

static mut S_USB: Option<USB> = None;

#[interrupt]
unsafe fn UOTGHS() {
  let uotghs = &Peripherals::get().uotghs;
  let lcd = Display::get();

  if uotghs.hstisr.read().ddisci().bit_is_set() {
    lcd.write_str("Disconnected").unwrap();
  }

  if uotghs.hstisr.read().dconni().bit_is_set() {
    lcd.write_str("Connected").unwrap();
  }
}

pub struct USB;

impl USB {
  pub fn init() {
    let peripherals = Peripherals::get();
    let pmc = &mut peripherals.pmc;
    let nvic = &mut peripherals.nvic;
    let uotghs = &mut peripherals.uotghs;
    let ctrl = &uotghs.ctrl;

    pmc.enable_clock(PeripheralClock::UOtgHs);

    unsafe { nvic.set_priority(I_UOTGHS, 0) };
    unsafe { NVIC::unmask(I_UOTGHS) };

    ctrl.write_with_zero(|w| w.uide().clear_bit());
    ctrl.write_with_zero(|w| w.uimod().clear_bit());
    ctrl.write_with_zero(|w| w.vbuspo().set_bit());
    ctrl.write_with_zero(|w| w.otgpade().set_bit());
    ctrl.write_with_zero(|w| w.usbe().set_bit());
    ctrl.write_with_zero(|w| w.frzclk().clear_bit());

    while uotghs.sr.read().clkusable().bit_is_set() {}

    uotghs.hsticr.write_with_zero(|w| {
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

    uotghs
      .scr
      .write_with_zero(|w| unsafe { w.bits(0x1_u32 << 1) });

    ctrl.write_with_zero(|w| w.vbushwc().set_bit().vbuste().set_bit().vberre().set_bit());

    uotghs.sfr.write_with_zero(|w| w.vbusrqs().set_bit());

    if uotghs.sr.read().vbus().bit_is_set() {
      uotghs
        .sfr
        .write_with_zero(|w| unsafe { w.bits(0x1_u32 << 1) });
    }

    uotghs
      .hstier
      .write_with_zero(|w| unsafe { w.bits(0x1_u32 << 0) });

    ctrl.write_with_zero(|w| w.frzclk().set_bit());

    unsafe { S_USB = Some(USB) }
  }

  pub fn get() -> &'static mut Self {
    unsafe { S_USB.as_mut().unwrap() }
  }
}
