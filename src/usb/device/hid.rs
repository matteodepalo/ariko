use crate::usb::device::DeviceDescriptor;
use crate::usb::{Device, DeviceClass, Error};

pub struct HIDDevice {}
pub struct HIDDeviceClass {}

impl HIDDevice {
  pub fn poll(&self) -> Result<(), Error> {
    Ok(())
  }

  pub fn release(&self) {}
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
