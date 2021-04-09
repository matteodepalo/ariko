#![allow(dead_code)]

mod generic;
mod hid;
mod serial;

use crate::usb::device::generic::{GenericDevice, GenericDeviceClass};
use crate::usb::device::hid::{HIDDevice, HIDDeviceClass};
use crate::usb::device::serial::{SerialDevice, SerialDeviceClass};
use crate::usb::packet::{SetupPacket, SetupRequestDirection, SetupRequestType};
use crate::usb::{Error, USB};
use log::debug;

pub struct DeviceDescriptor;

pub enum Device {
  Serial(SerialDevice),
  Generic(GenericDevice),
  HID(HIDDevice),
}

pub enum DeviceClass {
  Serial(SerialDeviceClass),
  Generic(GenericDeviceClass),
  HID(HIDDeviceClass),
}

enum RequestType {
  GetDescriptor = 6,
  SetAddress = 5,
  SetConfiguration = 9,
}

enum DescriptorType {
  Device = 1,
  Configuration = 2,
  String = 3,
  Interface = 4,
  Endpoint = 5,
}

impl DeviceDescriptor {
  fn new(_buffer: &[u8]) -> Result<Self, Error> {
    Ok(Self)
  }
}

impl Device {
  pub fn poll(&self) -> Result<(), Error> {
    match self {
      Device::Serial(serial) => serial.poll(),
      Device::Generic(generic) => generic.poll(),
      Device::HID(hid) => hid.poll(),
    }
  }

  pub fn release(&self) {
    match self {
      Device::Serial(serial) => serial.release(),
      Device::Generic(generic) => generic.release(),
      Device::HID(hid) => hid.release(),
    }
  }
}

impl DeviceClass {
  pub fn all() -> [DeviceClass; 3] {
    [
      DeviceClass::HID(HIDDeviceClass {}),
      DeviceClass::Serial(SerialDeviceClass {}),
      DeviceClass::Generic(GenericDeviceClass {}),
    ]
  }

  pub fn configure(&self, address: u8) -> Result<Device, Error> {
    let descriptor = self.get_descriptor(0)?;

    self.set_address(0, address)?;
    self.set_configuration(address, 1)?;

    match self {
      DeviceClass::HID(hid) => hid.configure(&self, address, &descriptor),
      DeviceClass::Serial(serial) => serial.configure(&self, address, &descriptor),
      DeviceClass::Generic(generic) => generic.configure(&self, address, &descriptor),
    }
  }

  pub fn get_descriptor(&self, address: u8) -> Result<DeviceDescriptor, Error> {
    let mut buffer = [0_u8; 18];

    let setup_packet = SetupPacket::new(
      SetupRequestType::default(),
      RequestType::GetDescriptor as u8,
      [0, DescriptorType::Device as u8],
      0,
    );

    debug!("[USB :: Device] Get descriptor at address {}", address);

    self
      .usb()
      .control_pipe()
      .control_transfer(address, &setup_packet, Some(&mut buffer))?;

    DeviceDescriptor::new(&buffer)
  }

  pub fn set_address(&self, old_address: u8, new_address: u8) -> Result<(), Error> {
    let setup_packet = SetupPacket::new(
      SetupRequestType::default().with_direction(SetupRequestDirection::HostToDevice),
      RequestType::SetAddress as u8,
      [new_address, 0],
      0,
    );

    debug!(
      "[USB :: Device] Set address {} at address {}",
      new_address, old_address
    );

    self
      .usb()
      .control_pipe()
      .control_transfer(old_address, &setup_packet, None)
  }

  pub fn set_configuration(&self, address: u8, configuration: u8) -> Result<(), Error> {
    let setup_packet = SetupPacket::new(
      SetupRequestType::default().with_direction(SetupRequestDirection::HostToDevice),
      RequestType::SetConfiguration as u8,
      [configuration, 0],
      0,
    );

    debug!(
      "[USB :: Device] Set configuration {} at address {}",
      configuration, address
    );

    self
      .usb()
      .control_pipe()
      .control_transfer(address, &setup_packet, None)
  }

  fn usb(&self) -> &mut USB {
    USB::get()
  }
}
