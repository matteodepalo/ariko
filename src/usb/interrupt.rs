use crate::peripherals::Peripherals;
use crate::serial::Serial;
use crate::usb::{VBusState, USB};
use core::fmt::Write;
use sam3x8e_hal::pac::interrupt;
use sam3x8e_hal::pac::UOTGHS as P_UOTGHS;

struct Interrupt<'a> {
  uotghs: &'a P_UOTGHS,
  usb: &'a mut USB,
  serial: &'a mut Serial,
}

impl<'a> Interrupt<'a> {
  fn new() -> Self {
    Self {
      uotghs: unsafe { &Peripherals::get().uotghs },
      usb: USB::get(),
      serial: Serial::get(),
    }
  }

  fn handle(&mut self) {
    self
      .serial
      .write_str("[USB] Handling interrupt\n\r")
      .unwrap();

    self
      .serial
      .write_fmt(format_args!("[USB] Is error: {:?}\n\r", self.is_error()))
      .unwrap();

    if self.is_disconnected() {
      self.handle_disconnected()
    } else if self.is_connected() {
      self.handle_connected()
    } else if self.is_vbus_error() {
      self.handle_vbus_error()
    } else {
      self.handle_vbus_enable()
    }
  }

  fn is_disconnected(&self) -> bool {
    self.uotghs.hstisr.read().ddisci().bit_is_set()
      && self.uotghs.hstimr.read().ddiscie().bit_is_set()
  }

  fn is_connected(&self) -> bool {
    self.uotghs.hstisr.read().dconni().bit_is_set()
      && self.uotghs.hstimr.read().dconnie().bit_is_set()
  }

  fn is_error(&self) -> bool {
    self.uotghs.sr.read().vberri().bit_is_set()
      || self.uotghs.sr.read().bcerri().bit_is_set()
      || self.uotghs.sr.read().hnperri().bit_is_set()
      || self.uotghs.sr.read().stoi().bit_is_set()
  }

  fn is_vbus_error(&self) -> bool {
    self.uotghs.sr.read().vberri().bit_is_set()
  }

  fn handle_disconnected(&mut self) {
    self
      .serial
      .write_str("[USB :: Interrupt] Disconnected\n\r")
      .unwrap();

    self
      .uotghs
      .hsticr
      .write_with_zero(|w| w.ddiscic().set_bit());

    self
      .uotghs
      .hstidr
      .write_with_zero(|w| w.ddisciec().set_bit());

    self.uotghs.hstctrl.modify(|_, w| w.reset().clear_bit());

    self
      .uotghs
      .hsticr
      .write_with_zero(|w| w.dconnic().set_bit());

    self
      .uotghs
      .hstier
      .write_with_zero(|w| w.dconnies().set_bit());

    self.usb.set_vbus_state(VBusState::Disconnected)
  }

  fn handle_connected(&mut self) {
    self
      .serial
      .write_str("[USB :: Interrupt] Connected\n\r")
      .unwrap();

    self
      .uotghs
      .hsticr
      .write_with_zero(|w| w.dconnic().set_bit());

    self
      .uotghs
      .hstidr
      .write_with_zero(|w| w.dconniec().set_bit());

    self
      .uotghs
      .hsticr
      .write_with_zero(|w| w.ddiscic().set_bit());

    self
      .uotghs
      .hstier
      .write_with_zero(|w| w.ddiscies().set_bit());

    self.usb.set_vbus_state(VBusState::Connected)
  }

  fn handle_vbus_error(&mut self) {
    self
      .serial
      .write_str("[USB :: Interrupt] VBus Error\n\r")
      .unwrap();

    self.uotghs.scr.write_with_zero(|w| w.vberric().set_bit());
    self.usb.set_vbus_state(VBusState::Disconnected)
  }

  fn handle_vbus_enable(&mut self) {
    self
      .serial
      .write_str("[USB :: Interrupt] VBus Enable\n\r")
      .unwrap();

    while !self.uotghs.sr.read().clkusable().bit_is_set() {}

    self.uotghs.ctrl.modify(|_, w| w.frzclk().clear_bit());

    if self.uotghs.sr.read().vbusti().bit_is_set() {
      self.handle_vbus_disable()
    } else if self.is_error() {
      self.clear_error()
    }
  }

  fn handle_vbus_disable(&mut self) {
    self
      .serial
      .write_str("[USB :: Interrupt] VBus Disable\n\r")
      .unwrap();

    self.uotghs.scr.write_with_zero(|w| w.vbustic().set_bit());

    if self.uotghs.sr.read().vbus().bit_is_set() {
      self.usb.set_vbus_state(VBusState::Disconnected)
    } else {
      self.uotghs.ctrl.modify(|_, w| w.frzclk().set_bit());
      self.usb.set_vbus_state(VBusState::Off)
    }
  }

  fn clear_error(&mut self) {
    self
      .serial
      .write_str("[USB :: Interrupt] Clear Error\n\r")
      .unwrap();

    self.uotghs.scr.write_with_zero(|w| {
      w.vberric()
        .set_bit()
        .bcerric()
        .set_bit()
        .hnperric()
        .set_bit()
        .stoic()
        .set_bit()
    })
  }
}

#[interrupt]
unsafe fn UOTGHS() {
  Interrupt::new().handle();
}
