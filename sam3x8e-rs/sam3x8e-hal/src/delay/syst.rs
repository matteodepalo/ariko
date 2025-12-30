/*
 *    This file (src/delay/syst.rs) is part of sam3x8e-hal.
 *
 *    sam3x8e-hal is free software: you can redistribute it and/or modify
 *    it under the terms of the GNU Lesser General Public License as published
 *    by the Free Software Foundation, either version 3 of the License, or
 *    (at your option) any later version.
 *
 *    sam3x8e-hal is distributed in the hope that it will be useful,
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *    GNU Lesser General Public License for more details.
 *
 *    You should have received a copy of the GNU Lesser General Public License
 *    along with sam3x8e-hal.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::{Delay, DelayExt};
use crate::pmc::{Clocks, MasterClockSrc};
use cortex_m::peripheral::{syst::SystClkSource, SYST};

use hal::delay::DelayNs;

use crate::time::Hertz;

impl DelayExt<SYST> for SYST {
  fn delay(self, clocks: Clocks) -> Delay<SYST> {
    Delay::<SYST>::new(self, clocks)
  }
}

impl Delay<SYST> {
  /// Configures the system timer (SysTick) as a delay provider
  pub fn new(mut source: SYST, clocks: Clocks) -> Self {
    // Set Systick to MCK instead of MCK / 8 §10.22.1
    // TODO: Maybe use MCK / 8 for longer duration timers?
    source.set_clock_source(SystClkSource::Core);

    Delay { source, clocks }
  }

  /// Releases the system timer (SysTick) resource
  pub fn free(self) -> SYST {
    self.source
  }
}

const MILLISECONDS_PER_SECOND: u32 = 1_000;
const MICROSECONDS_PER_SECOND: u32 = 1_000_000;

impl DelayNs for Delay<SYST> {
  fn delay_ns(&mut self, ns: u32) {
    // For nanoseconds, we convert to microseconds (minimum granularity)
    // This loses precision for sub-microsecond delays
    let us = ns / 1000;
    if us > 0 {
      self.delay_us(us);
    }
  }

  fn delay_us(&mut self, us: u32) {
    let frequency: Hertz = self.clocks.processor_clk();

    let reload_value = (us * (frequency.0 / MICROSECONDS_PER_SECOND)).saturating_sub(1);

    // The register is only 24 bits wide
    if reload_value >= (1 << 24) {
      // For long delays, break into chunks
      // Calculate max_us using u64 to avoid overflow: (2^24 * 1_000_000) / freq
      let max_us = (((1_u64 << 24) * MICROSECONDS_PER_SECOND as u64) / frequency.0 as u64) as u32;
      let mut remaining = us;
      while remaining > max_us {
        self.delay_us(max_us);
        remaining -= max_us;
      }
      if remaining > 0 {
        self.delay_us(remaining);
      }
      return;
    }

    if reload_value == 0 {
      return;
    }

    self.source.set_reload(reload_value);
    self.source.clear_current();
    self.source.enable_counter();
    self.source.disable_interrupt();

    while !self.source.has_wrapped() {}

    self.source.disable_counter();
  }

  fn delay_ms(&mut self, ms: u32) {
    if let MasterClockSrc::SlowClock = self.clocks.source() {
      // Too slow to get us precision
      let frequency: Hertz = self.clocks.processor_clk();
      let reload_value = (ms * (frequency.0 / MILLISECONDS_PER_SECOND)).saturating_sub(1);

      // The register is only 24 bits wide
      if reload_value >= (1 << 24) {
        // Break into smaller chunks
        for _ in 0..ms {
          self.delay_ms(1);
        }
        return;
      }

      self.source.set_reload(reload_value);
      self.source.clear_current();
      self.source.enable_counter();
      self.source.disable_interrupt();

      while !self.source.has_wrapped() {}

      self.source.disable_counter();
    } else {
      self.delay_us(ms * 1_000);
    }
  }
}
