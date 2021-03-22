use crate::usb::{Device, DeviceClass, Error};

#[derive(Copy, Clone)]
pub struct HIDDevice {}

#[derive(Copy, Clone)]
pub struct HIDDeviceClass {}

impl HIDDeviceClass {
  pub fn configure(&self, class: &DeviceClass, address: u8) -> Result<Device, Error> {
    Ok(Device::HID(HIDDevice {}))
  }
}
