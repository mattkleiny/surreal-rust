//! Logging utilities.

/// A level for a logger.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum LogLevel {
  Trace = 0,
  Debug = 1,
  Warn = 2,
  Error = 3,
}

/// A logger for some component.
pub struct Logger {
  name: &'static str,
  level: LogLevel,
}

impl Logger {
  pub fn new(name: &'static str, level: LogLevel) -> Self {
    Self { name, level }
  }

  #[inline]
  pub fn trace(&mut self, message: &String) {
    self.write(message, LogLevel::Trace)
  }

  #[inline]
  pub fn debug(&mut self, message: &String) {
    self.write(message, LogLevel::Debug)
  }

  #[inline]
  pub fn warn(&mut self, message: &String) {
    self.write(message, LogLevel::Warn)
  }

  #[inline]
  pub fn error(&mut self, message: &String) {
    self.write(message, LogLevel::Error)
  }

  pub fn write(&mut self, _message: &String, level: LogLevel) {
    if level >= self.level {
      unimplemented!()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_write_messages() {
    let mut logger = Logger::new("Some Component", LogLevel::Trace);

    logger.trace(&"It's working!".to_string());
  }
}