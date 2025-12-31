#![no_std]
#![no_main]
#![allow(dead_code)]

extern crate cortex_m_rt;

use core::panic::PanicInfo;
use core::sync::atomic::Ordering;

use certabo::buzzer::Buzzer;
use certabo::certabo::buffer::MAX_LINE_LEN;
use certabo::certabo::calibration::CalibrationData;
use certabo::certabo::leds::LedState;
use certabo::certabo::protocol::RfidReading;
use certabo::display::Display;
use certabo::events::{consume, BLUE_BUTTON_PRESSED, MILLIS, TIMER_TICK, WHITE_BUTTON_PRESSED};
use certabo::game::state::{GameState, GameStatus};
use certabo::game::timer::Color;
use certabo::i2c::I2C;
use certabo::logger::Logger;
use certabo::peripherals::Peripherals;
use certabo::serial::Serial;
use certabo::usb::{CP210xDevice, USB};
use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use sam3x8e_hal::pac::{PIOB, PIOC, TC0};

/// Application state machine
#[derive(Clone, Copy, Debug, PartialEq)]
enum AppState {
  /// System initializing
  Initializing,
  /// Waiting for user to press calibrate button
  WaitingForCalibration,
  /// Calibration in progress
  Calibrating,
  /// Waiting for pieces to be set up in starting position
  WaitingForSetup,
  /// Game in progress
  GameInProgress,
  /// Game paused
  GamePaused,
  /// Game ended
  GameEnded,
}

/// Main application context
struct App {
  state: AppState,
  calibration: CalibrationData,
  game: GameState,
  last_reading: Option<RfidReading>,
  led_state: LedState,
  led_dirty: bool,
}

impl App {
  fn new() -> Self {
    Self {
      state: AppState::Initializing,
      calibration: CalibrationData::new(),
      game: GameState::new(),
      last_reading: None,
      led_state: LedState::new(),
      led_dirty: false,
    }
  }

  /// Handle blue button press (Calibrate)
  fn on_blue_button(&mut self) {
    match self.state {
      AppState::WaitingForCalibration | AppState::WaitingForSetup | AppState::GameEnded => {
        self.state = AppState::Calibrating;
        Display::with(|d| d.show_calibration_prompt());
      }
      _ => {}
    }
  }

  /// Handle white button press (Pause/Resume)
  fn on_white_button(&mut self) {
    match self.state {
      AppState::GameInProgress => {
        self.state = AppState::GamePaused;
        self.game.set_status(GameStatus::Paused);
        Display::with(|d| d.show_paused());
      }
      AppState::GamePaused => {
        self.state = AppState::GameInProgress;
        self.game.set_status(GameStatus::InProgress);
      }
      AppState::GameEnded => {
        // Reset for new game
        self.game.reset();
        self.state = AppState::WaitingForSetup;
        Display::with(|d| d.show_waiting_for_setup());
      }
      _ => {}
    }
  }

  /// Process new RFID reading from board
  fn on_board_reading(&mut self, reading: RfidReading) {
    match self.state {
      AppState::Calibrating => {
        self.do_calibration(&reading);
      }
      AppState::WaitingForSetup => {
        self.check_starting_position(&reading);
      }
      AppState::GameInProgress => {
        self.process_game_move(&reading);
      }
      _ => {}
    }

    self.last_reading = Some(reading);
  }

  /// Perform calibration from current board reading
  fn do_calibration(&mut self, reading: &RfidReading) {
    let count = self.calibration.calibrate_from_starting_position(reading);

    Display::with(|d| d.show_calibration_progress(count as u8));

    if self.calibration.is_complete() {
      self.state = AppState::WaitingForSetup;
      Buzzer::with(|b| b.calibration_complete());
      Display::with(|d| d.show_calibration_complete());

      // Brief delay to show completion message
      Peripherals::with(|p| p.delay.delay_ms(1500));
      Display::with(|d| d.show_waiting_for_setup());
    }
  }

  /// Check if pieces are in starting position
  fn check_starting_position(&mut self, reading: &RfidReading) {
    let board = self.calibration.reading_to_board(reading);

    // Count pieces in correct positions
    let mut correct = 0;
    for square in 0..64u8 {
      if board[square as usize].is_some() {
        correct += 1;
      }
    }

    // If all 32 pieces detected, start the game
    if correct >= 32 {
      self.state = AppState::GameInProgress;
      self.game.set_status(GameStatus::InProgress);
      Buzzer::with(|b| b.move_sound());
      self.update_display();
    }
  }

  /// Process potential move during game
  fn process_game_move(&mut self, reading: &RfidReading) {
    let current_board = self.calibration.reading_to_board(reading);

    if let Some(ref last_reading) = self.last_reading {
      let previous_board = self.calibration.reading_to_board(last_reading);

      // Detect changes between readings
      let mut lifted_from: Option<u8> = None;
      let mut placed_to: Option<u8> = None;

      for square in 0..64u8 {
        let prev = previous_board[square as usize];
        let curr = current_board[square as usize];

        if prev.is_some() && curr.is_none() {
          // Piece was lifted
          lifted_from = Some(square);
        } else if prev.is_none() && curr.is_some() {
          // Piece was placed
          placed_to = Some(square);
        }
      }

      // Handle piece lift (show legal moves)
      if let Some(from) = lifted_from {
        if placed_to.is_none() {
          self.game.piece_lifted(from);
          let destinations = self.game.legal_destinations(from);
          self.led_state.clear_all();
          self.led_state.set(from);
          for dest in destinations {
            self.led_state.set(dest);
          }
          self.led_dirty = true;
        }
      }

      // Handle complete move (piece placed)
      if let (Some(from), Some(to)) = (self.game.lifted_piece(), placed_to) {
        if self.game.is_legal_move(from, to) {
          // Valid move
          self.game.make_move(from, to);
          Buzzer::with(|b| b.move_sound());
          self.led_state.clear_all();
          self.led_dirty = true;
        } else {
          // Invalid move
          Buzzer::with(|b| b.error_sound());
          Display::with(|d| d.show_invalid_move());
        }

        // Check for game end
        self.check_game_end();
        self.update_display();
      }
    }
  }

  /// Check if game has ended
  fn check_game_end(&mut self) {
    match self.game.status() {
      GameStatus::WhiteWins => {
        self.state = AppState::GameEnded;
        Buzzer::with(|b| b.game_over_sound());
        Display::with(|d| d.show_game_over("White", "Checkmate"));
      }
      GameStatus::BlackWins => {
        self.state = AppState::GameEnded;
        Buzzer::with(|b| b.game_over_sound());
        Display::with(|d| d.show_game_over("Black", "Checkmate"));
      }
      GameStatus::Draw => {
        self.state = AppState::GameEnded;
        Buzzer::with(|b| b.game_over_sound());
        Display::with(|d| d.show_draw("Stalemate"));
      }
      _ => {}
    }
  }

  /// Update timer and check for timeout
  fn tick(&mut self, elapsed_ms: u32) {
    if self.state != AppState::GameInProgress {
      return;
    }

    let current_turn = self.game.current_turn();

    // Check for low time warning
    if self.game.timer().is_low_time(current_turn) && elapsed_ms % 1000 < 50 {
      Buzzer::with(|b| b.low_time_warning());
    }

    // Update timer
    if self.game.tick_timer(elapsed_ms) {
      // Time expired
      self.state = AppState::GameEnded;
      Buzzer::with(|b| b.time_expired());

      let winner = match current_turn {
        Color::White => "Black",
        Color::Black => "White",
      };
      Display::with(|d| d.show_game_over(winner, "Time"));
    }
  }

  /// Update display with current game status
  fn update_display(&self) {
    if self.state != AppState::GameInProgress {
      return;
    }

    let turn = self.game.current_turn();
    let (minutes, seconds) = self.game.timer().formatted_time(turn);
    let is_white = matches!(turn, Color::White);

    Display::with(|d| {
      d.show_game_status(is_white, minutes, seconds, "Your move");
    });
  }
}

#[entry]
fn main() -> ! {
  // Initialize peripherals
  Peripherals::init();
  Serial::init(57600);
  Logger::init();
  I2C::init();
  Display::init();
  USB::init();
  Buzzer::init();

  // Create application context
  let mut app = App::new();

  // Show startup message
  Display::with(|d| {
    d.write_str("Certabo Chess").unwrap();
  });
  Peripherals::with(|p| p.delay.delay_ms(1000));

  // Enter waiting for calibration state
  app.state = AppState::WaitingForCalibration;
  Display::with(|d| {
    d.show_calibration_prompt();
  });

  // Main loop - event driven with WFI
  loop {
    // Process button events (set by interrupt handlers)
    if consume(&BLUE_BUTTON_PRESSED) {
      app.on_blue_button();
    }

    if consume(&WHITE_BUTTON_PRESSED) {
      app.on_white_button();
    }

    // Process timer tick (100ms, set by TC0 interrupt)
    if consume(&TIMER_TICK) {
      app.tick(100);
      app.update_display();
    }

    // Poll USB for board data (still polling, but runs on each wake)
    USB::with(|usb| usb.poll());

    // Check for complete RFID reading from Certabo board
    let mut line_buffer = [0u8; MAX_LINE_LEN];
    if let Some(len) = CP210xDevice::read_line(&mut line_buffer) {
      if let Some(reading) = RfidReading::parse(&line_buffer[..len]) {
        app.on_board_reading(reading);
      }
    }

    // Send LED updates to board when changed
    if app.led_dirty {
      let _ = CP210xDevice::send_leds(app.led_state.as_bytes());
      app.led_dirty = false;
    }

    // Sleep until next interrupt (button, timer, or any other)
    cortex_m::asm::wfi();
  }
}

// Interrupt handlers
// These are extern "C" functions that match the vector table entries

#[unsafe(no_mangle)]
pub extern "C" fn PIOB() {
  // Read ISR to clear interrupt and get triggered pins
  let isr = unsafe { (*PIOB::ptr()).isr().read().bits() };

  // Blue button on PB25
  if isr & (1 << 25) != 0 {
    BLUE_BUTTON_PRESSED.store(true, Ordering::SeqCst);
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn PIOC() {
  // Read ISR to clear interrupt and get triggered pins
  let isr = unsafe { (*PIOC::ptr()).isr().read().bits() };

  // White button on PC28
  if isr & (1 << 28) != 0 {
    WHITE_BUTTON_PRESSED.store(true, Ordering::SeqCst);
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn TC0() {
  // Read SR to clear interrupt flag
  let _ = unsafe { (*TC0::ptr()).sr0().read() };

  // Increment monotonic counter by 100ms
  MILLIS.fetch_add(100, Ordering::Relaxed);

  // Signal timer tick to main loop
  TIMER_TICK.store(true, Ordering::SeqCst);
}

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
