//! Integration tests for display command sequences
//!
//! These tests verify LCD display command generation and cursor positioning
//! based on the HD44780/JHD1802 display controller protocol.

use certabo::jhd1802::calculate_cursor_address;

/// HD44780 LCD command constants
mod lcd_commands {
    pub const CLEAR_DISPLAY: u8 = 0x01;
    pub const RETURN_HOME: u8 = 0x02;
    pub const ENTRY_MODE_SET: u8 = 0x04;
    pub const DISPLAY_CONTROL: u8 = 0x08;
    pub const CURSOR_SHIFT: u8 = 0x10;
    pub const FUNCTION_SET: u8 = 0x20;
    pub const SET_CGRAM_ADDR: u8 = 0x40;
    pub const SET_DDRAM_ADDR: u8 = 0x80;
}

/// Entry mode flags
mod entry_mode {
    pub const INCREMENT: u8 = 0x02;
    pub const DECREMENT: u8 = 0x00;
    pub const SHIFT_DISPLAY: u8 = 0x01;
}

/// Display control flags
mod display_control {
    pub const DISPLAY_ON: u8 = 0x04;
    pub const CURSOR_ON: u8 = 0x02;
    pub const BLINK_ON: u8 = 0x01;
}

/// Function set flags
mod function_set {
    pub const EIGHT_BIT: u8 = 0x10;
    pub const TWO_LINE: u8 = 0x08;
    pub const LARGE_FONT: u8 = 0x04;
}

#[test]
fn test_cursor_addressing_first_row() {
    // First row starts at DDRAM address 0x00
    // Set DDRAM address command has bit 7 set
    for col in 0..16u8 {
        let addr = calculate_cursor_address(col, 0);
        assert_eq!(addr, lcd_commands::SET_DDRAM_ADDR | col);
        assert!(addr >= 0x80 && addr < 0x90);
    }
}

#[test]
fn test_cursor_addressing_second_row() {
    // Second row starts at DDRAM address 0x40
    // Set DDRAM address command: 0x80 | 0x40 = 0xC0
    for col in 0..16u8 {
        let addr = calculate_cursor_address(col, 1);
        assert_eq!(addr, 0xC0 | col);
        assert!(addr >= 0xC0 && addr < 0xD0);
    }
}

#[test]
fn test_init_sequence_function_set() {
    // JHD1802 initialization: 4-bit mode, 2 lines
    let function_set_2line = lcd_commands::FUNCTION_SET | function_set::TWO_LINE;
    assert_eq!(function_set_2line, 0x28);
}

#[test]
fn test_init_sequence_display_on() {
    // Display on, cursor off, blink off
    let display_on = lcd_commands::DISPLAY_CONTROL | display_control::DISPLAY_ON;
    assert_eq!(display_on, 0x0C);
}

#[test]
fn test_init_sequence_entry_mode() {
    // Entry mode: increment cursor, no display shift
    let entry_mode = lcd_commands::ENTRY_MODE_SET | entry_mode::INCREMENT;
    assert_eq!(entry_mode, 0x06);
}

#[test]
fn test_clear_display_command() {
    assert_eq!(lcd_commands::CLEAR_DISPLAY, 0x01);
}

#[test]
fn test_return_home_command() {
    assert_eq!(lcd_commands::RETURN_HOME, 0x02);
}

#[test]
fn test_ddram_address_boundaries() {
    // 16x2 LCD has 16 characters per row
    // Row 0: addresses 0x00-0x0F (displayed as 0x80-0x8F)
    // Row 1: addresses 0x40-0x4F (displayed as 0xC0-0xCF)

    // First character of first row
    assert_eq!(calculate_cursor_address(0, 0), 0x80);

    // Last character of first row
    assert_eq!(calculate_cursor_address(15, 0), 0x8F);

    // First character of second row
    assert_eq!(calculate_cursor_address(0, 1), 0xC0);

    // Last character of second row
    assert_eq!(calculate_cursor_address(15, 1), 0xCF);
}

#[test]
fn test_full_display_control_options() {
    // All display options off
    let all_off = lcd_commands::DISPLAY_CONTROL;
    assert_eq!(all_off, 0x08);

    // Display on only
    let display_only = lcd_commands::DISPLAY_CONTROL | display_control::DISPLAY_ON;
    assert_eq!(display_only, 0x0C);

    // Display and cursor on
    let with_cursor =
        lcd_commands::DISPLAY_CONTROL | display_control::DISPLAY_ON | display_control::CURSOR_ON;
    assert_eq!(with_cursor, 0x0E);

    // Display, cursor, and blink all on
    let all_on = lcd_commands::DISPLAY_CONTROL
        | display_control::DISPLAY_ON
        | display_control::CURSOR_ON
        | display_control::BLINK_ON;
    assert_eq!(all_on, 0x0F);
}

#[test]
fn test_text_wrapping_boundary() {
    // When text reaches column 16, it should wrap to row 1
    // Column 15 is the last position on row 0
    let last_col_row0 = calculate_cursor_address(15, 0);
    assert_eq!(last_col_row0, 0x8F);

    // Column 0 on row 1 is where wrapping should go
    let first_col_row1 = calculate_cursor_address(0, 1);
    assert_eq!(first_col_row1, 0xC0);

    // The gap between 0x8F and 0xC0 is intentional - that's how HD44780 works
    assert_eq!(first_col_row1 - last_col_row0, 0x31); // 49 addresses gap
}
