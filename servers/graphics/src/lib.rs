//! A cross-platform graphics engine for Surreal.
//!
//! The engine is split into different `pipeline`s to allow specific targeting
//! of different project goals, and `server`s to allow decouple from underlying
//! graphics APIs.

pub use server::*;
pub use pipeline::*;

mod server;
mod pipeline;
