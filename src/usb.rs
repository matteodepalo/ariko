use crate::peripherals::Peripherals;
use crate::serial::Serial;
use core::fmt::Write;
use cortex_m::peripheral::NVIC;
use sam3x8e_hal::pac::interrupt;
use sam3x8e_hal::pac::Interrupt::UOTGHS as I_UOTGHS;
use sam3x8e_hal::pmc::PeripheralClock;

static mut S_USB: Option<USB> = None;

enum State {
  DetachedStart,
}

#[derive(Copy, Clone)]
struct Device {}

pub struct USB {
  state: State,
  devices: [Option<Device>; 16],
}

impl USB {
  pub fn init() {
    unsafe {
      S_USB = Some(USB {
        state: State::DetachedStart,
        devices: [None; 16],
      })
    }
  }

  pub fn poll(&self) {
    match self.state {
      State::DetachedStart => (),
    }
  }

  fn start(&self) {
    let peripherals = Peripherals::get();
    let nvic = &mut peripherals.nvic;
    let uotghs = &mut peripherals.uotghs;
    let pmc = &mut peripherals.pmc;
    let ctrl = &uotghs.ctrl;

    // Enable USB peripheral clock
    pmc.enable_clock(PeripheralClock::UOtgHs);

    // Always authorize asynchronous USB interrupts to exit sleep mode
    unsafe { nvic.set_priority(I_UOTGHS, 0) };
    unsafe { NVIC::unmask(I_UOTGHS) };

    // Disable ID pin
    ctrl.modify(|_, w| w.uide().clear_bit());

    // Force host mode
    ctrl.modify(|_, w| w.uimod().clear_bit());

    // Set VBOF active high
    ctrl.modify(|_, w| w.vbuspo().set_bit());

    // Enable OTG pad
    ctrl.modify(|_, w| w.otgpade().set_bit());

    // Enable USB macro
    ctrl.modify(|_, w| w.usbe().set_bit());

    // Unfreeze USB clock
    ctrl.modify(|_, w| w.frzclk().clear_bit());

    // Check USB clock
    while !uotghs.sr.read().clkusable().bit_is_set() {}

    // Clear VBus interrupt
    uotghs.scr.write_with_zero(|w| w.vbustic().set_bit());

    // Enable VBus transition and error interrupts
    // automatic VBus control after VBus error
    ctrl.modify(|_, w| w.vbushwc().set_bit().vbuste().set_bit().vberre().set_bit());

    // Requests VBus activation
    uotghs.sfr.write_with_zero(|w| w.vbusrqs().set_bit());

    // Force VBus transition
    uotghs.sfr.write_with_zero(|w| w.vbustis().set_bit());

    // Enable main control interrupt
    // Connection, SOF and reset
    uotghs
      .hstier
      .write_with_zero(|w| w.dconnies().set_bit().hwupies().set_bit());

    // Freeze USB clock
    ctrl.modify(|_, w| w.frzclk().set_bit());
  }

  pub fn get() -> &'static mut Self {
    unsafe { S_USB.as_mut().unwrap() }
  }
}

#[interrupt]
unsafe fn UOTGHS() {
  let _uotghs = &Peripherals::get().uotghs;
  let _serial = Serial::get();
}
