#![allow(unused_parens)] // modular_bitfield macro generates spurious warnings

use crate::usb::packet::{SetupPacket, SetupRequestDirection, SetupRequestType};
use crate::usb::{Device, Error, USB};
use core::fmt;
use core::mem::size_of;
use log::debug;
use modular_bitfield::prelude::*;

pub struct GenericDeviceClass;

#[bitfield]
#[derive(Copy, Clone, Debug)]
pub struct DeviceDescriptor {
  pub length: u8,
  pub kind: u8,
  pub usb_bcd: u16,
  pub device_class: u8,
  pub device_sub_class: u8,
  pub device_protocol: u8,
  pub max_packet_size: u8,
  pub vendor_id: u16,
  pub product_id: u16,
  pub device_bcd: u16,
  pub manufacturer_index: u8,
  pub product_index: u8,
  pub serial_number_index: u8,
  pub num_configurations: u8,
}

#[derive(Copy, Clone)]
pub struct GenericDevice {
  pub address: u8,
  descriptor: Option<DeviceDescriptor>,
}

impl fmt::Debug for GenericDevice {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("GenericDevice")
      .field("address", &self.address)
      .finish()
  }
}

enum DescriptorType {
  Device = 1,
  Configuration = 2,
  String = 3,
  Interface = 4,
  Endpoint = 5,
}

enum RequestType {
  GetDescriptor = 6,
  SetAddress = 5,
  SetConfiguration = 9,
}

impl GenericDevice {
  pub fn default() -> Self {
    Self {
      address: 0,
      descriptor: None,
    }
  }

  pub fn poll(&self) -> Result<(), Error> {
    Ok(())
  }

  pub fn release(&self) {}

  pub fn descriptor(&mut self) -> Result<&DeviceDescriptor, Error> {
    if self.descriptor.is_none() {
      let mut buffer = [0_u8; size_of::<DeviceDescriptor>()];

      let setup_packet = SetupPacket::new(
        SetupRequestType::default(),
        RequestType::GetDescriptor as u8,
        [0, DescriptorType::Device as u8],
        0,
      );

      debug!("[USB :: Device :: Generic] Get descriptor for {:?}", self);

      self.control(&setup_packet, Some(&mut buffer))?;
      self.descriptor = Some(DeviceDescriptor::from_bytes(buffer))
    }

    Ok(self.descriptor.as_ref().unwrap())
  }

  pub fn set_address(&mut self, address: u8) -> Result<(), Error> {
    let setup_packet = SetupPacket::new(
      SetupRequestType::default().with_direction(SetupRequestDirection::HostToDevice),
      RequestType::SetAddress as u8,
      [address, 0],
      0,
    );

    debug!(
      "[USB :: Device :: Generic] Set address {} for {:?}",
      address, self
    );

    self.control(&setup_packet, None)?;
    self.address = address;

    Ok(())
  }

  pub fn set_configuration(&self, configuration: u8) -> Result<(), Error> {
    let setup_packet = SetupPacket::new(
      SetupRequestType::default().with_direction(SetupRequestDirection::HostToDevice),
      RequestType::SetConfiguration as u8,
      [configuration, 0],
      0,
    );

    debug!(
      "[USB :: Device :: Generic] Set configuration {} for device {:?}",
      configuration, self
    );

    self.control(&setup_packet, None)
  }

  pub fn control(&self, setup_packet: &SetupPacket, data: Option<&mut [u8]>) -> Result<(), Error> {
    USB::with(|usb| {
      usb
        .control_pipe()
        .control_transfer(self.address, setup_packet, data)
    })
  }
}

impl GenericDeviceClass {
  pub fn configure(&self, generic_device: &GenericDevice) -> Result<Device, Error> {
    Ok(Device::Generic(generic_device.clone()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_device_descriptor_size() {
    // USB Device Descriptor is 18 bytes
    assert_eq!(size_of::<DeviceDescriptor>(), 18);
  }

  #[test]
  fn test_device_descriptor_parsing() {
    // Example USB device descriptor bytes (CP210x UART Bridge)
    let bytes: [u8; 18] = [
      0x12,       // bLength (18)
      0x01,       // bDescriptorType (Device)
      0x00, 0x02, // bcdUSB (2.00)
      0x00,       // bDeviceClass
      0x00,       // bDeviceSubClass
      0x00,       // bDeviceProtocol
      0x40,       // bMaxPacketSize0 (64)
      0xc4, 0x10, // idVendor (0x10c4 - Silicon Labs)
      0x60, 0xea, // idProduct (0xea60 - CP210x)
      0x00, 0x01, // bcdDevice (1.00)
      0x01,       // iManufacturer
      0x02,       // iProduct
      0x03,       // iSerialNumber
      0x01,       // bNumConfigurations
    ];

    let descriptor = DeviceDescriptor::from_bytes(bytes);

    assert_eq!(descriptor.length(), 0x12);
    assert_eq!(descriptor.kind(), 0x01);
    assert_eq!(descriptor.usb_bcd(), 0x0200);
    assert_eq!(descriptor.device_class(), 0x00);
    assert_eq!(descriptor.device_sub_class(), 0x00);
    assert_eq!(descriptor.device_protocol(), 0x00);
    assert_eq!(descriptor.max_packet_size(), 64);
    assert_eq!(descriptor.vendor_id(), 0x10c4);
    assert_eq!(descriptor.product_id(), 0xea60);
    assert_eq!(descriptor.device_bcd(), 0x0100);
    assert_eq!(descriptor.manufacturer_index(), 1);
    assert_eq!(descriptor.product_index(), 2);
    assert_eq!(descriptor.serial_number_index(), 3);
    assert_eq!(descriptor.num_configurations(), 1);
  }

  #[test]
  fn test_cp210x_device_identification() {
    // CP210x has vendor_id 0x10c4 and product_id 0xea60
    let bytes: [u8; 18] = [
      0x12, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x40,
      0xc4, 0x10, // vendor_id = 0x10c4
      0x60, 0xea, // product_id = 0xea60
      0x00, 0x01, 0x01, 0x02, 0x03, 0x01,
    ];

    let descriptor = DeviceDescriptor::from_bytes(bytes);

    // This is how CP210x device is identified
    assert_eq!(descriptor.vendor_id(), 0x10c4);
    assert_eq!(descriptor.product_id(), 0xea60);
  }

  #[test]
  fn test_generic_device_default() {
    let device = GenericDevice::default();
    assert_eq!(device.address, 0);
  }

  #[test]
  fn test_device_descriptor_zero_bytes() {
    let bytes: [u8; 18] = [0; 18];
    let descriptor = DeviceDescriptor::from_bytes(bytes);

    assert_eq!(descriptor.length(), 0);
    assert_eq!(descriptor.vendor_id(), 0);
    assert_eq!(descriptor.product_id(), 0);
  }
}
