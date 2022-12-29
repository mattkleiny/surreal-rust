//! Diagnostic utilities for the engine.

pub use log::{debug, error, info, trace, warn, Level, LevelFilter};

/// A builder pattern for [`ConsoleLogger`].
pub struct ConsoleLoggerBuilder {
  level: LevelFilter,
}

impl ConsoleLoggerBuilder {
  pub fn new() -> Self {
    Self { level: LevelFilter::Info }
  }

  pub fn with_level(mut self, level: LevelFilter) -> Self {
    self.level = level;
    self
  }

  /// Installs this logger as the global logger.
  pub fn install(self) {
    let logger = self.build();

    log::set_max_level(LevelFilter::Trace);
    log::set_logger(logger).expect("Failed to set logger");
  }

  /// Builds a [`ConsoleLogger`] with the given configuration in static context.
  fn build(self) -> &'static mut ConsoleLogger {
    Box::leak(Box::new(ConsoleLogger { level: self.level }))
  }
}

/// A simple [`log::Log`] that logs to the console.
struct ConsoleLogger {
  level: LevelFilter,
}

impl log::Log for ConsoleLogger {
  fn enabled(&self, metadata: &log::Metadata) -> bool {
    metadata.level() >= self.level
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
    let timestamp = chrono::Local::now();

    println!(
      "{:<5} {} - [{}] [{}] {}",
      level,
      timestamp.format("%Y-%m-%d %H:%M:%S:%f"),
      target,
      thread,
      record.args()
    );
  }

  fn flush(&self) {
    // no-op
  }
}
