//! Diagnostic utilities.

// TODO: get an in-game console working
// TODO: support low-level cpu profiling
// TODO: support low-level memory profiling

pub use gizmos::*;
pub use logging::*;
pub use profiling::*;

mod logging;
mod profiling;
mod gizmos;
