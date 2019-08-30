//! Logging utilities.

use chrono::Timelike;
use log::{LevelFilter, Log, Metadata, Record};
pub use log::{debug, error, info, trace, warn};

/// The static logger for Surreal.
pub static LOGGER: Logger = Logger;

/// The standard logger for Surreal.
pub struct Logger;

impl Logger {
  pub fn install() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .expect("Failed to set system logger!");
  }
}

impl Log for Logger {
  fn enabled(&self, _metadata: &Metadata) -> bool { true }

  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      // TODO: better formatting for thread id
      let thread_id = std::thread::current().id();

      let now = chrono::Local::now();
      let (is_pm, hour) = now.hour12();

      println!(
        "{:02}:{:02}:{:02} {} - <{:?}> {} [{}]: {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" },
        thread_id,
        record.target(),
        record.level(),
        record.args()
      );
    }
  }

  fn flush(&self) {}
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn logger_should_write_messages() {
    Logger::install();

    trace!("It's working!");
    debug!("It's working!");
    warn!("It's working!");
    error!("It's working!");
  }
}