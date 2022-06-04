//! Scripting support for the engine.
//!
//! The eventual goal for scripting in Surreal is to provide a common runtime
//! for all manner of different scripting languages, each with a unified API.
//!
//! Allowing interoperation between different languages allows for the application
//! of the best tool for a particular job whilst still allowing the engine to
//! take care of optimization and management.

pub use lang::*;
pub use vm::*;

mod lang;
mod vm;

/// An opaque handle to resource in the scripting subsystem.
pub type ScriptHandle = crate::collections::ArenaIndex;

/// The script backend implementation for the underlying scripting subsystem.
///
/// This is a high-level abstraction that makes use of 'opaque' handles to hide away implementation
/// details. The server is intended to be a low-level implementation abstraction.
pub struct ScriptBackend {}

impl ScriptBackend {
  pub fn new() -> Self {
    Self {}
  }
}
