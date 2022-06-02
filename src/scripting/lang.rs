//! Shared abstractions for scripting languages.
//!
//! Different language front-ends can implement these abstractions
//! to allow unification in the scripting system.

pub use lisp::*;
pub use lox::*;

mod lisp;
mod lox;

use super::*;

/// Represents a potential scripting language in the scripting system.
pub trait ScriptLanguage {
  /// Compiles the given raw code into a `BytecodeChunk`.
  fn compile(&self, code: &str) -> crate::Result<BytecodeChunk>;
}
