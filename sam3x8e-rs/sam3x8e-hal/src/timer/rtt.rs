use super::{CountDown, Timer, TimerExt};
use crate::pac::RTT;
use crate::time::Hertz;

static mut EXPIRES_AT: u32 = u32::MAX;

impl Timer<RTT> {
  pub fn rtt<T>(rtt: RTT, timeout: T) -> Self
  where
    T: Into<Hertz>,
  {
    // Configure RTT resolution to approx. 1ms
    unsafe { rtt.mr().write_with_zero(|w| w.rtpres().bits(0x20)); }

    let mut timer = Timer { tim: rtt };

    timer.try_start(timeout).unwrap();
    timer
  }
}

impl CountDown for Timer<RTT> {
  type Error = ();
  type Time = Hertz;

  fn try_start<T>(&mut self, timeout: T) -> Result<(), Self::Error>
  where
    T: Into<Hertz>,
  {
    unsafe { EXPIRES_AT = self.tim.vr().read().bits() + timeout.into().0 };
    Ok(())
  }

  fn try_wait(&mut self) -> nb::Result<(), Self::Error> {
    let expires_at = unsafe { EXPIRES_AT };

    if self.tim.vr().read().bits() >= expires_at {
      Ok(())
    } else {
      Err(nb::Error::WouldBlock)
    }
  }
}

impl TimerExt<RTT> for RTT {
  fn timer<T>(self, timeout: T) -> Timer<RTT>
  where
    T: Into<Hertz>,
  {
    Timer::rtt(self, timeout)
  }
}
