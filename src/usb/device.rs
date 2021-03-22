mod generic;
mod hid;
mod serial;

use crate::usb::device::generic::{GenericDevice, GenericDeviceClass};
use crate::usb::device::hid::{HIDDevice, HIDDeviceClass};
use crate::usb::device::serial::{SerialDevice, SerialDeviceClass};
use crate::usb::Error;
use modular_bitfield::prelude::*;

#[derive(Copy, Clone)]
pub enum Device {
  Serial(SerialDevice),
  Generic(GenericDevice),
  HID(HIDDevice),
}

#[derive(Copy, Clone)]
pub enum DeviceClass {
  Serial(SerialDeviceClass),
  Generic(GenericDeviceClass),
  HID(HIDDeviceClass),
}

#[bitfield]
#[derive(Copy, Clone)]
struct RequestType {
  recipient: B5,
  value: B2,
  direction: B1,
}

#[repr(C)]
union SetupPacket {
  pub bm_request_type: RequestType,
  pub b_request: u8,
  pub w_value: u16,
  pub w_index: u16,
  pub w_length: u16,
}

impl Device {
  pub fn poll(&self) -> Result<(), Error> {
    Ok(())
  }
  pub fn release(&self) -> Result<(), Error> {
    Ok(())
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
    match self {
      DeviceClass::HID(hid) => hid.configure(&self, address),
      DeviceClass::Serial(serial) => serial.configure(&self, address),
      DeviceClass::Generic(generic) => generic.configure(&self, address),
    }
  }

  pub fn get_descriptor(&self, addr: u32, ep: u32, nbytes: u32, dataptr: *const u8) {
    self.control_request(
      addr,
      ep,
      0, //bmREQ_GET_DESCR,
      0, //USB_REQUEST_GET_DESCRIPTOR,
      0x00,
      0, //USB_DESCRIPTOR_DEVICE,
      0x0000,
      nbytes as u16,
      nbytes,
      dataptr,
    );
  }

  pub fn control_request(
    &self,
    addr: u32,
    ep: u32,
    bmReqType: u8,
    bRequest: u8,
    wValLo: u8,
    wValHi: u8,
    wInd: u16,
    total: u16,
    nbytes: u32,
    dataptr: *const u8,
  ) {
    let mut direction = 0_u32;
    let mut rcode = 0_u32;

    //	// Request direction, IN or OUT
    // 	uint32_t direction = 0;
    // 	uint32_t rcode = 0;
    // 	SETUP_PKT setup_pkt;
    //
    // 	EpInfo *pep = 0;
    // 	uint32_t nak_limit;
    //
    // 	TRACE_USBHOST(printf("    => ctrlReq\r\n");)
    //
    // 	// Set peripheral address
    // 	rcode = setPipeAddress(addr, ep, &pep, nak_limit);
    // 	if (rcode)
    // 		return rcode;
    //
    // 	// Allocate Pipe0 with default 64 bytes size if not already initialized
    // 	// TODO : perform a get device descriptor first to get device endpoint size (else data can be missed if device ep0 > host pipe0)
    // 	rcode = UHD_Pipe0_Alloc(0, 64);
    // 	if (rcode)
    // 	{
    // 		TRACE_USBHOST(printf("/!\\ USBHost::ctrlReq : EP0 allocation error: %lu\r\n", rcode);)
    // 		return (rcode);
    // 	}
    //
    // 	// Determine request direction
    // 	direction = ((bmReqType & 0x80 ) > 0);
    //
    // 	// Fill in setup packet
    //     setup_pkt.ReqType_u.bmRequestType	= bmReqType;
    //     setup_pkt.bRequest					= bRequest;
    //     setup_pkt.wVal_u.wValueLo			= wValLo;
    //     setup_pkt.wVal_u.wValueHi			= wValHi;
    //     setup_pkt.wIndex					= wInd;
    //     setup_pkt.wLength					= total;
    //
    // 	// Configure and write the setup packet into the FIFO
    // 	uhd_configure_pipe_token(0, tokSETUP);
    // 	UHD_Pipe_Write(pep->hostPipeNum, 8, (uint8_t *)&setup_pkt);
    //
    // 	// Dispatch packet
    // 	rcode = dispatchPkt(tokSETUP, pep->hostPipeNum, nak_limit);
    // 	if (rcode)
    // 	{
    // 		// Return HRSLT if not zero
    // 		TRACE_USBHOST(printf("/!\\ USBHost::ctrlReq : Setup packet error: %lu\r\n", rcode);)
    // 		return (rcode);
    // 	}
    //
    // 	// Data stage (if present)
    // 	if (dataptr != 0)
    // 	{
    // 		if (direction)
    // 		{
    // 			// IN transfer
    // 			TRACE_USBHOST(printf("    => ctrlData IN\r\n");)
    // 			uint32_t left = total;
    //
    // 			while (left)
    // 			{
    // 				// Bytes read into buffer
    // 				uint32_t read = nbytes;
    //
    // 				rcode = InTransfer(pep, nak_limit, &read, dataptr);
    // 				if (rcode)
    // 					return rcode;
    //
    // 				// Invoke callback function if inTransfer completed successfuly and callback function pointer is specified
    // 				if (!rcode && p)
    // 					((USBReadParser*)p)->Parse(read, dataptr, total - left);
    //
    // 				left -= read;
    //
    // 				if (read < nbytes)
    // 					break;
    // 			}
    // 		}
    // 		else
    // 		{
    // 			// OUT transfer
    // 			TRACE_USBHOST(printf("    => ctrlData OUT\r\n");)
    // 			rcode = OutTransfer(pep, nak_limit, nbytes, dataptr);
    // 		}
    //
    // 		if (rcode)
    // 		{
    // 			TRACE_USBHOST(printf("/!\\ USBHost::ctrlData : Data packet error: %lu\r\n", rcode);)
    // 			return (rcode);
    // 		}
    // 	}
  }
}
