use crate::usb::{Device, DeviceConfiguration, Error};

#[derive(Copy, Clone)]
pub struct SerialDevice {}

#[derive(Copy, Clone)]
pub struct SerialDeviceClass {}

impl SerialDeviceClass {
  pub fn configure(&self) -> Result<DeviceConfiguration, Error> {
    Ok(DeviceConfiguration::new(Device::Serial(SerialDevice {})))
  }
}
