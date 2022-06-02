//! Shared abstractions for scripting languages.
//!
//! Different language front-ends can implement these abstractions
//! to allow unification in the scripting system.

pub use lisp::*;
pub use lua::*;

mod lisp;
mod lua;

use super::*;

/// Represents a potential scripting language in the scripting system.
///
/// A language provides it's own front-end for the common scripting IR.
/// Internally the scripting system will lower the IR to a shared stack-based runtime.
pub trait ScriptLanguage {
  /// Compiles the given raw program code into a bytecode program.
  fn compile(&self, program: &str) -> crate::Result<BytecodeProgram>;
}
