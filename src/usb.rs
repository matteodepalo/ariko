use crate::jhd1802::Jhd1802;
use crate::serial::Serial;
use core::fmt::Write;
use sam3x8e_hal::pac::interrupt;
use sam3x8e_hal::pac::UOTGHS as P_UOTGHS;

static mut USB: Option<Usb> = None;

#[interrupt]
unsafe fn UOTGHS() {
  let usb = USB.as_mut().unwrap();

  if usb.uotghs.hstisr.read().ddisci().bit_is_set() {
    usb.serial.write_str("Disconnect").unwrap();
  }

  if usb.uotghs.hstisr.read().dconni().bit_is_set() {
    usb.serial.write_str("Connect").unwrap();
  }
}

pub struct Usb {
  serial: Serial,
  uotghs: P_UOTGHS,
}

impl Usb {
  pub fn init(uotghs: P_UOTGHS, serial: Serial) {
    unsafe { USB = Some(Usb { serial, uotghs }) };
  }
}
