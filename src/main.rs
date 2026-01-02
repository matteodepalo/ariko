#![cfg_attr(target_arch = "arm", no_std)]
#![cfg_attr(target_arch = "arm", no_main)]

#[cfg(target_arch = "arm")]
extern crate cortex_m_rt;

#[cfg(target_arch = "arm")]
use core::panic::PanicInfo;
#[cfg(target_arch = "arm")]
use core::sync::atomic::Ordering;
#[cfg(target_arch = "arm")]
use core::fmt::Write;

#[cfg(target_arch = "arm")]
use cortex_m_rt::entry;
#[cfg(target_arch = "arm")]
use embedded_hal::delay::DelayNs;
#[cfg(target_arch = "arm")]
use sam3x8e_hal::pac::{PIOB, PIOC, TC0};

#[cfg(target_arch = "arm")]
use certabo::app::{App, Hardware};
#[cfg(target_arch = "arm")]
use certabo::arm_io::{ArmBoard, ArmBuzzer, ArmClockDisplay, ArmDelay, ArmDisplay};
#[cfg(target_arch = "arm")]
use certabo::buzzer::Buzzer;
#[cfg(target_arch = "arm")]
use certabo::certabo::buffer::MAX_LINE_LEN;
#[cfg(target_arch = "arm")]
use certabo::certabo::protocol::RfidReading;
#[cfg(target_arch = "arm")]
use certabo::display::Display;
#[cfg(target_arch = "arm")]
use certabo::events::{consume, BLUE_BUTTON_PRESSED, MILLIS, TIMER_TICK, WHITE_BUTTON_PRESSED};
#[cfg(target_arch = "arm")]
use certabo::i2c::I2C;
#[cfg(target_arch = "arm")]
use certabo::logger::Logger;
#[cfg(target_arch = "arm")]
use certabo::peripherals::Peripherals;
#[cfg(target_arch = "arm")]
use certabo::serial::Serial;
#[cfg(target_arch = "arm")]
use certabo::tm1637::ChessClockDisplays;
#[cfg(target_arch = "arm")]
use certabo::usb::USB;

#[cfg(target_arch = "arm")]
#[entry]
fn main() -> ! {
    Peripherals::init();
    Serial::init(57600);
    Logger::init();
    I2C::init();
    Display::init();
    USB::init();
    Buzzer::init();
    ChessClockDisplays::init();

    let mut app = App::new();
    let mut hw = Hardware {
        board: ArmBoard,
        display: ArmDisplay,
        buzzer: ArmBuzzer,
        clock: ArmClockDisplay,
        delay: ArmDelay,
    };

    Display::with(|d| {
        d.write_str("Certabo Chess").unwrap();
    });
    Peripherals::with(|p| p.delay.delay_ms(1000));

    app.start(&mut hw);

    loop {
        if consume(&BLUE_BUTTON_PRESSED) {
            app.on_blue_button(&mut hw.display);
        }

        if consume(&WHITE_BUTTON_PRESSED) {
            app.on_white_button(&mut hw.display);
        }

        if consume(&TIMER_TICK) {
            app.tick(100, &mut hw.display, &mut hw.buzzer, &mut hw.clock);
            app.update_display(&mut hw.display);
        }

        hw.board.poll();

        let mut line_buffer = [0u8; MAX_LINE_LEN];
        if let Some(len) = hw.board.read_line(&mut line_buffer) {
            if len > 2 {
                let trimmed = &line_buffer[1..len - 1];
                if let Some(reading) = RfidReading::parse(trimmed) {
                    app.on_board_reading(reading, &mut hw.display, &mut hw.buzzer, &mut hw.delay);
                }
            }
        }

        if app.should_send_leds() {
            let _ = hw.board.send_leds(app.led_state().as_bytes());
            app.leds_sent();
        }

        Peripherals::with(|p| p.delay.delay_ms(10u32));
    }
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
pub extern "C" fn PIOB() {
    let isr = unsafe { (*PIOB::ptr()).isr().read().bits() };
    if isr & (1 << 25) != 0 {
        BLUE_BUTTON_PRESSED.store(true, Ordering::SeqCst);
    }
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
pub extern "C" fn PIOC() {
    let isr = unsafe { (*PIOC::ptr()).isr().read().bits() };
    if isr & (1 << 28) != 0 {
        WHITE_BUTTON_PRESSED.store(true, Ordering::SeqCst);
    }
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
pub extern "C" fn TC0() {
    let _ = unsafe { (*TC0::ptr()).sr0().read() };
    MILLIS.fetch_add(100, Ordering::Relaxed);
    TIMER_TICK.store(true, Ordering::SeqCst);
}

#[cfg(target_arch = "arm")]
#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let location = info.location().unwrap();

    loop {
        Serial::with(|serial| {
            serial
                .write_fmt(format_args!(
                    "Panic at {} ({}, {}): {}\n\r",
                    location.file(),
                    location.line(),
                    location.column(),
                    info.message()
                ))
                .unwrap();
        });

        Peripherals::with(|p| p.delay.delay_ms(1000_u32));
    }
}

#[cfg(not(target_arch = "arm"))]
fn main() {}
