#![no_std]
#![no_main]
#![allow(dead_code)]

extern crate cortex_m_rt;

use core::panic::PanicInfo;

use certabo::buzzer::Buzzer;
use certabo::certabo::calibration::CalibrationData;
use certabo::certabo::leds::LedState;
use certabo::certabo::protocol::RfidReading;
use certabo::display::Display;
use certabo::game::state::{GameState, GameStatus};
use certabo::game::timer::Color;
use certabo::i2c::I2C;
use certabo::logger::Logger;
use certabo::peripherals::Peripherals;
use certabo::serial::Serial;
use certabo::usb::USB;
use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::InputPin;

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
  last_tick_ms: u32,
}

impl App {
  fn new() -> Self {
    Self {
      state: AppState::Initializing,
      calibration: CalibrationData::new(),
      game: GameState::new(),
      last_reading: None,
      led_state: LedState::new(),
      last_tick_ms: 0,
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
          // TODO: Send LED state to board via USB
        }
      }

      // Handle complete move (piece placed)
      if let (Some(from), Some(to)) = (self.game.lifted_piece(), placed_to) {
        if self.game.is_legal_move(from, to) {
          // Valid move
          self.game.make_move(from, to);
          Buzzer::with(|b| b.move_sound());
          self.led_state.clear_all();
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

  // Main loop
  let mut last_time_ms: u32 = 0;

  loop {
    // Check buttons
    Peripherals::with(|p| {
      // Blue button = Calibrate
      if p.blue_button.is_low().unwrap_or(false) {
        app.on_blue_button();
        // Debounce
        p.delay.delay_ms(200);
      }

      // White button = Pause/Resume
      if p.white_button.is_low().unwrap_or(false) {
        app.on_white_button();
        // Debounce
        p.delay.delay_ms(200);
      }
    });

    // Poll USB for board data
    USB::with(|usb| {
      usb.poll();
      // TODO: Check if data received and parse as RfidReading
      // if let Some(data) = usb.get_received_data() {
      //   if let Some(reading) = RfidReading::parse(&data) {
      //     app.on_board_reading(reading);
      //   }
      // }
    });

    // Update timer (approximate timing using delay)
    // In real implementation, use RTT or hardware timer
    let current_time_ms = last_time_ms + 10; // Approximate 10ms per loop
    let elapsed = current_time_ms - last_time_ms;
    app.tick(elapsed);
    last_time_ms = current_time_ms;

    // Small delay to prevent busy loop
    Peripherals::with(|p| p.delay.delay_ms(10));
  }
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
