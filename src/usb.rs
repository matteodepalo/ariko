use crate::display::Display;
use crate::peripherals::Peripherals;
use core::fmt::Write;
use cortex_m::peripheral::NVIC;
use sam3x8e_hal::pac::interrupt;
use sam3x8e_hal::pac::Interrupt::{PIOB as I_PIOB, UOTGHS as I_UOTGHS};
use sam3x8e_hal::pmc::PeripheralClock;

static mut S_USB: Option<USB> = None;

const UOTGHS_SR_VBUSRQ: u32 = 0x1_u32 << 9;
const UOTGHS_HSTICR_DCONNIC: u32 = 0x1_u32 << 0;

#[interrupt]
unsafe fn UOTGHS() {
  let uotghs = &Peripherals::get().uotghs;
  let lcd = Display::get();

  lcd.write_str("Interrupt!").unwrap();

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
    let nvic = &mut peripherals.nvic;
    let uotghs = &mut peripherals.uotghs;
    let pmc = &mut peripherals.pmc;
    let ctrl = &uotghs.ctrl;

    // Enable USB peripheral clock
    pmc.enable_clock(PeripheralClock::UOtgHs);

    // Freeze internal USB clock
    ctrl.write(|w| w.frzclk().set_bit());

    // ID pin not used then force host mode
    ctrl.write(|w| w.uide().clear_bit());
    ctrl.write(|w| w.uimod().clear_bit());

    // According to the Arduino Due circuit the VBOF must be active high to power up the remote device
    ctrl.write(|w| w.vbuspo().clear_bit());

    // Enable OTG pad
    ctrl.write(|w| w.otgpade().set_bit());

    // Enable USB macro
    ctrl.write(|w| w.usbe().set_bit());

    // Clear VBus transition interrupt
    uotghs.scr.write_with_zero(|w| w.vbustic().set_bit());

    // Enable VBus transition and error interrupts
    // Disable automatic VBus control after VBus error
    ctrl.write(|w| w.vbushwc().set_bit().vbuste().set_bit().vberre().set_bit());

    // Requests VBus activation
    uotghs
      .sfr
      .write_with_zero(|w| unsafe { w.vbusrqs().set_bit() });

    // Enable main control interrupt
    // Connection, SOF and reset
    uotghs
      .hstier
      .write_with_zero(|w| unsafe { w.dconnies().set_bit() });

    // Check USB clock
    while !uotghs.sr.read().clkusable().bit_is_set() {}

    // Unfreeze USB clock
    ctrl.write(|w| w.frzclk().clear_bit());

    // Always authorize asynchronous USB interrupts to exit sleep mode
    unsafe { nvic.set_priority(I_UOTGHS, 0) };
    unsafe { NVIC::unmask(I_UOTGHS) };
    unsafe { S_USB = Some(USB) }
  }

  pub fn get() -> &'static mut Self {
    unsafe { S_USB.as_mut().unwrap() }
  }
}
