use crate::peripherals::Peripherals;
use crate::usb::device::generic::GenericDevice;
use crate::usb::packet::{
  SetupPacket, SetupRequestDirection, SetupRequestKind, SetupRequestRecipient, SetupRequestType,
};
use crate::usb::pipe::{StreamInPipe, Transfer};
use crate::usb::{Device, Error, USB};
use core::mem::size_of;
use embedded_hal::blocking::delay::DelayMs;
use log::debug;
use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Copy, Clone, Debug)]
pub struct StatusResponse {
  pub errors: u32,
  pub hold_reasons: u32,
  pub amount_in_in_queue: u32,
  pub amount_in_out_queue: u32,
  pub eof_received: u8,
  pub wait_for_immediate: u8,
  pub reserved: u8,
}

static mut IN_PIPE: Option<&StreamInPipe> = None;

#[derive(Debug)]
pub struct CP210xDevice {
  generic_device: GenericDevice,
}

pub struct CP210xDeviceClass;

impl CP210xDevice {
  pub fn poll(&self) -> Result<(), Error> {
    let mut buffer = [0_u8; size_of::<StatusResponse>()];

    let setup_packet = SetupPacket::new(
      SetupRequestType::new()
        .with_direction(SetupRequestDirection::DeviceToHost)
        .with_recipient(SetupRequestRecipient::Device)
        .with_kind(SetupRequestKind::Vendor),
      16,
      [0, 0],
      0,
    );

    debug!("[USB :: Device :: CP210x] Get comm status for {:?}", self);

    self
      .generic_device
      .control(&setup_packet, Some(&mut buffer))?;
    let status_response = StatusResponse::from_bytes(buffer);

    debug!(
      "[USB :: Device :: CP210x] Status response {:?}",
      status_response
    );

    if status_response.amount_in_in_queue() > 0 {
      let mut buffer = [0_u8; 512];

      unsafe { IN_PIPE.unwrap().in_transfer(&mut buffer)? }
      debug!("[USB] Buffer: {:?}", buffer);

      let setup_packet = SetupPacket::new(
        SetupRequestType::new()
          .with_direction(SetupRequestDirection::HostToDevice)
          .with_recipient(SetupRequestRecipient::Interface)
          .with_kind(SetupRequestKind::Vendor),
        18,
        [0x0a, 0],
        0,
      );

      debug!("[USB :: Device :: CP210x] Purged receive queue");

      self.generic_device.control(&setup_packet, None)?;
    }

    unsafe { Peripherals::get().delay.try_delay_ms(1000_u32) };

    Ok(())
  }

  pub fn release(&self) {}
}

impl CP210xDeviceClass {
  pub fn configure(&self, generic_device: &mut GenericDevice) -> Result<Device, Error> {
    let descriptor = generic_device.descriptor().unwrap();

    if descriptor.product_id() == 0xea60 && descriptor.vendor_id() == 0x10c4 {
      generic_device.set_configuration(1)?;

      generic_device.control(
        &SetupPacket::new(
          SetupRequestType::new()
            .with_direction(SetupRequestDirection::HostToDevice)
            .with_recipient(SetupRequestRecipient::Interface)
            .with_kind(SetupRequestKind::Vendor),
          0,
          [0, 0x00],
          0,
        ),
        None,
      )?;

      generic_device.control(
        &SetupPacket::new(
          SetupRequestType::new()
            .with_direction(SetupRequestDirection::HostToDevice)
            .with_recipient(SetupRequestRecipient::Interface)
            .with_kind(SetupRequestKind::Vendor),
          0,
          [0x01, 0],
          0,
        ),
        None,
      )?;

      generic_device.control(
        &SetupPacket::new(
          SetupRequestType::new()
            .with_direction(SetupRequestDirection::HostToDevice)
            .with_recipient(SetupRequestRecipient::Interface)
            .with_kind(SetupRequestKind::Vendor),
          3,
          [0, 0x08],
          0,
        ),
        None,
      )?;

      generic_device.control(
        &SetupPacket::new(
          SetupRequestType::default()
            .with_direction(SetupRequestDirection::HostToDevice)
            .with_recipient(SetupRequestRecipient::Interface)
            .with_kind(SetupRequestKind::Vendor),
          19,
          [0, 0],
          0,
        ),
        Some(&mut [0; 16]),
      )?;

      generic_device.control(
        &SetupPacket::new(
          SetupRequestType::default()
            .with_direction(SetupRequestDirection::HostToDevice)
            .with_recipient(SetupRequestRecipient::Interface)
            .with_kind(SetupRequestKind::Vendor),
          25,
          [0, 0],
          0,
        ),
        Some(&mut [0, 0, 0, 0, 0x11, 0x13]),
      )?;

      generic_device.control(
        &SetupPacket::new(
          SetupRequestType::default()
            .with_direction(SetupRequestDirection::HostToDevice)
            .with_recipient(SetupRequestRecipient::Interface)
            .with_kind(SetupRequestKind::Vendor),
          30,
          [0, 0],
          0,
        ),
        Some(&mut [0, 0xc2, 0x01, 0]),
      )?;

      generic_device.control(
        &SetupPacket::new(
          SetupRequestType::new()
            .with_direction(SetupRequestDirection::HostToDevice)
            .with_recipient(SetupRequestRecipient::Interface)
            .with_kind(SetupRequestKind::Vendor),
          18,
          [0x0f, 0],
          0,
        ),
        None,
      )?;

      let device = CP210xDevice {
        generic_device: generic_device.clone(),
      };

      unsafe {
        IN_PIPE = Some(
          USB::get()
            .alloc_pipe(|p| p.into_stream_in())?
            .configure(generic_device.address, 1, Transfer::Bulk)
            .as_stream_in(),
        )
      };

      Ok(Device::CP210x(device))
    } else {
      Err(Error::DeviceNotSupported)
    }
  }
}
