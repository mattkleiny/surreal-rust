//! Logging utilities.

use chrono::Timelike;
use log::{LevelFilter, Log, Metadata, Record};
pub use log::{debug, error, info, trace, warn};

static LOGGER: Logger = Logger;

/// The standard logger for Surreal.
struct Logger;

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

/// Installs the default system logger.
pub fn install_default_logger() {
  log::set_logger(&LOGGER)
      .map(|()| log::set_max_level(LevelFilter::Trace))
      .expect("Failed to set system logger!");
}