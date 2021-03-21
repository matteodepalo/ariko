use crate::usb::{Device, DeviceConfiguration, Error};

#[derive(Copy, Clone)]
pub struct GenericDevice {}

#[derive(Copy, Clone)]
pub struct GenericDeviceClass {}

impl GenericDeviceClass {
  pub fn configure(&self) -> Result<DeviceConfiguration, Error> {
    Ok(DeviceConfiguration::new(Device::Generic(GenericDevice {})))
  }
}
