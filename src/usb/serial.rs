use crate::usb::{Device, Error};

#[derive(Copy, Clone)]
pub struct SerialDevice {}

#[derive(Copy, Clone)]
pub struct SerialDeviceClass {}

impl SerialDeviceClass {
  pub fn configure(&self, _address: u8) -> Result<Device, Error> {
    Ok(Device::Serial(SerialDevice {}))
  }
}
