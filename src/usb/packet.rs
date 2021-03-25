use modular_bitfield::prelude::*;

pub struct DataPacket([u8; 8]);

#[derive(BitfieldSpecifier)]
#[bits = 1]
pub enum SetupRequestDirection {
  HostToDevice,
  DeviceToHost,
}

#[derive(BitfieldSpecifier)]
#[bits = 2]
pub enum SetupRequestKind {
  Standard,
  Class,
  Vendor,
}

#[derive(BitfieldSpecifier)]
#[bits = 5]
pub enum SetupRequestRecipient {
  Device,
  Interface,
  Endpoint,
  Other,
}

#[bitfield]
pub struct SetupRequestType {
  recipient: SetupRequestRecipient,
  kind: SetupRequestKind,
  direction: SetupRequestDirection,
}

pub struct SetupPacket {
  pub request_type: SetupRequestType,
  pub request: u8,
  pub value: u16,
  pub index: u16,
  pub length: u16,
}

#[repr(C)]
pub enum Packet {
  Data(DataPacket),
  Setup(SetupPacket),
}

impl SetupRequestType {
  pub fn default() -> Self {
    Self::new()
      .with_recipient(SetupRequestRecipient::Device)
      .with_direction(SetupRequestDirection::DeviceToHost)
      .with_kind(SetupRequestKind::Standard)
  }
}

impl SetupPacket {
  pub fn new(request_type: SetupRequestType, request: u8, value: u16, index: u16) -> Self {
    SetupPacket {
      request_type,
      request,
      value,
      index,
      length: 0,
    }
  }
}
