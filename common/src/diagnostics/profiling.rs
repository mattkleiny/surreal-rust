pub use macros::profile as profiling;

/// A sink for profiling output.
pub trait Profiler {}

/// Notifies the profiler that a frame has started.
#[macro_export]
macro_rules! profile_frame_start {
  () => {
    todo!();
  };
}

/// Notifies the profiler that a scope has started.
#[macro_export]
macro_rules! profile_scope {
  ($name:expr) => {
    todo!();
  };
  ($name:expr, $args:expr) => {
    todo!();
  };
}

/// Notifies the profiler that a function has started.
#[macro_export]
macro_rules! profile_function {
  ($name:expr) => {
    todo!();
  };
}

/// Notifies the profiler that a frame has ended.
#[macro_export]
macro_rules! profile_frame_end {
  () => {
    todo!();
  };
}
