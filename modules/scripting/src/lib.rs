//! Scripting engine for Surreal.

use common::{AsVirtualPath, StringName};

#[derive(thiserror::Error, Debug)]
pub enum ScriptError {
  #[error("Failed to compile script: {0}")]
  FailedToCompile(String),
}

/// Represents a scripting language for Surreal.
pub trait ScriptLanguage {
  type Script: Script;

  /// Returns the name of the scripting language.
  fn name(&self) -> StringName;

  /// Returns the file extension for the scripting language.
  /// For example, the file extension for Lua is "lua".
  fn file_extension(&self) -> StringName;

  /// Compiles the given raw code.
  fn compile_code(&self, code: &str) -> Result<Self::Script, ScriptError>;

  /// Compiles the given file.
  fn compile_file(&self, path: impl AsVirtualPath) -> Result<Self::Script, ScriptError>;
  fn compile_stream(&self, stream: &mut dyn common::InputStream) -> Result<Self::Script, ScriptError>;
}

/// Represents a script in a scripting language.
pub trait Script {}
