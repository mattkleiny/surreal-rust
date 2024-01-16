//! Profiling utilities

pub use macros::profile as profiling;

/// Is the profiler enabled?
#[inline]
pub fn is_profiling_enabled() -> bool {
  false
}

/// Enables the profiler.
#[inline]
pub fn enable_profiling() {}

/// Disables the profiler.
#[inline]
pub fn disable_profiling() {}

/// Notifies the profiler that a frame has completed.
#[inline]
pub fn finish_frame() {}
