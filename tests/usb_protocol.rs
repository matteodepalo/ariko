//! Integration tests for USB protocol handling
//!
//! These tests verify USB protocol flows using mock data,
//! testing packet construction and device identification logic.

use certabo::usb::packet::{
    SetupPacket, SetupRequestDirection, SetupRequestKind, SetupRequestRecipient, SetupRequestType,
};
use core::mem::size_of;

/// USB Standard Request Codes
mod request_codes {
    pub const GET_STATUS: u8 = 0x00;
    pub const CLEAR_FEATURE: u8 = 0x01;
    pub const SET_FEATURE: u8 = 0x03;
    pub const SET_ADDRESS: u8 = 0x05;
    pub const GET_DESCRIPTOR: u8 = 0x06;
    pub const SET_DESCRIPTOR: u8 = 0x07;
    pub const GET_CONFIGURATION: u8 = 0x08;
    pub const SET_CONFIGURATION: u8 = 0x09;
}

/// USB Descriptor Types
mod descriptor_types {
    pub const DEVICE: u8 = 0x01;
    pub const CONFIGURATION: u8 = 0x02;
    pub const STRING: u8 = 0x03;
    pub const INTERFACE: u8 = 0x04;
    pub const ENDPOINT: u8 = 0x05;
}

#[test]
fn test_get_device_descriptor_request() {
    // Standard GET_DESCRIPTOR request for device descriptor
    let request_type = SetupRequestType::new()
        .with_recipient(SetupRequestRecipient::Device)
        .with_kind(SetupRequestKind::Standard)
        .with_direction(SetupRequestDirection::DeviceToHost);

    let packet = SetupPacket::new(
        request_type,
        request_codes::GET_DESCRIPTOR,
        [0x00, descriptor_types::DEVICE], // wValue: descriptor index 0, type DEVICE
        0x0000,                            // wIndex: zero for device descriptor
    );

    // Verify the packet structure
    assert_eq!(packet.request, request_codes::GET_DESCRIPTOR);
    assert_eq!(packet.value[1], descriptor_types::DEVICE);
    assert_eq!(packet.index, 0);

    // Verify bmRequestType byte
    assert_eq!(packet.request_type.into_bytes()[0], 0x80);
}

#[test]
fn test_get_configuration_descriptor_request() {
    let request_type = SetupRequestType::new()
        .with_recipient(SetupRequestRecipient::Device)
        .with_kind(SetupRequestKind::Standard)
        .with_direction(SetupRequestDirection::DeviceToHost);

    let packet = SetupPacket::new(
        request_type,
        request_codes::GET_DESCRIPTOR,
        [0x00, descriptor_types::CONFIGURATION],
        0x0000,
    );

    assert_eq!(packet.value[1], descriptor_types::CONFIGURATION);
}

#[test]
fn test_set_address_request() {
    // SET_ADDRESS is host-to-device, no data phase
    let request_type = SetupRequestType::new()
        .with_recipient(SetupRequestRecipient::Device)
        .with_kind(SetupRequestKind::Standard)
        .with_direction(SetupRequestDirection::HostToDevice);

    let address: u8 = 5;
    let packet = SetupPacket::new(
        request_type,
        request_codes::SET_ADDRESS,
        [address, 0x00], // wValue: device address in low byte
        0x0000,
    );

    assert_eq!(packet.request, request_codes::SET_ADDRESS);
    assert_eq!(packet.value[0], 5);
    assert_eq!(packet.request_type.into_bytes()[0], 0x00); // Host-to-device, standard, device
}

#[test]
fn test_set_configuration_request() {
    let request_type = SetupRequestType::new()
        .with_recipient(SetupRequestRecipient::Device)
        .with_kind(SetupRequestKind::Standard)
        .with_direction(SetupRequestDirection::HostToDevice);

    let configuration: u8 = 1;
    let packet = SetupPacket::new(
        request_type,
        request_codes::SET_CONFIGURATION,
        [configuration, 0x00],
        0x0000,
    );

    assert_eq!(packet.request, request_codes::SET_CONFIGURATION);
    assert_eq!(packet.value[0], 1);
}

#[test]
fn test_vendor_specific_request() {
    // CP210x uses vendor-specific requests
    let request_type = SetupRequestType::new()
        .with_recipient(SetupRequestRecipient::Interface)
        .with_kind(SetupRequestKind::Vendor)
        .with_direction(SetupRequestDirection::HostToDevice);

    let packet = SetupPacket::new(
        request_type,
        0x00, // IFC_ENABLE
        [0x01, 0x00], // Enable
        0x0000,
    );

    assert!(matches!(
        packet.request_type.kind(),
        SetupRequestKind::Vendor
    ));
    assert!(matches!(
        packet.request_type.recipient(),
        SetupRequestRecipient::Interface
    ));
    // 0x41 = Interface (1) | Vendor (2 << 5) | HostToDevice (0 << 7)
    assert_eq!(packet.request_type.into_bytes()[0], 0x41);
}

#[test]
fn test_class_specific_request() {
    // Example: HID class request
    let request_type = SetupRequestType::new()
        .with_recipient(SetupRequestRecipient::Interface)
        .with_kind(SetupRequestKind::Class)
        .with_direction(SetupRequestDirection::DeviceToHost);

    let packet = SetupPacket::new(
        request_type,
        0x01, // GET_REPORT
        [0x01, 0x01], // Report type and ID
        0x0000,
    );

    assert!(matches!(
        packet.request_type.kind(),
        SetupRequestKind::Class
    ));
    // 0xA1 = Interface (1) | Class (1 << 5) | DeviceToHost (1 << 7)
    assert_eq!(packet.request_type.into_bytes()[0], 0xA1);
}

#[test]
fn test_setup_packet_is_8_bytes() {
    // All USB setup packets must be exactly 8 bytes
    assert_eq!(size_of::<SetupPacket>(), 8);
}

#[test]
fn test_device_enumeration_sequence() {
    // Test the typical device enumeration packet sequence

    // 1. Get Device Descriptor (first 8 bytes to learn max packet size)
    let get_desc = SetupPacket::new(
        SetupRequestType::default(),
        request_codes::GET_DESCRIPTOR,
        [0x00, descriptor_types::DEVICE],
        0x0000,
    );
    assert_eq!(get_desc.request_type.into_bytes()[0], 0x80);

    // 2. Set Address
    let set_addr = SetupPacket::new(
        SetupRequestType::default().with_direction(SetupRequestDirection::HostToDevice),
        request_codes::SET_ADDRESS,
        [0x01, 0x00], // Address 1
        0x0000,
    );
    assert_eq!(set_addr.request_type.into_bytes()[0], 0x00);

    // 3. Get full Device Descriptor
    let get_full_desc = SetupPacket::new(
        SetupRequestType::default(),
        request_codes::GET_DESCRIPTOR,
        [0x00, descriptor_types::DEVICE],
        0x0000,
    );
    assert_eq!(get_full_desc.request, request_codes::GET_DESCRIPTOR);

    // 4. Get Configuration Descriptor
    let get_config = SetupPacket::new(
        SetupRequestType::default(),
        request_codes::GET_DESCRIPTOR,
        [0x00, descriptor_types::CONFIGURATION],
        0x0000,
    );
    assert_eq!(get_config.value[1], descriptor_types::CONFIGURATION);

    // 5. Set Configuration
    let set_config = SetupPacket::new(
        SetupRequestType::default().with_direction(SetupRequestDirection::HostToDevice),
        request_codes::SET_CONFIGURATION,
        [0x01, 0x00], // Configuration 1
        0x0000,
    );
    assert_eq!(set_config.request, request_codes::SET_CONFIGURATION);
}
