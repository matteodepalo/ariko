use crate::usb::{Device, Error};

#[derive(Copy, Clone)]
pub struct GenericDevice {}

#[derive(Copy, Clone)]
pub struct GenericDeviceClass {}

impl GenericDeviceClass {
  pub fn configure(&self, _address: u8) -> Result<Device, Error> {
    Ok(Device::Generic(GenericDevice {}))
  }
}
