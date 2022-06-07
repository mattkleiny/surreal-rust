//! Diagnostic utilities for the engine.

pub use log::{debug, error, info, trace, warn, Level, LevelFilter};

/// A simple `log` that logs to the console.
/// 
/// This implementation provides no buffering and logs to the console immediately.
pub struct ConsoleLogger {}

impl ConsoleLogger {
  /// Installs the console logger as the default logger.
  pub fn install(level: LevelFilter) {
    let logger = &&ConsoleLogger {};

    log::set_max_level(level);
    log::set_logger(logger).expect("Failed to set logger");
  }
}

impl log::Log for ConsoleLogger {
  fn enabled(&self, _metadata: &log::Metadata) -> bool {
    true
  }

  fn log(&self, record: &log::Record) {
    let level = record.level().to_string();

    let target = if !record.target().is_empty() {
      record.target()
    } else {
      record.module_path().unwrap_or_default()
    };

    let thread = std::thread::current();
    let thread = thread.name().unwrap_or("?");

    println!(
      "{:<5} - [{}] [{}] {}",
      // timestamp, // TODO: add basic timestamp formatting
      level,
      target,
      thread,
      record.args()
    );
  }

  fn flush(&self) {}
}
