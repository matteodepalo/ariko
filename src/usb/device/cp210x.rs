use crate::usb::device::generic::GenericDevice;
use crate::usb::{Device, Error};

#[derive(Debug)]
pub struct CP210xDevice {
  generic_device: GenericDevice,
}

pub struct CP210xDeviceClass;

impl CP210xDevice {
  pub fn poll(&self) -> Result<(), Error> {
    Ok(())
  }

  pub fn release(&self) {}
}

impl CP210xDeviceClass {
  pub fn configure(&self, generic_device: &mut GenericDevice) -> Result<Device, Error> {
    let device = CP210xDevice {
      generic_device: generic_device.clone(),
    };

    Ok(Device::CP210x(device))
  }
}
