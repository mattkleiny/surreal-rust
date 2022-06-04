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

// TODO: implement MIR?

/// An opaque handle to resource in the scripting subsystem.
pub type ScriptHandle = crate::collections::ArenaIndex;

/// The scripting server implementation.
pub type ScriptServer = std::rc::Rc<Box<ScriptBackend>>;

/// Represents a graphical resource that possesses a `ScriptHandle`.
pub trait ScriptResource {
  fn handle(&self) -> ScriptHandle;
}

/// The script backend implementation for the underlying scripting subsystem.
///
/// This is a high-level abstraction that makes use of 'opaque' handles to hide away implementation
/// details. The server is intended to be a low-level implementation abstraction.
#[derive(Default)]
pub struct ScriptBackend {}

impl ScriptBackend {
  pub fn new() -> Self {
    Self {}
  }
}
