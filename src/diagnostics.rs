//! A lightweight diagnostics system.

pub use console::*;
pub use logging::*;

#[cfg(feature = "ui")]
mod console;
mod logging;

// TODO: get an in-game console working
// TODO: support low-level cpu profiling
// TODO: support low-level memory profiling