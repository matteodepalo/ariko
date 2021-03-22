use crate::usb::{Device, DeviceClass, Error};

#[derive(Copy, Clone)]
pub struct GenericDevice {}

#[derive(Copy, Clone)]
pub struct GenericDeviceClass {}

impl GenericDeviceClass {
  pub fn configure(&self, class: &DeviceClass, address: u8) -> Result<Device, Error> {
    Ok(Device::Generic(GenericDevice {}))
  }
}
