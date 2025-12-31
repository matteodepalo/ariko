use core::cell::RefCell;
use cortex_m::peripheral::{NVIC, SYST};
use critical_section::Mutex;
use sam3x8e_hal::gpio::pioa::PA29;
use sam3x8e_hal::gpio::piob::PB25;
use sam3x8e_hal::gpio::pioc::{PC22, PC23, PC24, PC25, PC28};
use sam3x8e_hal::gpio::{Input, Output, PullUp, PushPull};
use sam3x8e_hal::pac as sam3x8e;
use sam3x8e_hal::pac::{Interrupt, PIOB, PIOC, RTT, TC0, TWI0, UART, UOTGHS};
use sam3x8e_hal::pmc::{Config, MainOscillator, PeripheralClock, Pmc, PmcExt};
use sam3x8e_hal::prelude::*;
use sam3x8e_hal::time::Hertz;
use sam3x8e_hal::timer::Timer;

static PERIPHERALS: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));

pub struct Peripherals {
  pub uart: UART,
  pub twi0: TWI0,
  pub uotghs: UOTGHS,
  pub pmc: Pmc,
  pub delay: Delay<SYST>,
  pub blue_button: PB25<Input<PullUp>>,
  pub white_button: PC28<Input<PullUp>>,
  pub buzzer: PA29<Output<PushPull>>,
  pub timer: Timer<RTT>,
  // TM1637 display pins (Grove D7 = White clock, D5 = Black clock)
  pub white_clk: PC23<Output<PushPull>>,
  pub white_dio: PC22<Output<PushPull>>,
  pub black_clk: PC25<Output<PushPull>>,
  pub black_dio: PC24<Output<PushPull>>,
}

impl Peripherals {
  pub fn init() {
    let p = sam3x8e::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();

    // Disable watchdog to prevent restarts
    unsafe {
      p.WDT.mr().write_with_zero(|w| w.wddis().set_bit());
    }

    let mut pmc = p
      .PMC
      .freeze(Config::main_clock(MainOscillator::XtalOscillator));

    let mut pioa = p.PIOA.split(&mut pmc);
    let mut piob = p.PIOB.split(&mut pmc);
    let mut pioc = p.PIOC.split(&mut pmc);
    let delay = cp.SYST.delay(pmc.clocks);
    let timer = p.RTT.timer(Hertz(0));

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
      .pb10
      .disable_pio_line(&mut piob.pdr)
      .into_peripheral_a(&mut piob.absr);

    let blue_button = piob.pb25.into_pull_up_input(&mut piob.puer);
    let white_button = pioc.pc28.into_pull_up_input(&mut pioc.puer);

    let buzzer = pioa
      .pa29
      .into_push_pull_output(&mut pioa.mddr, &mut pioa.oer);

    // TM1637 display pins (Grove D7 = White clock, D5 = Black clock)
    let white_clk = pioc
      .pc23
      .into_push_pull_output(&mut pioc.mddr, &mut pioc.oer);
    let white_dio = pioc
      .pc22
      .into_push_pull_output(&mut pioc.mddr, &mut pioc.oer);
    let black_clk = pioc
      .pc25
      .into_push_pull_output(&mut pioc.mddr, &mut pioc.oer);
    let black_dio = pioc
      .pc24
      .into_push_pull_output(&mut pioc.mddr, &mut pioc.oer);

    // Configure button interrupts (falling edge on PB25 and PC28)
    configure_button_interrupts();

    // Configure TC0 for 100ms periodic interrupt
    configure_timer_interrupt(&p.TC0, &mut pmc);

    // Enable interrupts in NVIC
    unsafe {
      cp.NVIC.set_priority(Interrupt::PIOB, 2);
      cp.NVIC.set_priority(Interrupt::PIOC, 2);
      cp.NVIC.set_priority(Interrupt::TC0, 1);
      NVIC::unmask(Interrupt::PIOB);
      NVIC::unmask(Interrupt::PIOC);
      NVIC::unmask(Interrupt::TC0);
    }

    critical_section::with(|cs| {
      PERIPHERALS.borrow(cs).replace(Some(Peripherals {
        uart: p.UART,
        twi0: p.TWI0,
        uotghs: p.UOTGHS,
        blue_button,
        white_button,
        buzzer,
        pmc,
        delay,
        timer,
        white_clk,
        white_dio,
        black_clk,
        black_dio,
      }));
    });
  }

  /// Access peripherals within a critical section.
  /// The closure receives a mutable reference to the Peripherals struct.
  pub fn with<F, R>(f: F) -> R
  where
    F: FnOnce(&mut Peripherals) -> R,
  {
    critical_section::with(|cs| {
      let mut borrow = PERIPHERALS.borrow(cs).borrow_mut();
      let peripherals = borrow.as_mut().expect("Peripherals not initialized");
      f(peripherals)
    })
  }
}

/// Configure PIO interrupts for buttons (falling edge).
fn configure_button_interrupts() {
  unsafe {
    let piob = &*PIOB::ptr();
    let pioc = &*PIOC::ptr();

    // Blue button on PB25: enable PIO and configure falling edge interrupt
    // PER: PIO Enable Register - enable PIO control of this pin
    piob.per().write_with_zero(|w| w.bits(1 << 25));
    // ODR: Output Disable Register - configure as input
    piob.odr().write_with_zero(|w| w.bits(1 << 25));
    // AIMER: Additional Interrupt Modes Enable
    piob.aimer().write_with_zero(|w| w.bits(1 << 25));
    // ESR: Edge Select Register (1 = edge, not level)
    piob.esr().write_with_zero(|w| w.bits(1 << 25));
    // FELLSR: Falling Edge/Low Level Select (1 = falling edge)
    piob.fellsr().write_with_zero(|w| w.bits(1 << 25));
    // Clear any pending interrupt by reading ISR
    let _ = piob.isr().read();
    // IER: Interrupt Enable Register
    piob.ier().write_with_zero(|w| w.bits(1 << 25));

    // White button on PC28: enable PIO and configure falling edge interrupt
    pioc.per().write_with_zero(|w| w.bits(1 << 28));
    pioc.odr().write_with_zero(|w| w.bits(1 << 28));
    pioc.aimer().write_with_zero(|w| w.bits(1 << 28));
    pioc.esr().write_with_zero(|w| w.bits(1 << 28));
    pioc.fellsr().write_with_zero(|w| w.bits(1 << 28));
    let _ = pioc.isr().read();
    pioc.ier().write_with_zero(|w| w.bits(1 << 28));
  }
}

/// Configure TC0 channel 0 for 100ms periodic interrupt.
///
/// Clock: MCK/128 (84MHz/128 = 656,250 Hz)
/// For 100ms: RC = 656,250 * 0.1 = 65,625 counts
fn configure_timer_interrupt(tc0: &TC0, pmc: &mut Pmc) {
  // Enable TC0 peripheral clock
  pmc.enable_clock(PeripheralClock::Tc0);

  unsafe {
    // Disable clock first
    tc0.ccr0().write_with_zero(|w| w.clkdis().set_bit());

    // Configure channel mode:
    // - WAVE = 1 (waveform mode)
    // - WAVSEL = UP_RC (UP mode with automatic trigger on RC compare)
    // - TCCLKS = TIMER_CLOCK4 (MCK/128)
    tc0.wave_eq_1_cmr0_wave_eq_1().write(|w| {
      w.tcclks()
        .timer_clock4() // MCK/128 = 656,250 Hz
        .wave()
        .set_bit()
        .wavsel()
        .up_rc() // UP mode, reset on RC compare
    });

    // Set RC compare value for 100ms period
    // Empirically adjusted - system clock appears to be ~21MHz (84/4)
    // 21MHz / 128 = 164,062.5 Hz
    // 164,062.5 * 0.1s = 16,406 counts
    tc0.rc0().write(|w| w.rc().bits(16406));

    // Enable RC compare interrupt (CPCS)
    tc0.ier0().write_with_zero(|w| w.cpcs().set_bit());

    // Enable clock and trigger
    tc0.ccr0().write_with_zero(|w| w.clken().set_bit().swtrg().set_bit());
  }
}
