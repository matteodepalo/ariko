use crate::peripherals::Peripherals;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::OutputPin;
use sam3x8e_hal::gpio::Floating;

//  --0x01--
// |        |
//0x20     0x02
// |        |
//  --0x40- -
// |        |
//0x10     0x04
// |        |
//  --0x08--

const ADDR_AUTO: u8 = 0x40;
const BRIGHT_TYPICAL: u8 = 2;
const CMD_SET_ADDR: u8 = 0xc0;
const CMD_DISP_CTRL: u8 = 0x88 + BRIGHT_TYPICAL;

// TODO: Add clk_pin

pub struct TM1637<T: 'static, G: 'static> {
  clk_pin: &'static mut T,
  data_pin: &'static mut G,
}

impl<T: OutputPin, G: OutputPin> TM1637<T, G> {
  pub fn new(clk_pin: &'static mut T, data_pin: &'static mut G) -> Self {
    TM1637 { data_pin, clk_pin }
  }

  fn write_byte(&mut self, mut data: u8) {
    for _i in 0..7 {
      // Sent 8bit data
      self.clk_pin.try_set_low();

      if data & 0x01 != 0x00 {
        self.data_pin.try_set_high(); // LSB first
      } else {
        self.data_pin.try_set_low();
      }

      data >>= 1;
      self.clk_pin.try_set_high();
    }

    self.clk_pin.try_set_low(); // Wait for the ACK
    self.data_pin.try_set_high();
    self.clk_pin.try_set_high();
    // pinMode(datapin, INPUT);

    bit_delay();
    // let ack = digitalRead(datapin);

    // if ack == 0 {
    //   pinMode(datapin, OUTPUT);
    //   self.data_pin.try_set_low();
    // }

    // bit_delay();
    // pinMode(datapin, OUTPUT);
    bit_delay();

    // return ack;
  }

  pub fn clear(&mut self) {
    self.display("AAAA");
  }

  // TODO: implement display for single address and displayStr

  pub fn display(&mut self, string: &str) {
    let data = string.chars().map(|char| char_to_segment(char));

    self.start(); // Start signal sent to TM1637 from MCU
    self.write_byte(ADDR_AUTO); // Command1: Set data
    self.stop();
    self.start();
    self.write_byte(CMD_SET_ADDR); // Command2: Set address (automatic address adding)

    for value in data {
      self.write_byte(value); // Transfer display data (8 bits x num_of_digits)
    }

    self.stop();
    self.start();
    self.write_byte(CMD_DISP_CTRL); // Control display
    self.stop();
  }

  fn start(&mut self) {
    self.clk_pin.try_set_high();
    self.data_pin.try_set_high();
    self.data_pin.try_set_low();
    self.clk_pin.try_set_low();
  }

  fn stop(&mut self) {
    self.clk_pin.try_set_low();
    self.data_pin.try_set_low();
    self.clk_pin.try_set_high();
    self.data_pin.try_set_high();
  }
}

fn bit_delay() {
  Peripherals::get().delay.try_delay_us(50_u32);
}

fn char_to_segment(value: char) -> u8 {
  match value {
    '0' => 0x3f,
    '1' => 0x06,
    '2' => 0x5b,
    '3' => 0x4f,
    '4' => 0x66,
    '5' => 0x6d,
    '6' => 0x7d,
    '7' => 0x07,
    '8' => 0x7f,
    '9' => 0x6f,
    '_' => 0x08,
    '^' => 0x01,
    '-' => 0x40,
    '*' => 0x63,
    ' ' => 0x00,
    'A' => 0x77,
    'a' => 0x5f,
    'b' | 'B' => 0x7c,
    'C' => 0x39,
    'c' => 0x58,
    'd' | 'D' => 0x5e,
    'e' | 'E' => 0x79,
    'f' | 'F' => 0x71,
    'g' | 'G' => 0x35,
    'H' => 0x76,
    'h' => 0x74,
    'I' => 0x06,
    'i' => 0x04,
    'J' => 0x1e,
    'j' => 0x16,
    'k' | 'K' => 0x75,
    'l' | 'L' => 0x38,
    'm' | 'M' => 0x37,
    'n' | 'N' => 0x54,
    'o' | 'O' => 0x5c,
    'p' | 'P' => 0x73,
    'Q' => 0x7b,
    'q' => 0x67,
    'r' | 'R' => 0x50,
    's' | 'S' => 0x6d,
    't' | 'T' => 0x78,
    'u' | 'U' => 0x1c,
    'v' | 'V' => 0x3e,
    'W' => 0x7e,
    'w' => 0x2a,
    'x' | 'X' => 0x76,
    'y' | 'Y' => 0x6e,
    'z' | 'Z' => 0x1b,
    _ => 0x00,
  }
}
