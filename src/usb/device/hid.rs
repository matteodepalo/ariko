use crate::usb::device::DeviceDescriptor;
use crate::usb::{Device, DeviceClass, Error};

#[derive(Copy, Clone)]
pub struct HIDDevice {}

#[derive(Copy, Clone)]
pub struct HIDDeviceClass {}

impl HIDDevice {
  pub fn poll(&self) -> Result<(), Error> {
    Ok(())
  }
}

impl HIDDeviceClass {
  pub fn configure(
    &self,
    _class: &DeviceClass,
    _address: u8,
    _descriptor: &DeviceDescriptor,
  ) -> Result<Device, Error> {
    Ok(Device::HID(HIDDevice {}))
  }
}
