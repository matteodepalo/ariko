use crate::usb::device::DeviceDescriptor;
use crate::usb::{Device, DeviceClass, Error};

pub struct GenericDevice {}
pub struct GenericDeviceClass {}

impl GenericDevice {
  pub fn poll(&self) -> Result<(), Error> {
    Ok(())
  }

  pub fn release(&self) {}
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
