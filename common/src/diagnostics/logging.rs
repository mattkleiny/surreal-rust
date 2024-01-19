/// Writes a trace message to the log.
#[macro_export]
macro_rules! trace {
  ( $($arg:tt)*) => {};
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

/// A sink for log output.
pub trait LogSink {}
