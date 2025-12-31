# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Certabo-rs is a Rust firmware for Arduino Due (ARM Cortex-M3, SAM3X8E) that controls a Certabo electronic chess board. It handles piece detection via RFID, game state management, timer, LCD display, buzzer feedback, and LED control.

**Target:** `thumbv7m-none-eabi` (no_std embedded)

## Build Commands

```bash
# Build (requires nightly Rust at /opt/rust-nightly)
PATH="/opt/rust-nightly/bin:$PATH" cargo build --release

# Build and flash to Arduino Due
./deploy.sh
```

**Flashing requirements:**
- Arduino Due must be in bootloader mode (hold ERASE ~2s, then press RESET)
- User must be in `dialout` group for `/dev/ttyACM*` access
- Uses `bossac` for flashing (build from source: https://github.com/shumatech/BOSSA, arduino branch)

## Architecture

### Crate Structure

```
certabo-rs/
├── src/                    # Main firmware binary
├── sam3x8e-rs/
│   ├── sam3x8e/           # PAC (Peripheral Access Crate) - generated with svd2rust
│   ├── sam3x8e-hal/       # Hardware Abstraction Layer
│   └── sam3x8e-hal-codegen/ # Proc-macro for GPIO code generation
```

### Module Organization

**Hardware drivers:**
- `peripherals` - GPIO pins, delay, timer initialization
- `i2c` - I2C bus for display communication
- `serial` - UART for debugging (57600 baud)
- `usb` - USB host for CP210x (Certabo board uses USB-serial)
- `jhd1802` - JHD1802 LCD display driver
- `display` - High-level display API
- `buzzer` - PWM buzzer for audio feedback

**Certabo protocol:**
- `certabo::protocol` - Parses 320-byte RFID readings (64 squares × 5 bytes)
- `certabo::calibration` - Maps RFID chip IDs to piece types
- `certabo::leds` - 8-byte LED state (one byte per file, bits = ranks)

**Game logic:**
- `game::state` - Game state machine, turn tracking
- `game::timer` - 10+0 chess clock (10 min per side)

### Application State Machine

The main loop in `src/main.rs` implements:
```
Initializing → WaitingForCalibration → Calibrating → WaitingForSetup → GameInProgress ⟷ GamePaused → GameEnded
```

**Buttons:**
- Blue button: Start calibration
- White button: Pause/Resume game

### Global Singleton Pattern

Hardware peripherals use a static Mutex pattern for safe access:
```rust
static PERIPHERAL: Mutex<RefCell<Option<Peripheral>>> = Mutex::new(RefCell::new(None));

impl Peripheral {
    pub fn with<F, R>(f: F) -> R
    where F: FnOnce(&mut Peripheral) -> R {
        critical_section::with(|cs| { ... })
    }
}
```

## Key Technical Details

- **Rust Edition:** 2024 (requires nightly for `proc_macro_span` in HAL codegen)
- **embedded-hal:** 1.0 stable (methods like `is_low()`, `delay_ms()` without `try_` prefix)
- **No allocator:** MVP doesn't use heap allocation
- **Certabo protocol:** 38400 baud, space-separated ASCII values

## Serial Monitoring

The serial port cannot be monitored directly with blocking commands (e.g., `cat /dev/ttyACM0`) as they block the terminal completely. Use background logging instead:

```bash
# Kill any existing monitors
fuser -k /dev/ttyACM0 2>/dev/null

# Configure and start background logging
stty -F /dev/ttyACM0 57600 cs8 -cstopb -parenb raw -echo
cat /dev/ttyACM0 > /tmp/serial.log 2>&1 &

# View output
tail -f /tmp/serial.log
```

Note: `picocom` and interactive serial monitors don't work in non-TTY environments.

## Testing

Tests require host target and std:
```bash
cargo test --target x86_64-unknown-linux-gnu
```

Note: Most hardware-dependent code cannot be tested on host.
