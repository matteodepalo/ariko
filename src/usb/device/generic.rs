use crate::usb::device::DeviceDescriptor;
use crate::usb::{Device, DeviceClass, Error};

#[derive(Copy, Clone)]
pub struct GenericDevice {}

#[derive(Copy, Clone)]
pub struct GenericDeviceClass {}

impl GenericDevice {
  pub fn poll(&self) -> Result<(), Error> {
    Ok(())
  }
}

impl GenericDeviceClass {
  pub fn configure(
    &self,
    _class: &DeviceClass,
    _address: u8,
    _descriptor: &DeviceDescriptor,
  ) -> Result<Device, Error> {
    Ok(Device::Generic(GenericDevice {}))
  }
}
