use cortex_m::peripheral::NVIC;
use embedded_hal::digital::InputPin;
use sam3x8e_hal::gpio::piob::PB25;
use sam3x8e_hal::gpio::{Input, Output, PullUp, PushPull};
use sam3x8e_hal::pac as sam3x8e;
use sam3x8e_hal::pac::piob::PDSR;
use sam3x8e_hal::pac::{PIOB, SYST, TWI0, UART, UOTGHS};
use sam3x8e_hal::pmc::RcOscillatorSpeed::Speed12Mhz;
use sam3x8e_hal::pmc::{Config, MainOscillator, PeripheralClock, Pmc, PmcExt};
use sam3x8e_hal::prelude::*;

static mut S_PERIPHERALS: Option<Peripherals> = None;

pub struct Peripherals {
  pub uart: UART,
  pub twi0: TWI0,
  pub uotghs: UOTGHS,
  pub nvic: NVIC,
  pub pmc: Pmc,
  pub delay: Delay<SYST>,
  pub button: PB25<Input<PullUp>>,
}

impl Peripherals {
  pub fn init() {
    let p = sam3x8e::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut pmc = p
      .PMC
      .freeze(Config::main_clock(MainOscillator::XtalOscillator));

    let mut pioa = p.PIOA.split(&mut pmc);
    let mut piob = p.PIOB.split(&mut pmc);
    let delay = cp.SYST.delay(pmc.clocks);

    // TWI0
    pioa
      .pa18
      .disable_pio_line(&mut pioa.pdr)
      .into_peripheral_a(&mut pioa.absr);

    pioa
      .pa17
      .disable_pio_line(&mut pioa.pdr)
      .into_peripheral_a(&mut pioa.absr);

    // UART
    pioa
      .pa8
      .disable_pio_line(&mut pioa.pdr)
      .into_peripheral_a(&mut pioa.absr);

    pioa
      .pa9
      .disable_pio_line(&mut pioa.pdr)
      .into_peripheral_a(&mut pioa.absr);

    // UOTGHS
    piob
      .pb11
      .disable_pio_line(&mut piob.pdr)
      .into_peripheral_a(&mut piob.absr)
      .into_pull_up_input(&mut piob.puer);

    piob
      .pb10
      .disable_pio_line(&mut piob.pdr)
      .into_peripheral_a(&mut piob.absr);

    // Dual button
    let button = piob.pb25.into_pull_up_input(&mut piob.puer);

    unsafe {
      S_PERIPHERALS = Some(Peripherals {
        uart: p.UART,
        twi0: p.TWI0,
        uotghs: p.UOTGHS,
        nvic: cp.NVIC,
        button,
        pmc,
        delay,
      })
    }
  }

  pub fn get() -> &'static mut Self {
    unsafe { S_PERIPHERALS.as_mut().unwrap() }
  }
}
