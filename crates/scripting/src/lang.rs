//! Scripting language abstractions

use common::ToVirtualPath;
#[cfg(feature = "lua")]
pub use lua::*;
#[cfg(feature = "wren")]
pub use wren::*;

#[cfg(feature = "lua")]
mod lua;
#[cfg(feature = "wren")]
mod wren;

/// Represents a scripting language
pub trait ScriptLanguage {
  /// Loads a script from the given path
  fn load(path: impl ToVirtualPath) -> Result<Script, ScriptError>;
}

/// Represents a parsed script
pub struct Script {
  module: ast::Module,
}

/// Represents an error that occurred while parsing a script
pub enum ScriptError {
  NotFound,
  ParseError,
}

mod ast {
  //! The internal AST representation of a script

  pub struct Module {}
  pub enum Statement {}
  pub enum Expression {}
  pub enum Literal {}
}
