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
use embedded_hal::delay::DelayNs;
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
  pub fn poll(&self, usb: &USB) -> Result<(), Error> {
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
    debug!("[CP210x] Status raw: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
      status_buffer[0], status_buffer[1], status_buffer[2], status_buffer[3], status_buffer[4],
      status_buffer[5], status_buffer[6], status_buffer[7], status_buffer[8], status_buffer[9]);
    let status_response = StatusResponse::from_bytes(status_buffer);
    let queue_len = status_response.amount_in_in_queue();
    debug!("[CP210x] Queue len: {}, errors: {}", queue_len, status_response.errors());
    if queue_len > 0 {
      let pipe_index = critical_section::with(|cs| *IN_PIPE_INDEX.borrow(cs).borrow())
        .expect("IN_PIPE not configured");

      if let Some(pipe) = usb.get_pipe(pipe_index) {
        // Read in 64-byte chunks (max packet size) since the hardware
        // only handles one packet per IN request
        let mut remaining = queue_len as usize;
        let mut first_bytes = [0u8; 8];
        let mut is_first = true;
        while remaining > 0 {
          let mut chunk_buffer = [0_u8; 64];

          // Read data and get actual byte count from USB
          let bytes_received = pipe.as_stream_in().in_transfer(&mut chunk_buffer)?;

          // Save first 8 bytes for debugging
          if is_first && bytes_received >= 8 {
            first_bytes.copy_from_slice(&chunk_buffer[..8]);
            is_first = false;
          }

          // Push only the bytes we actually received
          critical_section::with(|cs| {
            LINE_BUFFER
              .borrow(cs)
              .borrow_mut()
              .push(&chunk_buffer[..bytes_received]);
          });

          remaining = remaining.saturating_sub(bytes_received);
        }
        // Log first few bytes as both hex and ASCII
        debug!("[CP210x] Hex: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} | ASCII: {} {} {} {} {} {} {} {}",
          first_bytes[0], first_bytes[1], first_bytes[2], first_bytes[3],
          first_bytes[4], first_bytes[5], first_bytes[6], first_bytes[7],
          first_bytes[0] as char, first_bytes[1] as char, first_bytes[2] as char, first_bytes[3] as char,
          first_bytes[4] as char, first_bytes[5] as char, first_bytes[6] as char, first_bytes[7] as char);
      } else {
        return Err(Error::TooManyPipes);
      }
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
  /// Takes 8 bytes, one per rank (byte 0 = rank 8, byte 7 = rank 1).
  /// Each bit represents a file (bit 0 = file a, bit 7 = file h).
  pub fn send_leds(data: &[u8; 8]) -> Result<(), Error> {
    USB::with(|usb| {
      let pipe_index = critical_section::with(|cs| *OUT_PIPE_INDEX.borrow(cs).borrow());

      match pipe_index {
        Some(index) => {
          if let Some(pipe) = usb.get_pipe(index) {
            let mut buf = *data;
            pipe.as_stream_out().out_transfer(&mut buf)?;
            Ok(())
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
  pub fn configure(&self, generic_device: &mut GenericDevice, usb: &mut USB) -> Result<Device, Error> {
    let descriptor = generic_device.descriptor().unwrap();

    if descriptor.product_id() == 0xea60 && descriptor.vendor_id() == 0x10c4 {
      debug!("[CP210x] Starting configuration for VID:PID {:04x}:{:04x}",
        descriptor.vendor_id(), descriptor.product_id());
      generic_device.set_configuration(1)?;
      debug!("[CP210x] Configuration set");

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

      debug!("[CP210x] Setting baud rate to 38400...");

      // Try SET_BAUDDIV first (works on CP2101/2/3)
      // Divisor = 3686400 / baud = 3686400 / 38400 = 96 = 0x0060
      generic_device.control(
        &SetupPacket::new(
          SetupRequestType::default()
            .with_direction(SetupRequestDirection::HostToDevice)
            .with_recipient(SetupRequestRecipient::Interface)
            .with_kind(SetupRequestKind::Vendor),
          1,  // SET_BAUDDIV
          [0x60, 0x00],  // divisor 96 for 38400 baud
          0,
        ),
        None,
      )?;
      debug!("[CP210x] SET_BAUDDIV sent (divisor=96)");

      // Also try SET_BAUDRATE (works on CP2104+)
      generic_device.control(
        &SetupPacket::new(
          SetupRequestType::default()
            .with_direction(SetupRequestDirection::HostToDevice)
            .with_recipient(SetupRequestRecipient::Interface)
            .with_kind(SetupRequestKind::Vendor),
          30,  // SET_BAUDRATE
          [0, 0],
          0,
        ),
        Some(&mut [0x00, 0x96, 0x00, 0x00]),  // 38400 baud (little-endian)
      )?;
      debug!("[CP210x] SET_BAUDRATE sent");

      // Read back baud rate to verify
      let mut baud_readback = [0u8; 4];
      generic_device.control(
        &SetupPacket::new(
          SetupRequestType::default()
            .with_direction(SetupRequestDirection::DeviceToHost)
            .with_recipient(SetupRequestRecipient::Interface)
            .with_kind(SetupRequestKind::Vendor),
          29,  // GET_BAUDRATE
          [0, 0],
          0,
        ),
        Some(&mut baud_readback),
      )?;
      let baud_rate = u32::from_le_bytes(baud_readback);
      debug!("[CP210x] Baud rate readback: {} (raw: {:02x} {:02x} {:02x} {:02x})",
        baud_rate, baud_readback[0], baud_readback[1], baud_readback[2], baud_readback[3]);

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

      // Configure IN pipe for receiving RFID data (endpoint 1)
      let in_pipe = usb.alloc_pipe(|p| p.into_stream_in())?;
      in_pipe.configure(generic_device.address, 1, Transfer::Bulk);
      let in_pipe_index = in_pipe.index();

      critical_section::with(|cs| {
        IN_PIPE_INDEX.borrow(cs).replace(Some(in_pipe_index));
      });

      // Configure OUT pipe for sending LED commands (endpoint 1)
      // CP210x uses endpoint 1 for both IN and OUT (direction determined by pipe type)
      let out_pipe = usb.alloc_pipe(|p| p.into_stream_out())?;
      out_pipe.configure(generic_device.address, 1, Transfer::Bulk);
      let out_pipe_index = out_pipe.index();

      critical_section::with(|cs| {
        OUT_PIPE_INDEX.borrow(cs).replace(Some(out_pipe_index));
      });

      debug!("[USB :: CP210x] Configured IN pipe {} and OUT pipe {}", in_pipe_index, out_pipe_index);

      // Send Certabo initialization sequence (matches official run.py)
      // 1. All LEDs on (0xFF * 8)
      let mut init1 = [0xFFu8; 8];
      out_pipe.as_stream_out().out_transfer(&mut init1)?;
      crate::peripherals::Peripherals::with(|p| p.delay.delay_ms(2000u32));

      // 2. All LEDs off (0x00 * 8)
      let mut init2 = [0x00u8; 8];
      out_pipe.as_stream_out().out_transfer(&mut init2)?;

      debug!("[USB :: CP210x] Sent initialization sequence (LEDs flash)");

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
