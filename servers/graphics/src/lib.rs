//! A cross-platform graphics engine for Surreal.
//!
//! The engine is split into different [`pipeline`]s to allow specific targeting
//! of different project goals, and [`backend`]s to allow decouple from underlying
//! graphics APIs.

pub use backend::*;
pub use pipeline::*;

mod backend;
mod pipeline;
