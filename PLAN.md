# Certabo-rs Project Plan

## Project Goal

Build a standalone chess clock and game controller using Arduino Due + Certabo electronic chess board. Two players can play a complete game of chess using only the physical board, with the Arduino handling:

- Piece position detection via RFID
- Turn management and move validation
- Chess clock (10+0 time control)
- Audio feedback (buzzer)
- Visual feedback (LCD display + board LEDs)

No computer or phone required during gameplay.

---

## Hardware Setup

```
┌─────────────────┐     USB      ┌─────────────────┐
│  Certabo Board  │◄────────────►│   Arduino Due   │
│  (CP210x USB)   │              │   (SAM3X8E)     │
└─────────────────┘              └────────┬────────┘
                                          │
                        ┌─────────────────┼─────────────────┐
                        │                 │                 │
                   ┌────▼────┐      ┌─────▼─────┐     ┌─────▼─────┐
                   │  LCD    │      │  Buzzer   │     │  Buttons  │
                   │ JHD1802 │      │   PWM     │     │ Blue/White│
                   └─────────┘      └───────────┘     └───────────┘
```

**Communication:**
- Certabo → Arduino: 320 space-separated ASCII values (RFID readings) at 38400 baud
- Arduino → Certabo: 8-byte LED commands

---

## Implementation Status

### Phase 1: Foundation (COMPLETE)
- [x] Rust toolchain setup (nightly, thumbv7m-none-eabi)
- [x] SAM3X8E PAC regeneration (svd2rust)
- [x] HAL updates (embedded-hal 1.0)
- [x] Basic peripherals (GPIO, I2C, UART, PWM)

### Phase 2: Hardware Drivers (COMPLETE)
- [x] JHD1802 LCD display driver
- [x] Buzzer with sound effects
- [x] Button input handling
- [x] USB host controller (UOTGHS)
- [x] CP210x device driver

### Phase 3: Certabo Protocol (COMPLETE)
- [x] RFID reading parser (320-byte format)
- [x] Calibration system (RFID → piece mapping)
- [x] LED state management (8-byte format)

### Phase 4: Game Logic (COMPLETE)
- [x] Game state machine
- [x] Chess timer (10+0)
- [x] Turn tracking
- [x] Piece lift/place detection

### Phase 5: USB Data Flow (COMPLETE)
- [x] USB bulk IN - receive RFID data from board
- [x] Message buffering and framing (LineBuffer module)
- [x] USB bulk OUT - send LED commands to board
- [x] Integration with main loop (interrupt-driven architecture)

### Phase 6: Polish (IN PROGRESS)
- [ ] Hardware timer (RTT) for accurate timing (TC0 at 100ms is sufficient for now)
- [x] Move validation with lightweight chess module (no external deps)
- [x] Legal move LED highlighting (integrated with game state)
- [ ] Calibration persistence (EEPROM)

---

## Phase 5 Details: USB Data Flow

### Current State Analysis

**What exists:**
- USB host controller (UOTGHS) initialization ✓
- CP210x device detection and configuration ✓
- Bulk IN pipe allocated and configured ✓
- Status polling to check input queue ✓
- Data transfer into 512-byte buffer ✓

**What's broken/missing:**
1. **Baud rate is wrong:** Set to 115200, should be 38400
2. **Data is discarded:** Read into buffer, logged, then thrown away
3. **1-second delay in poll:** Way too slow for real-time chess
4. **No line buffering:** Certabo sends newline-terminated messages
5. **No API to retrieve data:** Main loop can't access received bytes
6. **No OUT pipe for LEDs:** Only IN pipe is configured

### 5.1 Bug Fix: Baud Rate

**Location:** `src/usb/device/cp210x.rs:186`

```rust
// Current (115200 baud = 0x0001c200):
Some(&mut [0, 0xc2, 0x01, 0]),

// Correct (38400 baud = 0x00009600):
Some(&mut [0x00, 0x96, 0x00, 0x00]),
```

### 5.2 Data Reception Architecture

**Line buffer design:**
```rust
// In CP210x device or separate module
const MAX_LINE_LEN: usize = 1024;  // 320 values * 3 chars + spaces

struct LineBuffer {
    buffer: [u8; MAX_LINE_LEN],
    len: usize,
    complete_line: Option<usize>,  // Length of complete line ready to read
}

impl LineBuffer {
    fn push(&mut self, data: &[u8]) {
        // Append data, check for newline
        // If newline found, set complete_line
    }

    fn take_line(&mut self) -> Option<&[u8]> {
        // Return complete line, shift remaining bytes
    }
}
```

**CP210x changes:**
```rust
// Add to CP210xDevice struct:
static LINE_BUFFER: Mutex<RefCell<LineBuffer>> = ...;

// In poll():
fn poll(&self) -> Result<(), Error> {
    // 1. Check status (keep existing code)
    // 2. If data available, read into temp buffer
    // 3. Push to LINE_BUFFER
    // 4. Remove 1-second delay
    Ok(())
}

// New method:
pub fn read_line() -> Option<[u8; MAX_LINE_LEN]> {
    // Take complete line from buffer if available
}
```

### 5.3 LED Transmission

**Certabo LED protocol (from Python client):**
- Uses bulk OUT endpoint (endpoint 2)
- 8 bytes, one per file (a-h)
- Each bit represents a rank (bit 0 = rank 1)

**Implementation:**
```rust
// Add OUT pipe during CP210x configuration
static OUT_PIPE_INDEX: Mutex<RefCell<Option<u8>>> = ...;

// In configure():
let out_pipe = usb.alloc_pipe(|p| p.into_stream_out())?;
out_pipe.configure(generic_device.address, 2, Transfer::Bulk);

// New method:
pub fn send_leds(data: &[u8; 8]) -> Result<(), Error> {
    // Use OUT pipe to send LED state
}
```

### 5.4 Main Loop Integration

```rust
// In main.rs main loop:

loop {
    // 1. Poll USB (handles device connection, data reception)
    USB::with(|usb| usb.poll());

    // 2. Check for complete RFID reading
    if let Some(line) = CP210xDevice::read_line() {
        if let Some(reading) = RfidReading::parse(&line) {
            app.on_board_reading(reading);
        }
    }

    // 3. Send LED updates when changed
    if app.led_state_dirty {
        if CP210xDevice::send_leds(app.led_state.as_bytes()).is_ok() {
            app.led_state_dirty = false;
        }
    }

    // 4. Update timer, check buttons, etc.
    // ... existing code ...
}
```

### 5.5 Implementation Order

1. **Fix baud rate** (5 min) - Critical bug fix
2. **Add LineBuffer** (30 min) - New module for line accumulation
3. **Modify CP210x poll()** (30 min) - Push data to buffer, remove delay
4. **Add read_line() API** (15 min) - Expose complete lines
5. **Add OUT pipe** (30 min) - Configure second pipe for LEDs
6. **Add send_leds() API** (15 min) - Transmit LED state
7. **Integrate in main.rs** (30 min) - Wire everything together
8. **Test on hardware** (varies) - Debug with actual Certabo board

### 5.6 Testing Strategy

**Without hardware:**
- Unit tests for LineBuffer (boundary conditions, partial lines, etc.)
- Verify baud rate calculation

**With hardware:**
- Serial debug output showing received bytes
- Verify RFID readings parse correctly
- Test LED response to piece movement
- Full game playthrough

### 5.7 Files to Modify

| File | Changes |
|------|---------|
| `src/usb/device/cp210x.rs` | Fix baud, add buffer, add APIs |
| `src/usb/device/mod.rs` | Export new types if needed |
| `src/usb.rs` | Maybe expose device-level APIs |
| `src/main.rs` | Integrate data flow |
| `src/certabo/mod.rs` | Add LineBuffer module (new) |

---

## Nice-to-Have Features (Future)

### Move Validation (DONE)
- ~~Integrate no_std chess engine~~ Implemented lightweight chess module (~300 lines, no lookup tables)
- Reject illegal moves with error buzzer ✓
- Show legal destinations via LEDs when piece lifted ✓

### Persistence
- Store calibration data in EEPROM/flash
- Survive power cycles without recalibration
- Save/restore game state

### Advanced Chess Rules
- Checkmate/stalemate detection
- Draw by repetition (3-fold)
- 50-move rule
- Insufficient material detection

### User Experience
- Configurable time controls (not just 10+0)
- Pawn promotion piece selection
- Board orientation for black
- Move history display
- Game review mode

### Connectivity (Stretch Goal)
- Bluetooth LE for phone app
- WiFi for online play
- PGN export

---

## Technical Notes

### Memory Constraints
- No heap allocation (no_std, no alloc)
- All buffers statically sized
- 96KB SRAM available on SAM3X8E

### Timing
- Current: TC0 interrupt at 100ms with WFI-based main loop
- Ideal: RTT (Real-Time Timer) interrupt at 1ms (not critical for chess)
- Acceptable drift: ±1 second per 10 minutes

### USB Considerations
- Arduino Due has USB host capability (UOTGHS)
- CP210x appears as CDC-ACM but uses vendor commands
- Baud rate must match Certabo expectations (38400)
- May need handshake/initialization sequence

---

## File Reference

| Module | Purpose |
|--------|---------|
| `src/main.rs` | Application entry, state machine |
| `src/usb.rs` | USB host driver |
| `src/usb/device/cp210x.rs` | CP210x serial adapter |
| `src/certabo/protocol.rs` | RFID message parser |
| `src/certabo/calibration.rs` | RFID → piece mapping |
| `src/certabo/leds.rs` | LED state management |
| `src/certabo/buffer.rs` | Line buffering for USB data |
| `src/game/chess.rs` | Lightweight chess move validation |
| `src/game/state.rs` | Game state machine |
| `src/game/timer.rs` | Chess clock |
| `src/display.rs` | LCD output |
| `src/buzzer.rs` | Audio feedback |
