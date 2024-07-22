/// A sink for profiling output.
pub trait Profiler {}

/// Notifies the profiler that a frame has started.
#[macro_export]
macro_rules! profile_frame_start {
  () => {
    // TODO: implement me
  };
}

/// Notifies the profiler that a frame has ended.
#[macro_export]
macro_rules! profile_frame_end {
  () => {
    // TODO: implement me
  };
}

/// Notifies the profiler that a scope has started.
#[macro_export]
macro_rules! profile_scope {
  ($($arg:tt)*) => {
    // TODO: implement me
  };
}

/// Notifies the profiler that a function is executing.
#[macro_export]
macro_rules! profile_function {
  ($($arg:tt)*) => {
    $crate::profile_scope!($($arg)*);
  };
}

#[cfg(test)]
mod tests {
  #[test]
  fn test() {
    profile_function!("{}", "test");
  }
}
