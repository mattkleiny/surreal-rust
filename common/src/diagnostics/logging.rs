//! Logging support for Surreal.

pub use log::{debug, error, info, trace, warn, Level, LevelFilter};

/// A simple [`log::Log`] that logs to the console.
pub struct ConsoleLogger {
  level: LevelFilter,
}

impl ConsoleLogger {
  /// Installs the [`ConsoleLogger`] as the main logger.
  pub fn install(level: LevelFilter) {
    let logger = Box::leak(Box::new(ConsoleLogger { level }));

    // ignore this; it fails setting twice during integration tests
    log::set_max_level(LevelFilter::Trace);
    let _ = log::set_logger(logger);
  }
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
