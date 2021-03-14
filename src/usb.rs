use crate::display::Display;
use crate::peripherals::Peripherals;
use core::fmt::Write;
use cortex_m::peripheral::NVIC;
use sam3x8e_hal::pac::interrupt;
use sam3x8e_hal::pac::Interrupt::UOTGHS as I_UOTGHS;
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
    let ctrl = &uotghs.ctrl;

    // Always authorize asynchronous USB interrupts to exit of sleep mode
    // For SAM3 USB wake up device except BACKUP mode
    unsafe { nvic.set_priority(I_UOTGHS, 0) };
    unsafe { NVIC::unmask(I_UOTGHS) };

    // ID pin not used then force host mode
    ctrl.write_with_zero(|w| w.uide().clear_bit());
    ctrl.write_with_zero(|w| w.uimod().clear_bit());

    // According to the Arduino Due circuit the VBOF must be active high to power up the remote device
    ctrl.write_with_zero(|w| w.vbuspo().set_bit());

    // Enable OTG pad
    ctrl.write_with_zero(|w| w.otgpade().set_bit());
    // Enable USB macro
    ctrl.write_with_zero(|w| w.usbe().set_bit());

    // Unfreeze internal USB clock
    ctrl.write_with_zero(|w| w.frzclk().clear_bit());

    // Check USB clock
    while !uotghs.sr.read().clkusable().bit_is_set() {}

    // Clear all interrupts that may have been set by a previous host mode
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

    // otg ack vbus transition
    uotghs.scr.write_with_zero(|w| w.vbustic().set_bit());

    // Enable Vbus change and error interrupts
    // Disable automatic Vbus control after Vbus error
    ctrl.write_with_zero(|w| w.vbushwc().set_bit().vbuste().set_bit().vberre().set_bit());

    // Requests VBus activation
    uotghs
      .sfr
      .write_with_zero(|w| unsafe { w.vbusrqs().set_bit() });

    // Force Vbus interrupt when Vbus is always high
    // This is possible due to a short timing between a Host mode stop/start.
    if uotghs.sr.read().vbus().bit_is_set() {
      uotghs.sfr.write_with_zero(|w| w.vbustis().set_bit());
    }

    // Enable main control interrupt
    // Connection, SOF and reset
    uotghs
      .hstier
      .write_with_zero(|w| unsafe { w.dconnies().set_bit() });

    // otg freeze clock
    ctrl.write_with_zero(|w| w.frzclk().set_bit());

    unsafe { S_USB = Some(USB) }
  }

  pub fn get() -> &'static mut Self {
    unsafe { S_USB.as_mut().unwrap() }
  }
}
