#[cfg(target_arch = "arm")]
use crate::usb::{CP210xDevice, Error, USB};

pub trait CertaboBoard {
    fn poll(&mut self);
    fn has_line(&self) -> bool;
    fn read_line(&mut self, out: &mut [u8]) -> Option<usize>;
    fn send_leds(&mut self, data: &[u8; 8]) -> Result<(), BoardError>;
    fn clear_buffer(&mut self);
}

#[derive(Debug)]
pub enum BoardError {
    #[cfg(target_arch = "arm")]
    Usb(Error),
}

#[cfg(target_arch = "arm")]
impl From<Error> for BoardError {
    fn from(e: Error) -> Self {
        BoardError::Usb(e)
    }
}

#[cfg(target_arch = "arm")]
pub struct PhysicalBoard;

#[cfg(target_arch = "arm")]
impl PhysicalBoard {
    pub fn new() -> Self {
        PhysicalBoard
    }
}

#[cfg(target_arch = "arm")]
impl Default for PhysicalBoard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_arch = "arm")]
impl CertaboBoard for PhysicalBoard {
    fn poll(&mut self) {
        USB::with(|usb| usb.poll());
    }

    fn has_line(&self) -> bool {
        CP210xDevice::has_line()
    }

    fn read_line(&mut self, out: &mut [u8]) -> Option<usize> {
        CP210xDevice::read_line(out)
    }

    fn send_leds(&mut self, data: &[u8; 8]) -> Result<(), BoardError> {
        CP210xDevice::send_leds(data)?;
        Ok(())
    }

    fn clear_buffer(&mut self) {
        CP210xDevice::clear_buffer()
    }
}
