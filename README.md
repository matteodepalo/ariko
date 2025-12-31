# Certabo-rs

Standalone chess clock and game controller for Arduino Due + Certabo electronic chess board.

Play a complete game of chess using only the physical board—no computer or phone required during gameplay.

## Features

- Piece position detection via RFID
- Move validation with legal move highlighting
- Chess clock (10 minutes per side)
- Audio feedback (buzzer)
- Visual feedback (LCD display + board LEDs)

## Hardware Requirements

- Arduino Due (SAM3X8E)
- Certabo electronic chess board (with CP210x USB adapter)
- JHD1802 LCD display (I2C)
- Buzzer (PWM)
- Two buttons (Blue and White)

## Building & Flashing

```bash
# Build (requires nightly Rust)
PATH="/opt/rust-nightly/bin:$PATH" cargo build --release

# Flash to Arduino Due
./deploy.sh
```

Before flashing, put the Arduino Due in bootloader mode:
1. Hold the ERASE button for ~2 seconds
2. Press RESET

## How to Play

### 1. Power On

Connect the Certabo board to the Arduino Due via USB. The LCD will show:

```
Certabo Chess
```

Then prompt for calibration:

```
Press BLUE to
calibrate
```

### 2. Calibrate the Board

Place all 32 pieces in their starting positions, then press the **Blue button**.

The board reads the RFID chips in each piece and learns which chip belongs to which piece. The LCD shows progress:

```
Calibrating...
Found: 32/32
```

When complete, you'll hear a confirmation beep.

### 3. Set Up for a New Game

After calibration, ensure all pieces are in the standard starting position:

```
Set up pieces
in start pos
```

The game starts automatically when all 32 pieces are detected in their correct squares.

### 4. Play!

The LCD shows the current player and remaining time:

```
WHITE  09:45
Your move
```

**Making a move:**
1. Lift a piece — LEDs light up showing all legal destination squares
2. Place the piece on a legal square — you'll hear a confirmation beep
3. The clock switches to the other player

**Illegal moves:**
- If you place a piece on an illegal square, you'll hear an error buzzer
- Pick up the piece and try again

### 5. Pause/Resume

Press the **White button** to pause the game. The clock stops and the LCD shows:

```
PAUSED
```

Press **White** again to resume.

### 6. Game End

The game ends when:
- **Time runs out** — the player whose clock expires loses
- **Checkmate** — the checkmated player loses
- **Stalemate** — the game is a draw

After the game ends:
- Press **White** to start a new game (keeps calibration)
- Press **Blue** to recalibrate (if pieces changed)

## Button Reference

| Button | During Game | Other States |
|--------|-------------|--------------|
| Blue   | —           | Start calibration |
| White  | Pause/Resume | New game (after game ends) |

## LED Indicators

- **Lifted piece square** — LED on
- **Legal destinations** — LEDs on
- All LEDs turn off after completing a move

## Time Control

Fixed 10+0 (10 minutes per side, no increment).

When a player has less than 30 seconds remaining, a warning beep sounds each second.

## Troubleshooting

**Board not detected:**
- Check USB connection between Certabo and Arduino
- Ensure Certabo board is powered on

**Pieces not recognized:**
- Recalibrate (press Blue button)
- Ensure pieces have RFID chips installed
- Check that pieces are centered on squares

**Wrong piece detected:**
- Recalibrate with pieces in correct starting positions
- Avoid moving pieces during calibration

## Resources

### Arduino Due
- https://github.com/Shock-1/sam3x8e-hal
- https://github.com/inferiorhumanorgans/sam3x8e-rs
- https://github.com/compenguy/atsam3xa

### Display
- https://github.com/JohnDoneth/jhd1802-driver

### Libraries
- https://github.com/arduino/ArduinoCore-sam/blob/master/libraries/Wire/src/Wire.cpp
- https://github.com/arduino/ArduinoCore-sam/blob/master/system/libsam/source/twi.c

## License

MIT
