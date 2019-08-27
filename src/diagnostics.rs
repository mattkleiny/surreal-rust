//! A lightweight diagnostics system.

pub use console::*;
pub use logging::*;
pub use profiling::*;

pub use super::*;

mod console;
mod logging;
mod profiling;
