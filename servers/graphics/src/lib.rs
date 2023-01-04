//! A cross-platform graphics engine for Surreal.
//!
//! The engine is split into different 'pipelines' to allow specific targeting
//! of different project goals.

pub use backend::*;
pub use pipeline::*;

mod backend;
mod pipeline;
