# certabo-rs Dependency Update Plan

This document outlines the steps to update the certabo-rs project from its 4-year-old state to use current (December 2025) dependencies and Rust edition.

## Project Overview

The project consists of:
- **certabo** (main binary) - Chess board controller
- **sam3x8e** (PAC) - Peripheral Access Crate generated with svd2rust 0.16.1
- **sam3x8e-hal** - Hardware Abstraction Layer
- **sam3x8e-hal-codegen** - Proc-macro for GPIO code generation

Target: Arduino Due (ARM Cortex-M3, SAM3X8E)

---

## Phase 1: Rust Edition and Toolchain Update

### 1.1 Update Rust Edition
**All Cargo.toml files**: Change `edition = "2018"` to `edition = "2021"`

Affected files:
- `/Cargo.toml`
- `/sam3x8e-rs/sam3x8e/Cargo.toml`
- `/sam3x8e-rs/sam3x8e-hal/Cargo.toml`
- `/sam3x8e-rs/sam3x8e-hal-codegen/Cargo.toml`

**Note**: Edition 2024 is available but 2021 is more conservative and well-supported.

### 1.2 Create rust-toolchain.toml
Create `/rust-toolchain.toml`:
```toml
[toolchain]
channel = "stable"
targets = ["thumbv7m-none-eabi"]
```

---

## Phase 2: Core Embedded Dependencies

### 2.1 cortex-m Stack Updates

| Crate | Current | Target | Breaking Changes |
|-------|---------|--------|------------------|
| cortex-m | 0.5.8/0.6.0/0.7.1 | **0.7.7** | Minor - unify all to 0.7.7 |
| cortex-m-rt | 0.6.10/0.6.13 | **0.7.5** | **YES** - see below |
| cortex-m-semihosting | 0.3.3 | **0.5.0** | Minor |

**cortex-m-rt 0.6 → 0.7 Breaking Changes:**
- Minimum Rust version is now 1.61.0
- The `device` feature now requires the PAC to export `__INTERRUPTS` as `#[no_mangle]`
- `HardFaultTrampoline` is now the default HardFault handler
- `link.x` changes may require linker script updates

### 2.2 panic-halt Update

| Current | Target |
|---------|--------|
| 0.2.0 | **1.0.0** |

No significant API changes.

---

## Phase 3: embedded-hal Ecosystem (CRITICAL)

### 3.1 embedded-hal Migration

| Current | Target |
|---------|--------|
| 1.0.0-alpha.4 | **1.0.0** (stable) |

**Major Breaking Changes:**

1. **GPIO Traits (`InputPin`, `OutputPin`)**:
   - Methods now take `&mut self` instead of `&self`
   - `try_is_high()` → `is_high()` (returns `Result`)
   - `try_is_low()` → `is_low()` (returns `Result`)
   - `try_set_high()` → `set_high()` (returns `Result`)
   - `try_set_low()` → `set_low()` (returns `Result`)

2. **Delay Traits**:
   - `DelayMs` and `DelayUs` merged into **`DelayNs`**
   - `try_delay_ms()` → `delay_ms()` (no `try_` prefix)
   - `try_delay_us()` → `delay_us()` (no `try_` prefix)
   - New: `delay_ns()` for nanosecond precision

3. **Module Organization**:
   - Traits moved to `embedded_hal::digital::{InputPin, OutputPin}`
   - Was: `embedded_hal::digital::v2::*` or `embedded_hal::digital::*`

4. **Error Types**:
   - Must implement `core::fmt::Debug`

5. **Blocking Traits Location**:
   - `embedded_hal::blocking::delay::*` → `embedded_hal::delay::*`

### 3.2 Additional embedded-hal Crates

New crates to consider adding:
- `embedded-hal-nb = "1.0"` - For non-blocking operations with `nb`
- `embedded-hal-bus = "0.2"` - For bus sharing (SPI/I2C)

### 3.3 nb Crate Update

| Current | Target |
|---------|--------|
| 0.1.2 / 1.0.0 | **1.1.0** |

Unify all uses to 1.1.0.

---

## Phase 4: SAM3X8E PAC Regeneration (CRITICAL)

The PAC was generated with svd2rust 0.16.1 (very old). Current version is **0.37.0**.

### 4.1 Option A: Regenerate PAC (Recommended)

1. Install latest svd2rust:
   ```bash
   cargo install svd2rust --version 0.37.0
   ```

2. Download SAM3X8E SVD file from Microchip/Atmel

3. Regenerate:
   ```bash
   svd2rust -i ATSAM3X8E.svd --target cortex-m
   ```

4. Apply form tool:
   ```bash
   cargo install form
   form -i lib.rs -o src/
   ```

**Breaking changes from svd2rust 0.16 → 0.37:**
- Register access API completely rewritten
- `read().bits()` → `read().bits()`
- `write(|w| w.bits())` → similar but may need adjustments
- `modify()` API changes
- Generated code uses modern `cortex-m` and `critical-section` crates
- Peripheral struct access patterns may differ

### 4.2 Option B: Use Existing Crate

There's an existing [sam3x8e crate on crates.io](https://crates.io/crates/sam3x8e) (v0.1.1).

However, it's also old (svd2rust 0.14.0). A newer alternative is [alt-sam3x8e](https://crates.io/crates/alt-sam3x8e).

### 4.3 PAC Dependencies Update

After regeneration, update:

| Crate | Current | Target |
|-------|---------|--------|
| bare-metal | 0.2.0 | **1.0.0** (or remove if not needed) |
| vcell | 0.1.0 | **0.1.3** |
| cortex-m | 0.5.8 | **0.7.7** |

**Note**: Modern svd2rust uses `critical-section` instead of `bare-metal`.

---

## Phase 5: Proc-Macro Dependencies (sam3x8e-hal-codegen)

### 5.1 syn 1.0 → 2.0 Migration (CRITICAL)

| Crate | Current | Target |
|-------|---------|--------|
| syn | 1.0 | **2.0** |
| quote | 1.0 | **1.0** (compatible) |
| proc-macro2 | 1.0.2 | **1.0.103** |

**syn 2.0 Breaking Changes:**
- `syn::parse_macro_input!` moved
- `DeriveInput` and other types restructured
- `Ident::new()` signature changes
- Custom keywords syntax changed
- `syn::Error` API changes

The codegen in `/sam3x8e-rs/sam3x8e-hal-codegen/src/` will need updates.

### 5.2 Pin Wildcard Dependencies

| Crate | Current | Target |
|-------|---------|--------|
| Inflector | "*" | **"0.11.4"** |
| maplit | "*" | **"1.0.2"** |

Always pin dependencies to specific versions.

---

## Phase 6: Other Dependencies

### 6.1 General Updates

| Crate | Current | Target | Notes |
|-------|---------|--------|-------|
| modular-bitfield | 0.11.2 | **0.13.0** | Minor API changes |
| log | 0.4.14 | **0.4.22** | Compatible |
| void | 1.0.2 | **1.0.2** | No change |

---

## Phase 7: Code Migrations

### 7.1 Main Binary (`src/main.rs`)

```rust
// OLD (embedded-hal alpha)
use embedded_hal::blocking::delay::DelayMs;
info.message().unwrap()  // panic_info.message() unstable

// NEW (embedded-hal 1.0)
use embedded_hal::delay::DelayNs;
// panic_info.message() - check current API
```

Changes needed:
1. Update `embedded_hal::blocking::delay::DelayMs` → `embedded_hal::delay::DelayNs`
2. Change `try_delay_ms()` → `delay_ms()`
3. Update `#![feature(panic_info_message)]` - may not be needed on stable
4. Review `PanicInfo::message()` usage

### 7.2 HAL GPIO Implementation (`sam3x8e-hal-codegen/src/pio/pin.rs`)

```rust
// OLD
impl<MODE> OutputPin for #pin_ident<Output<MODE>> {
    type Error = ();
    fn try_set_high(&mut self) -> Result<(), Self::Error> { ... }
    fn try_set_low(&mut self) -> Result<(), Self::Error> { ... }
}

impl<MODE> InputPin for #pin_ident<Input<MODE>> {
    type Error = ();
    fn try_is_high(&self) -> Result<bool, Self::Error> { ... }
    fn try_is_low(&self) -> Result<bool, Self::Error> { ... }
}

// NEW
impl<MODE> OutputPin for #pin_ident<Output<MODE>> {
    fn set_high(&mut self) -> Result<(), Self::Error> { ... }
    fn set_low(&mut self) -> Result<(), Self::Error> { ... }
}

impl<MODE> ErrorType for #pin_ident<Output<MODE>> {
    type Error = Infallible;  // or custom error type
}

impl<MODE> InputPin for #pin_ident<Input<MODE>> {
    fn is_high(&mut self) -> Result<bool, Self::Error> { ... }  // Note: &mut self
    fn is_low(&mut self) -> Result<bool, Self::Error> { ... }   // Note: &mut self
}
```

### 7.3 HAL Delay Implementation (`sam3x8e-hal/src/delay/syst.rs`)

```rust
// OLD
impl DelayMs<u32> for Delay<SYST> {
    type Error = ();
    fn try_delay_ms(&mut self, ms: u32) -> Result<(), Self::Error> { ... }
}

impl DelayUs<u32> for Delay<SYST> {
    type Error = ();
    fn try_delay_us(&mut self, us: u32) -> Result<(), Self::Error> { ... }
}

// NEW
impl DelayNs for Delay<SYST> {
    fn delay_ns(&mut self, ns: u32) { ... }
    fn delay_us(&mut self, us: u32) { ... }
    fn delay_ms(&mut self, ms: u32) { ... }
}
```

---

## Phase 8: Build System Updates

### 8.1 Makefile.toml Updates

Update `cargo +nightly objcopy` - may not need nightly anymore:
```toml
[tasks.post-build]
script = '''
  mkdir ./build
  exec cargo objcopy --release --bin certabo -- -S -O binary ./build/certabo.img
'''
```

### 8.2 .cargo/config.toml

Current config looks fine, but verify `thumbv7m-none-eabi` target is still correct.

---

## Execution Order

1. **Phase 1**: Rust edition update (low risk)
2. **Phase 5**: Proc-macro deps (syn 2.0) - isolated, can test independently
3. **Phase 6**: General deps (modular-bitfield, log) - low risk
4. **Phase 2**: cortex-m stack - medium risk
5. **Phase 4**: PAC regeneration - **high risk, most work**
6. **Phase 3**: embedded-hal migration - **high risk, invasive changes**
7. **Phase 7**: Code migrations - follows from above
8. **Phase 8**: Build system - final cleanup

---

## Testing Strategy

1. After each phase, attempt `cargo build --release`
2. Create a simple blink test program to verify GPIO works
3. Test delay functionality
4. Full integration test on hardware

---

## Potential Issues

1. **PAC Regeneration**: The biggest risk. May need to adjust HAL code significantly.
2. **embedded-hal `&mut self`**: May require refactoring code that shares pins.
3. **Unstable features**: `#![feature(panic_info_message)]` may need alternatives.
4. **cortex-m-rt 0.7**: Linker script changes could cause boot failures.

---

## Resources

- [embedded-hal migration guide](https://github.com/rust-embedded/embedded-hal/blob/master/docs/migrating-from-0.2-to-1.0.md)
- [embedded-hal 1.0 announcement](https://blog.rust-embedded.org/embedded-hal-v1/)
- [svd2rust documentation](https://docs.rs/svd2rust/latest/svd2rust/)
- [cortex-m-rt changelog](https://github.com/rust-embedded/cortex-m/blob/master/cortex-m-rt/CHANGELOG.md)
- [syn 2.0 release notes](https://github.com/dtolnay/syn/releases/tag/2.0.0)
- [Rust 2021 Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2021/index.html)
