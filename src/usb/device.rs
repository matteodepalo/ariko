#![allow(dead_code)]

mod cp210x;
mod generic;

// Re-export CP210xDevice for use in main
pub use cp210x::CP210xDevice;

use cp210x::CP210xDeviceClass;
use generic::{GenericDevice, GenericDeviceClass};
use crate::usb::Error;
use log::debug;

const DEVICE_CLASSES: [DeviceClass; 2] = [
  DeviceClass::CP210x(CP210xDeviceClass),
  DeviceClass::Generic(GenericDeviceClass),
];

#[derive(Debug)]
pub enum Device {
  CP210x(CP210xDevice),
  Generic(GenericDevice),
}

enum DeviceClass {
  CP210x(CP210xDeviceClass),
  Generic(GenericDeviceClass),
}

impl Device {
  pub fn configure(address: u8) -> Result<Self, Error> {
    let mut generic_device = GenericDevice::default();
    let mut result = Err(Error::DeviceNotSupported);

    generic_device.set_address(address)?;

    for device_class in DEVICE_CLASSES.iter() {
      match device_class.configure(&mut generic_device) {
        Ok(device) => {
          debug!("[USB :: Device] Configured: {:?}", device);
          result = Ok(device);
          break;
        }
        Err(Error::DeviceNotSupported) => (),
        Err(error) => {
          result = Err(error);
          break;
        }
      }
    }

    result
  }

  pub fn poll(&self) -> Result<(), Error> {
    match self {
      Device::CP210x(cp210x) => cp210x.poll(),
      Device::Generic(generic) => generic.poll(),
    }
  }

  pub fn release(&self) {
    match self {
      Device::CP210x(cp210x) => cp210x.release(),
      Device::Generic(generic) => generic.release(),
    }
  }
}

impl DeviceClass {
  fn configure(&self, generic_device: &mut GenericDevice) -> Result<Device, Error> {
    match self {
      DeviceClass::CP210x(cp210x) => cp210x.configure(generic_device),
      DeviceClass::Generic(generic) => generic.configure(generic_device),
    }
  }
}
