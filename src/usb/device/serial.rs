use crate::usb::device::DeviceDescriptor;
use crate::usb::{Device, DeviceClass, Error};

pub struct SerialDevice {}
pub struct SerialDeviceClass {}

impl SerialDevice {
  pub fn poll(&self) -> Result<(), Error> {
    Ok(())
  }

  pub fn release(&self) {}
}

impl SerialDeviceClass {
  pub fn configure(
    &self,
    _class: &DeviceClass,
    _address: u8,
    _descriptor: &DeviceDescriptor,
  ) -> Result<Device, Error> {
    Err(Error::DeviceNotSupported)
  }
}
