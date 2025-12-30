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
