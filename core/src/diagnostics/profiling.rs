//! Profiling utilities

pub use puffin::{profile_function, profile_scope};

pub use macros::profile as profiling;

/// Is the profiler enabled?
#[inline]
pub fn is_profiling_enabled() -> bool {
  puffin::are_scopes_on()
}

/// Enables the profiler.
#[inline]
pub fn enable_profiling() {
  puffin::set_scopes_on(true);
}

/// Disables the profiler.
#[inline]
pub fn disable_profiling() {
  puffin::set_scopes_on(false);
}

/// Notifies the profiler that a frame has completed.
#[inline]
pub fn finish_frame() {
  puffin::GlobalProfiler::lock().new_frame();
}
