use std::fmt::Display;

pub use console::*;

/// A level for log messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
  Trace,
  Debug,
  Info,
  Warn,
  Error,
}

impl Display for LogLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      LogLevel::Trace => write!(f, "TRACE"),
      LogLevel::Debug => write!(f, "DEBUG"),
      LogLevel::Info => write!(f, "INFO"),
      LogLevel::Warn => write!(f, "WARN"),
      LogLevel::Error => write!(f, "ERROR"),
    }
  }
}

/// A sink for log output.
pub trait Log {
  /// Determines whether the given level is enabled.
  fn is_level_enabled(&self, level: LogLevel) -> bool;

  /// Logs a message at the given [`LogLevel`].
  fn log(&self, level: LogLevel, message: &str);

  #[inline(always)]
  fn trace(&self, message: &str) {
    self.log(LogLevel::Trace, message)
  }

  #[inline(always)]
  fn debug(&self, message: &str) {
    self.log(LogLevel::Debug, message)
  }

  #[inline(always)]
  fn info(&self, message: &str) {
    self.log(LogLevel::Info, message)
  }

  #[inline(always)]
  fn warn(&self, message: &str) {
    self.log(LogLevel::Warn, message)
  }

  #[inline(always)]
  fn error(&self, message: &str) {
    self.log(LogLevel::Error, message)
  }
}

/// Writes a trace message to the log.
#[macro_export]
macro_rules! trace {
  ($($arg:tt)*) => {};
}

/// Writes a debug message to the log.
#[macro_export]
macro_rules! debug {
  ($($arg:tt)*) => {};
}

/// Writes an info message to the log.
#[macro_export]
macro_rules! info {
  ($($arg:tt)*) => {};
}

/// Writes a warning message to the log.
#[macro_export]
macro_rules! warn {
  ($($arg:tt)*) => {};
}

/// Writes an error message to the log.
#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => {};
}

mod console {
  //! A console log sink.
  use std::borrow::Cow;

  use super::*;

  /// A console log sink.
  pub struct ConsoleLog {
    name: Cow<'static, str>,
    min_level: LogLevel,
  }

  impl ConsoleLog {
    /// Creates a new console log sink.
    pub fn new_target(name: impl AsRef<str>, min_level: LogLevel) -> ConsoleLog {
      ConsoleLog {
        name: Cow::Owned(name.as_ref().to_owned()),
        min_level,
      }
    }

    /// Creates a new console log sink for the current module.
    pub const fn new_module(min_level: LogLevel) -> ConsoleLog {
      ConsoleLog {
        name: Cow::Borrowed(std::module_path!()),
        min_level,
      }
    }
  }

  impl Log for ConsoleLog {
    fn is_level_enabled(&self, level: LogLevel) -> bool {
      level >= self.min_level
    }

    fn log(&self, level: LogLevel, message: &str) {
      if self.is_level_enabled(level) {
        println!("{} [{}]: {}", self.name, level, message);
      }
    }
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    static LOG: ConsoleLog = ConsoleLog::new_module(LogLevel::Trace);

    #[test]
    fn test_console_log_operations() {
      LOG.trace("trace");
      LOG.debug("debug");
      LOG.info("info");
      LOG.warn("warn");
    }
  }
}
