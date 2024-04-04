pub use macros::profiling as profiling;

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
  ($name:expr) => {
    // TODO: implement me
  };
  ($name:expr, $args:expr) => {
    $crate::profile_scope!(concat!($name, "(", stringify!($args), ")"));
  };
}

/// Notifies the profiler that a function is executing.
#[macro_export]
macro_rules! profile_function {
  ($name:expr) => {
    $crate::profile_scope!($name);
  };
  ($name:expr, $args:expr) => {
    $crate::profile_scope!($name, $args);
  };
}
