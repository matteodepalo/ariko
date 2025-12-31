#!/bin/bash
set -e

# Use nightly Rust from /opt/rust-nightly
export PATH="/opt/rust-nightly/bin:$PATH"

echo "=== Building certabo-rs for Arduino Due ==="
cargo build --release

echo "=== Converting ELF to binary ==="
arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/release/certabo certabo.bin

echo "=== Detecting Arduino Due ==="
DEVICE=$(ls /dev/ttyACM* 2>/dev/null | head -n1)

if [ -z "$DEVICE" ]; then
    echo "ERROR: No Arduino Due found on /dev/ttyACM*"
    echo "Make sure the board is connected and in bootloader mode:"
    echo "  1. Press and hold ERASE button for ~2 seconds"
    echo "  2. Press RESET button"
    echo "  3. Run this script again"
    exit 1
fi

echo "Found device: $DEVICE"

echo "=== Flashing to Arduino Due ==="
bossac -p "$DEVICE" -e -w -v -b certabo.bin -R

echo "=== Done! ==="
