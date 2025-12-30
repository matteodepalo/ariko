use crate::serial::Serial;
use core::fmt::Write;
use log::LevelFilter;
use log::{Metadata, Record};

pub struct Logger;

static LOGGER: Logger = Logger;

impl Logger {
  pub fn init() {
    log::set_logger(&LOGGER)
      .map(|_| log::set_max_level(LevelFilter::Debug))
      .unwrap()
  }
}

impl log::Log for Logger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= log::max_level()
  }

  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      Serial::with(|serial| {
        serial
          .write_fmt(format_args!("[{}] {}\n\r", record.level(), record.args()))
          .unwrap();
      });
    }
  }

  fn flush(&self) {}
}
