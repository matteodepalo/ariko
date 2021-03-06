/*
 *    This file (src/pmc.rs) is part of sam3x8e-hal.
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

use crate::pac::PMC;
use crate::time::*;
use sam3x8e::pmc::ckgr_mor::MOSCRCF_A;

#[derive(Clone, Copy)]
pub enum RcOscillatorSpeed {
  Speed4Mhz,
  Speed8Mhz,
  Speed12Mhz,
}

/// The xtal oscillator is an external device and can be any frequency
/// between 3 and 20 MHz.  However there are many assumptions baked into
/// having xtal run at 12 MHz.
#[cfg(feature = "xtal-12mhz")]
pub const XTAL_SPEED: MegaHertz = MegaHertz(12);

pub const SLOW_CLOCK_SPEED: Hertz = Hertz(32_768);

#[derive(Clone, Copy)]
pub enum MainOscillator {
  /// A 3 to 20 MHz Crystal or Ceramic Resonator-based Oscillator, which can
  /// be bypassed. A.K.A XTAL.  Typically this is 12 MHz and the xtal-12mhz feature
  /// is set.  A non-12 MHz crystal will require additional work and will preclude
  /// USB from working.
  XtalOscillator,

  /// A factory programmed Fast RC Oscillator. 3 output frequencies can be
  /// selected: 4, 8 or 12 MHz. By default / at boot 4 MHz is selected.
  FastRcOscillator(RcOscillatorSpeed),
}

#[derive(Clone, Copy)]
pub enum DivA {
  Zero,       // 0
  Bypassed,   // 1
  Output(u8), // 2-255
}

#[derive(Clone, Copy)]
pub enum MulA {
  Deactivated,    // 0
  Activated(u16), // 11 bits wide (1-2047)
}

/// System clock source
#[derive(Clone, Copy)]
pub enum MasterClockSrc {
  /// A Low Power 32,768 Hz Slow Clock Oscillator with bypass mode.  This is
  /// the only permanent clock within the system.  Technically there are
  /// a variety of inputs it can use. TODO: implement them.
  SlowClock,

  /// MAINCK is the output of the Main Clock Oscillator selection: either the
  /// Crystal or Ceramic Resonator-based Oscillator or 4/8/12 MHz Fast RC
  /// Oscillator.
  MainClock(MainOscillator),

  /// PLLACK is the output of the Divider and 96 to 192 MHz programmable
  /// PLL (PLLA).
  Pll(MainOscillator, DivA, MulA),
}

pub enum ProcessorClockPrescaler {
  Clk,
  Clk2,  // Master clock / 2
  Clk3,  // Master clock / 3
  Clk4,  // Master clock / 4
  Clk8,  // Master clock / 8
  Clk16, // Master clock / 16
  Clk32, // Master clock / 32
  Clk64, // Master clock / 64
}

/// PLL divisor
pub enum PllDivMode {
  DividedBy1,
  DividedBy2,
}

/// Clock configutation runtime constants
pub struct Config {
  css: MasterClockSrc,
  pres: ProcessorClockPrescaler,
  plla_div2: PllDivMode,
  upll_div2: PllDivMode,
}

// This is not how the board comes up by default
impl Default for Config {
  fn default() -> Config {
    Config {
      css: MasterClockSrc::SlowClock,
      pres: ProcessorClockPrescaler::Clk,
      plla_div2: PllDivMode::DividedBy1,
      upll_div2: PllDivMode::DividedBy1,
    }
  }
}

impl Config {
  /// Returns the master clock source
  pub fn master_clock(mut self, css: MasterClockSrc) -> Self {
    self.css = css;
    self
  }

  /// Returns the processor prescaler divisor
  pub fn prescaler(mut self, pres: ProcessorClockPrescaler) -> Self {
    self.pres = pres;
    self
  }

  /// Configures the PLLA (general purpose PLL) divisor
  pub fn plla_div(mut self, plla_div2: PllDivMode) -> Self {
    self.plla_div2 = plla_div2;
    self
  }

  /// Configures the UPLL (USB PLL) divisor
  pub fn upll_div(mut self, upll_div2: PllDivMode) -> Self {
    self.upll_div2 = upll_div2;
    self
  }

  /// Generates a clock config with PLLA as the source
  pub fn pll(pll_src: MainOscillator, pll_div: DivA, pll_mul: MulA) -> Config {
    Config {
      css: MasterClockSrc::Pll(pll_src, pll_div, pll_mul),
      pres: ProcessorClockPrescaler::Clk,
      plla_div2: PllDivMode::DividedBy1,
      upll_div2: PllDivMode::DividedBy1,
    }
  }

  /// Generates a clock config with the main clock as the source
  pub fn main_clock(oscillator: MainOscillator) -> Config {
    Config {
      css: MasterClockSrc::MainClock(oscillator),
      pres: ProcessorClockPrescaler::Clk,
      plla_div2: PllDivMode::DividedBy1,
      upll_div2: PllDivMode::DividedBy1,
    }
  }

  /// Generates a clock config with the slow clock as the source.  Roughly 32 KHz.
  pub fn slow_clock() -> Config {
    Config {
      css: MasterClockSrc::SlowClock,
      pres: ProcessorClockPrescaler::Clk,
      plla_div2: PllDivMode::DividedBy1,
      upll_div2: PllDivMode::DividedBy1,
    }
  }

  #[cfg(feature = "xtal-12mhz")]
  /// Generates a clock config with PLLA as the source and a processor clock speed of 84 MHz.
  /// This is essentially the maximum stable speed for a SAM3x8e that also allows for useful
  /// USART and USB I/O.
  pub fn hclk_84mhz() -> Config {
    Config {
      css: MasterClockSrc::Pll(
        MainOscillator::XtalOscillator,
        DivA::Bypassed,
        MulA::Activated(13),
      ),
      pres: ProcessorClockPrescaler::Clk2,
      plla_div2: PllDivMode::DividedBy1,
      upll_div2: PllDivMode::DividedBy1,
    }
  }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can (should) no longer be changed
#[derive(Clone, Copy)]
pub struct Clocks {
  source: MasterClockSrc,

  processor_clock: Hertz, // HCLK
  master_clock: Hertz,    // MCK
}

impl Clocks {
  /// Returns the master clock (MCK) speed in hertz
  pub fn master_clk(&self) -> Hertz {
    self.master_clock
  }

  /// Returns the processor clock (HCLK) speed in hertz
  pub fn processor_clk(&self) -> Hertz {
    self.processor_clock
  }

  pub fn source(&self) -> MasterClockSrc {
    self.source
  }
}

/// PMC (Power Management Controller) peripheral
pub struct Pmc {
  pub clocks: Clocks,
  pub(crate) pmc: PMC,
}

pub enum PeripheralClock {
  PioA,
  PioB,
  PioC,
  PioD,
  Usart0,
  Usart1,
  Usart2,
  Usart3,
  HsMci,
  Twi0,
  Twi1,
  Spi0,
  Ssc,
  Tc0,
  Tc1,
  Tc2,
  Tc3,
  Tc4,
  Tc5,
  Tc6,
  Tc7,
  Tc8,
  Pwm,
  Adc,
  Dacc,
  Dmac,
  UOtgHs,
  Trng,
  Emac,
  Can0,
  Can1,
}

impl Pmc {
  /// Disable a peripheral clock.  Datasheet §9.1, 28.15.4, 28.15.23
  pub fn enable_clock(&mut self, clock: PeripheralClock) {
    // Enable write access to the PMC
    self
      .pmc
      .pmc_wpmr
      .write(|w| w.wpkey().passwd().wpen().clear_bit());

    match clock {
      PeripheralClock::PioA => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid11().set_bit()),
      PeripheralClock::PioB => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid12().set_bit()),
      PeripheralClock::PioC => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid13().set_bit()),
      PeripheralClock::PioD => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid14().set_bit()),
      PeripheralClock::Usart0 => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid17().set_bit()),
      PeripheralClock::Usart1 => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid18().set_bit()),
      PeripheralClock::Usart2 => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid19().set_bit()),
      PeripheralClock::Usart3 => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid20().set_bit()),
      PeripheralClock::HsMci => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid21().set_bit()),
      PeripheralClock::Twi0 => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid22().set_bit()),
      PeripheralClock::Twi1 => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid23().set_bit()),
      PeripheralClock::Spi0 => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid24().set_bit()),
      PeripheralClock::Ssc => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid26().set_bit()),
      PeripheralClock::Tc0 => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid27().set_bit()),
      PeripheralClock::Tc1 => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid28().set_bit()),
      PeripheralClock::Tc2 => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid29().set_bit()),
      PeripheralClock::Tc3 => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid30().set_bit()),
      PeripheralClock::Tc4 => self.pmc.pmc_pcer0.write_with_zero(|w| w.pid31().set_bit()),
      PeripheralClock::Tc5 => self.pmc.pmc_pcer1.write_with_zero(|w| w.pid32().set_bit()),
      PeripheralClock::Tc6 => self.pmc.pmc_pcer1.write_with_zero(|w| w.pid33().set_bit()),
      PeripheralClock::Tc7 => self.pmc.pmc_pcer1.write_with_zero(|w| w.pid34().set_bit()),
      PeripheralClock::Tc8 => self.pmc.pmc_pcer1.write_with_zero(|w| w.pid35().set_bit()),
      PeripheralClock::Pwm => self.pmc.pmc_pcer1.write_with_zero(|w| w.pid36().set_bit()),
      PeripheralClock::Adc => self.pmc.pmc_pcer1.write_with_zero(|w| w.pid37().set_bit()),
      PeripheralClock::Dacc => self.pmc.pmc_pcer1.write_with_zero(|w| w.pid38().set_bit()),
      PeripheralClock::Dmac => self.pmc.pmc_pcer1.write_with_zero(|w| w.pid39().set_bit()),
      PeripheralClock::UOtgHs => self.pmc.pmc_pcer1.write_with_zero(|w| w.pid40().set_bit()),
      PeripheralClock::Trng => self.pmc.pmc_pcer1.write_with_zero(|w| w.pid41().set_bit()),
      PeripheralClock::Emac => self.pmc.pmc_pcer1.write_with_zero(|w| w.pid42().set_bit()),
      PeripheralClock::Can0 => self.pmc.pmc_pcer1.write_with_zero(|w| w.pid43().set_bit()),
      PeripheralClock::Can1 => self.pmc.pmc_pcer1.write_with_zero(|w| w.pid44().set_bit()),
    }
  }

  /// Disable a peripheral clock.  Datasheet §9.1, 28.15.5, 28.15.24.
  pub fn disable_clock(&mut self, clock: PeripheralClock) {
    // Enable write access to the PMC
    self
      .pmc
      .pmc_wpmr
      .write(|w| w.wpkey().passwd().wpen().clear_bit());

    match clock {
      PeripheralClock::PioA => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid11().set_bit()),
      PeripheralClock::PioB => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid12().set_bit()),
      PeripheralClock::PioC => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid13().set_bit()),
      PeripheralClock::PioD => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid14().set_bit()),
      PeripheralClock::Usart0 => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid17().set_bit()),
      PeripheralClock::Usart1 => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid18().set_bit()),
      PeripheralClock::Usart2 => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid19().set_bit()),
      PeripheralClock::Usart3 => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid20().set_bit()),
      PeripheralClock::HsMci => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid21().set_bit()),
      PeripheralClock::Twi0 => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid22().set_bit()),
      PeripheralClock::Twi1 => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid23().set_bit()),
      PeripheralClock::Spi0 => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid24().set_bit()),
      PeripheralClock::Ssc => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid26().set_bit()),
      PeripheralClock::Tc0 => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid27().set_bit()),
      PeripheralClock::Tc1 => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid28().set_bit()),
      PeripheralClock::Tc2 => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid29().set_bit()),
      PeripheralClock::Tc3 => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid30().set_bit()),
      PeripheralClock::Tc4 => self.pmc.pmc_pcdr0.write_with_zero(|w| w.pid31().set_bit()),
      PeripheralClock::Tc5 => self.pmc.pmc_pcdr1.write_with_zero(|w| w.pid32().set_bit()),
      PeripheralClock::Tc6 => self.pmc.pmc_pcdr1.write_with_zero(|w| w.pid33().set_bit()),
      PeripheralClock::Tc7 => self.pmc.pmc_pcdr1.write_with_zero(|w| w.pid34().set_bit()),
      PeripheralClock::Tc8 => self.pmc.pmc_pcdr1.write_with_zero(|w| w.pid35().set_bit()),
      PeripheralClock::Pwm => self.pmc.pmc_pcdr1.write_with_zero(|w| w.pid36().set_bit()),
      PeripheralClock::Adc => self.pmc.pmc_pcdr1.write_with_zero(|w| w.pid37().set_bit()),
      PeripheralClock::Dacc => self.pmc.pmc_pcdr1.write_with_zero(|w| w.pid38().set_bit()),
      PeripheralClock::Dmac => self.pmc.pmc_pcdr1.write_with_zero(|w| w.pid39().set_bit()),
      PeripheralClock::UOtgHs => self.pmc.pmc_pcdr1.write_with_zero(|w| w.pid40().set_bit()),
      PeripheralClock::Trng => self.pmc.pmc_pcdr1.write_with_zero(|w| w.pid41().set_bit()),
      PeripheralClock::Emac => self.pmc.pmc_pcdr1.write_with_zero(|w| w.pid42().set_bit()),
      PeripheralClock::Can0 => self.pmc.pmc_pcdr1.write_with_zero(|w| w.pid43().set_bit()),
      PeripheralClock::Can1 => self.pmc.pmc_pcdr1.write_with_zero(|w| w.pid44().set_bit()),
    }
  }
}

pub trait PmcExt {
  fn freeze(self, config: Config) -> Pmc;
}

#[inline(always)]
fn set_prescaler(pmc: &PMC, cfg: &Config) {
  // Set the prescaler.  Supposedly this should be done separatelly from picking the PLL as an
  // input source. Datasheet §28.12.4
  pmc.pmc_mckr.write(|w| {
    let w = w.pres();
    match cfg.pres {
      ProcessorClockPrescaler::Clk => w.clk_1(),
      ProcessorClockPrescaler::Clk2 => w.clk_2(),
      ProcessorClockPrescaler::Clk3 => w.clk_3(),
      ProcessorClockPrescaler::Clk4 => w.clk_4(),
      ProcessorClockPrescaler::Clk8 => w.clk_8(),
      ProcessorClockPrescaler::Clk16 => w.clk_16(),
      ProcessorClockPrescaler::Clk32 => w.clk_32(),
      ProcessorClockPrescaler::Clk64 => w.clk_64(),
    }
  });

  while !pmc.pmc_sr.read().mckrdy().bit_is_set() {}
}

#[inline(always)]
fn set_clock(pmc: &PMC, cfg: &Config) {
  // Set the prescaler.  Supposedly this should be done separatelly from picking the PLL as an
  // input source. Datasheet §28.12.4
  pmc.pmc_mckr.write(|w| {
    let w = w.pres();
    let w = match cfg.pres {
      ProcessorClockPrescaler::Clk => w.clk_1(),
      ProcessorClockPrescaler::Clk2 => w.clk_2(),
      ProcessorClockPrescaler::Clk3 => w.clk_3(),
      ProcessorClockPrescaler::Clk4 => w.clk_4(),
      ProcessorClockPrescaler::Clk8 => w.clk_8(),
      ProcessorClockPrescaler::Clk16 => w.clk_16(),
      ProcessorClockPrescaler::Clk32 => w.clk_32(),
      ProcessorClockPrescaler::Clk64 => w.clk_64(),
    };

    match cfg.css {
      MasterClockSrc::MainClock(_) => w.css().main_clk(),
      MasterClockSrc::Pll(_, _, _) => w.css().plla_clk(),
      MasterClockSrc::SlowClock => w.css().slow_clk(),
    }
  });

  while !pmc.pmc_sr.read().mckrdy().bit_is_set() {}
}

#[inline(always)]
fn configure_pll_a(pmc: &PMC, div_a: DivA, mul_a: MulA) {
  pmc.ckgr_pllar.write(|w| {
    unsafe {
      let w = match div_a {
        DivA::Zero => w.diva().bits(0),
        DivA::Bypassed => w.diva().bits(1),
        DivA::Output(d) => w.diva().bits(d),
      };

      let w = match mul_a {
        MulA::Deactivated => w.mula().bits(0),
        MulA::Activated(m) => w.mula().bits(m),
      };

      w.one()
        .set_bit()
        // Settling time taken from Arduino
        .pllacount()
        .bits(0x3F)
    }
  });

  while !pmc.pmc_sr.read().locka().bit_is_set() {}
}

impl PmcExt for PMC {
  fn freeze(self, cfg: Config) -> Pmc {
    match cfg.css {
      MasterClockSrc::Pll(oscillator, div_a, mul_a) => {
        match oscillator {
          MainOscillator::XtalOscillator => {
            // Initialize main oscillator
            self.ckgr_mor.write(|w| unsafe {
              w
                // set "password"
                .key()
                .passwd()
                // Set the startup time that Arduino seems to think is appropriate
                .moscxtst()
                .bits(8)
                .moscrcen()
                .set_bit()
                .moscxten()
                .set_bit()
            });

            // Wait for main oscillator to come online
            while !self.pmc_sr.read().moscxts().bit_is_set() {}

            // Switch to 3-20MHz Xtal oscillator
            {
              self.ckgr_mor.write(|w| unsafe {
                w
                  // set "password"
                  .key()
                  .passwd()
                  // Set main clock to 84 MHz
                  .moscxtst()
                  .bits(8)
                  // Main On-Chip RC Oscillator Enable
                  .moscrcen()
                  .set_bit()
                  // Main Crystal Oscillator Enable
                  .moscxten()
                  .set_bit()
                  // Main Oscillator Selection
                  // the 3 to 20 MHz Crystal or Ceramic Resonator-based oscillator clock is selected as the source clock of MAINCK (MOSCSEL = 1),
                  .moscsel()
                  .set_bit()
              });

              while !self.pmc_sr.read().moscsels().bit_is_set() {}

              // Master Clock Register
              self.pmc_mckr.write(|w| w.css().main_clk());
              while !self.pmc_sr.read().mckrdy().bit_is_set() {}
            }

            configure_pll_a(&self, div_a, mul_a);

            set_prescaler(&self, &cfg);
            set_clock(&self, &cfg);
          }
          MainOscillator::FastRcOscillator(_rc_oscillator_speed) => unimplemented!(),
        }
      }
      MasterClockSrc::MainClock(oscillator) => {
        // Initialize main oscillator
        match oscillator {
          // Select the 3 to 20 MHz Crystal or Ceramic Resonator-based oscillator
          MainOscillator::XtalOscillator => {
            // Initialize main oscillator
            self.ckgr_mor.write(|w| unsafe {
              w
                // set "password"
                .key()
                .passwd()
                // Set the startup time that Arduino seems to think is appropriate
                .moscxtst()
                .bits(8)
                .moscrcen()
                .set_bit()
                .moscxten()
                .set_bit()
            });

            // Wait for main oscillator to come online
            while !self.pmc_sr.read().moscxts().bit_is_set() {}

            // Switch to 3-20MHz Xtal oscillator
            {
              self.ckgr_mor.write(|w| unsafe {
                w
                  // set "password"
                  .key()
                  .passwd()
                  // Set main clock to 84 MHz
                  .moscxtst()
                  .bits(8)
                  // Main On-Chip RC Oscillator Enable
                  .moscrcen()
                  .set_bit()
                  // Main Crystal Oscillator Enable
                  .moscxten()
                  .set_bit()
                  // Main Oscillator Selection
                  // the 3 to 20 MHz Crystal or Ceramic Resonator-based oscillator clock is selected as the source clock of MAINCK (MOSCSEL = 1),
                  .moscsel()
                  .set_bit()
              });

              while !self.pmc_sr.read().moscsels().bit_is_set() {}
            }
          }
          MainOscillator::FastRcOscillator(rc_speed) => {
            self.ckgr_mor.write(|w| {
              w.moscsel()
                .clear_bit()
                .moscrcen()
                .set_bit()
                .moscrcf()
                .variant(match rc_speed {
                  RcOscillatorSpeed::Speed4Mhz => MOSCRCF_A::_4_MHZ,
                  RcOscillatorSpeed::Speed8Mhz => MOSCRCF_A::_8_MHZ,
                  RcOscillatorSpeed::Speed12Mhz => MOSCRCF_A::_12_MHZ,
                })
            });
          }
        }

        // Switch master clock to Main Clock
        self.pmc_mckr.write(|w| {
          w
            // Main Clock is selected
            .css()
            .main_clk()
        });

        set_prescaler(&self, &cfg);

        while !self.pmc_sr.read().mckrdy().bit_is_set() {}

        // Switch to PLLA
        // {
        //   self.pmc_mckr.write(|w| {
        //     w
        //       // Select clock divided by 2
        //       .pres()
        //       .clk_2()
        //       // PLLA Clock is selected
        //       .css()
        //       .plla_clk()
        //   });
        //
        //   while !self.pmc_sr.read().mckrdy().bit_is_set() {}
        // }

        // Enable UPLL clock
        self
          .ckgr_uckr
          .write(|w| unsafe { w.upllcount().bits(3).upllen().set_bit() });

        while !self.pmc_sr.read().locku().bit_is_set() {}

        // Switch USB clock to UPLL
        self
          .pmc_usb
          .write_with_zero(|w| unsafe { w.usbs().set_bit().usbdiv().bits(0) });

        // Enable USB clock
        self.pmc_scer.write_with_zero(|w| w.uotgclk().set_bit());
      }
      MasterClockSrc::SlowClock => {
        self
          .pmc_wpmr
          .write(|w| w.wpkey().passwd().wpen().clear_bit());

        self.pmc_mckr.write(|w| w.css().slow_clk());
        while !self.pmc_sr.read().mckrdy().bit_is_set() {}

        self.pmc_mckr.write(|w| w.css().slow_clk().pres().clk_1());
        while !self.pmc_sr.read().mckrdy().bit_is_set() {}
      }
    }

    let master_clock: Hertz = match cfg.css {
      MasterClockSrc::SlowClock => SLOW_CLOCK_SPEED.into(),
      MasterClockSrc::Pll(oscillator, div_a, mul_a) => {
        let oscillator_speed: Hertz = match oscillator {
          MainOscillator::XtalOscillator => XTAL_SPEED.into(),
          MainOscillator::FastRcOscillator(rc_speed) => match rc_speed {
            RcOscillatorSpeed::Speed4Mhz => 4.mhz().into(),
            RcOscillatorSpeed::Speed8Mhz => 8.mhz().into(),
            RcOscillatorSpeed::Speed12Mhz => 12.mhz().into(),
          },
        };
        match (div_a, mul_a) {
          (DivA::Zero, _) => 0.hz(),
          (_, MulA::Deactivated) => 0.hz(),
          (DivA::Bypassed, MulA::Activated(m)) => (oscillator_speed.0 * (m as u32 + 1)).hz(),
          (DivA::Output(d), MulA::Activated(m)) => {
            (oscillator_speed.0 * (m as u32 + 1) / d as u32).hz()
          }
        }
      }
      MasterClockSrc::MainClock(oscillator) => match oscillator {
        MainOscillator::XtalOscillator => XTAL_SPEED.into(),
        MainOscillator::FastRcOscillator(rc_speed) => match rc_speed {
          RcOscillatorSpeed::Speed4Mhz => 4.mhz().into(),
          RcOscillatorSpeed::Speed8Mhz => 8.mhz().into(),
          RcOscillatorSpeed::Speed12Mhz => 12.mhz().into(),
        },
      },
    };

    let processor_clock: Hertz = match cfg.pres {
      ProcessorClockPrescaler::Clk => master_clock,
      ProcessorClockPrescaler::Clk2 => (master_clock.0 / 2).hz(),
      ProcessorClockPrescaler::Clk3 => (master_clock.0 / 3).hz(),
      ProcessorClockPrescaler::Clk4 => (master_clock.0 / 4).hz(),
      ProcessorClockPrescaler::Clk8 => (master_clock.0 / 8).hz(),
      ProcessorClockPrescaler::Clk16 => (master_clock.0 / 16).hz(),
      ProcessorClockPrescaler::Clk32 => (master_clock.0 / 32).hz(),
      ProcessorClockPrescaler::Clk64 => (master_clock.0 / 64).hz(),
    };

    Pmc {
      pmc: self,
      clocks: Clocks {
        source: cfg.css,
        processor_clock,
        master_clock,
      },
    }
  }
}
