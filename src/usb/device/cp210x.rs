#![allow(unused_parens)] // modular_bitfield macro generates spurious warnings

use crate::certabo::buffer::LineBuffer;
use crate::usb::device::generic::GenericDevice;
use crate::usb::packet::{
  SetupPacket, SetupRequestDirection, SetupRequestKind, SetupRequestRecipient, SetupRequestType,
};
use crate::usb::pipe::Transfer;
use crate::usb::{Device, Error, USB};
use core::cell::RefCell;
use core::mem::size_of;
use critical_section::Mutex;
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

// Store pipe indices
static IN_PIPE_INDEX: Mutex<RefCell<Option<u8>>> = Mutex::new(RefCell::new(None));
static OUT_PIPE_INDEX: Mutex<RefCell<Option<u8>>> = Mutex::new(RefCell::new(None));

// Line buffer for accumulating RFID data
static LINE_BUFFER: Mutex<RefCell<LineBuffer>> = Mutex::new(RefCell::new(LineBuffer::new()));

#[derive(Debug)]
pub struct CP210xDevice {
  generic_device: GenericDevice,
}

pub struct CP210xDeviceClass;

impl CP210xDevice {
  pub fn poll(&self) -> Result<(), Error> {
    let mut status_buffer = [0_u8; size_of::<StatusResponse>()];

    let setup_packet = SetupPacket::new(
      SetupRequestType::new()
        .with_direction(SetupRequestDirection::DeviceToHost)
        .with_recipient(SetupRequestRecipient::Device)
        .with_kind(SetupRequestKind::Vendor),
      16,
      [0, 0],
      0,
    );

    self
      .generic_device
      .control(&setup_packet, Some(&mut status_buffer))?;
    let status_response = StatusResponse::from_bytes(status_buffer);

    let queue_len = status_response.amount_in_in_queue();
    if queue_len > 0 {
      let mut data_buffer = [0_u8; 512];
      let bytes_to_read = (queue_len as usize).min(512);

      USB::with(|usb| {
        let pipe_index = critical_section::with(|cs| *IN_PIPE_INDEX.borrow(cs).borrow())
          .expect("IN_PIPE not configured");

        if let Some(pipe) = usb.get_pipe(pipe_index) {
          pipe.as_stream_in().in_transfer(&mut data_buffer[..bytes_to_read])
        } else {
          Err(Error::TooManyPipes)
        }
      })?;

      // Push received data to line buffer
      critical_section::with(|cs| {
        LINE_BUFFER
          .borrow(cs)
          .borrow_mut()
          .push(&data_buffer[..bytes_to_read]);
      });

      debug!("[USB :: CP210x] Received {} bytes", bytes_to_read);
    }

    Ok(())
  }

  pub fn release(&self) {}
}

impl CP210xDevice {
  /// Check if a complete line is available from the Certabo board
  pub fn has_line() -> bool {
    critical_section::with(|cs| LINE_BUFFER.borrow(cs).borrow().has_complete_line())
  }

  /// Read a complete line from the Certabo board
  ///
  /// Returns the line data and length if available.
  pub fn read_line(out: &mut [u8]) -> Option<usize> {
    critical_section::with(|cs| LINE_BUFFER.borrow(cs).borrow_mut().take_line_into(out))
  }

  /// Send LED state to the Certabo board
  ///
  /// Takes 8 bytes, one per file (a-h), where each bit represents a rank.
  pub fn send_leds(data: &[u8; 8]) -> Result<(), Error> {
    USB::with(|usb| {
      let pipe_index = critical_section::with(|cs| *OUT_PIPE_INDEX.borrow(cs).borrow());

      match pipe_index {
        Some(index) => {
          if let Some(pipe) = usb.get_pipe(index) {
            let mut buf = *data;
            pipe.as_stream_out().out_transfer(&mut buf)
          } else {
            Err(Error::TooManyPipes)
          }
        }
        None => {
          debug!("[USB :: CP210x] OUT pipe not configured");
          Ok(()) // Silently ignore if not configured
        }
      }
    })
  }

  /// Clear the line buffer
  pub fn clear_buffer() {
    critical_section::with(|cs| LINE_BUFFER.borrow(cs).borrow_mut().clear());
  }
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
        Some(&mut [0x00, 0x96, 0x00, 0x00]),  // 38400 baud for Certabo
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

      USB::with(|usb| {
        // Configure IN pipe for receiving RFID data (endpoint 1)
        let in_pipe = usb.alloc_pipe(|p| p.into_stream_in())?;
        in_pipe.configure(generic_device.address, 1, Transfer::Bulk);
        let in_pipe_index = in_pipe.index();

        critical_section::with(|cs| {
          IN_PIPE_INDEX.borrow(cs).replace(Some(in_pipe_index));
        });

        // Configure OUT pipe for sending LED commands (endpoint 2)
        let out_pipe = usb.alloc_pipe(|p| p.into_stream_out())?;
        out_pipe.configure(generic_device.address, 2, Transfer::Bulk);
        let out_pipe_index = out_pipe.index();

        critical_section::with(|cs| {
          OUT_PIPE_INDEX.borrow(cs).replace(Some(out_pipe_index));
        });

        debug!("[USB :: CP210x] Configured IN pipe {} and OUT pipe {}", in_pipe_index, out_pipe_index);

        Ok::<(), Error>(())
      })?;

      Ok(Device::CP210x(device))
    } else {
      Err(Error::DeviceNotSupported)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_status_response_size() {
    // StatusResponse should be 19 bytes based on the bitfield definition
    // 4 + 4 + 4 + 4 + 1 + 1 + 1 = 19 bytes
    assert_eq!(size_of::<StatusResponse>(), 19);
  }

  #[test]
  fn test_status_response_empty_queue() {
    // All zeros - empty queue, no errors
    let bytes: [u8; 19] = [0; 19];
    let response = StatusResponse::from_bytes(bytes);

    assert_eq!(response.errors(), 0);
    assert_eq!(response.hold_reasons(), 0);
    assert_eq!(response.amount_in_in_queue(), 0);
    assert_eq!(response.amount_in_out_queue(), 0);
    assert_eq!(response.eof_received(), 0);
    assert_eq!(response.wait_for_immediate(), 0);
  }

  #[test]
  fn test_status_response_with_data_in_queue() {
    // Create a response with some data in the input queue
    let mut bytes: [u8; 19] = [0; 19];
    // Set amount_in_in_queue to 64 (bytes 8-11, little-endian)
    bytes[8] = 64;
    bytes[9] = 0;
    bytes[10] = 0;
    bytes[11] = 0;

    let response = StatusResponse::from_bytes(bytes);

    assert_eq!(response.amount_in_in_queue(), 64);
    assert_eq!(response.amount_in_out_queue(), 0);
  }

  #[test]
  fn test_status_response_with_errors() {
    let mut bytes: [u8; 19] = [0; 19];
    // Set errors field (bytes 0-3, little-endian) to 0x01 (some error)
    bytes[0] = 0x01;

    let response = StatusResponse::from_bytes(bytes);

    assert_eq!(response.errors(), 1);
  }

  #[test]
  fn test_status_response_eof_received() {
    let mut bytes: [u8; 19] = [0; 19];
    // Set eof_received (byte 16) to 1
    bytes[16] = 1;

    let response = StatusResponse::from_bytes(bytes);

    assert_eq!(response.eof_received(), 1);
  }

  #[test]
  fn test_status_response_large_queue() {
    let mut bytes: [u8; 19] = [0; 19];
    // Set amount_in_in_queue to 1024 (0x400)
    bytes[8] = 0x00;
    bytes[9] = 0x04;
    bytes[10] = 0x00;
    bytes[11] = 0x00;

    let response = StatusResponse::from_bytes(bytes);

    assert_eq!(response.amount_in_in_queue(), 1024);
  }
}
