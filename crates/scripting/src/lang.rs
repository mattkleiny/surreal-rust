//! Scripting language abstractions

use common::{ToVirtualPath, Variant};

#[cfg(feature = "basic")]
pub mod basic;
#[cfg(feature = "lisp")]
pub mod lisp;
#[cfg(feature = "lua")]
pub mod lua;
#[cfg(feature = "wren")]
pub mod wren;

/// Represents a scripting language
pub trait ScriptLanguage {
  /// Loads a script from the given path
  fn load(path: impl ToVirtualPath) -> Result<Script, ScriptError>;
}

/// Represents a parsed script
pub struct Script {
  module: ast::Module,
}

impl Script {
  /// Loads a script from the given path
  pub fn from_path<L: ScriptLanguage>(path: impl ToVirtualPath) -> Result<Self, ScriptError> {
    L::load(path)
  }

  /// Executes the script
  pub fn execute(&self) {
    todo!()
  }

  /// Evaluates the script with the given arguments
  pub fn evaluate(&self, _arguments: &[Variant]) -> Vec<Variant> {
    todo!()
  }

  /// Calls the given top-level function with the given arguments
  pub fn call(&self, _name: &str, _arguments: &[Variant]) -> Vec<Variant> {
    todo!()
  }
}

/// Represents an error that occurred while parsing a script
#[derive(Debug, Eq, PartialEq)]
pub enum ScriptError {
  NotFound,
  ParseError,
}

mod ast {
  //! The internal AST representation of a script

  pub struct Module {
    pub name: String,
  }

  pub enum Statement {}
  pub enum Expression {}
  pub enum Literal {}
}
