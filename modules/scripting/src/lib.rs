//! Scripting engine for Surreal.

pub use lua::*;

mod lua;

use common::{StringName, ToVirtualPath, Variant};

// Possible errors when compiling a script.
#[derive(thiserror::Error, Debug)]
pub enum ScriptError {
  #[error("Failed to compile script: {0}")]
  FailedToCompile(String),
}

pub trait ScriptProvider {
  /// Returns the name of the scripting language.
  fn name(&self) -> StringName;

  /// Returns the file extension for the scripting language, sans the dot.
  ///
  /// For example, the file extension for Lua is "lua".
  fn file_extension(&self) -> StringName;
}

/// Represents a scripting language for Surreal.
pub trait ScriptLanguage: ScriptProvider {
  /// The type of script that this language compiles to.
  type Script: Script;

  /// Compiles the given file.
  fn compile_file(&self, path: impl ToVirtualPath) -> common::Result<Self::Script> {
    let path = path.to_virtual_path();
    let mut stream = path.open_input_stream()?;

    self.compile_stream(&mut stream)
  }

  /// Compiles the given stream.
  fn compile_stream(&self, stream: &mut dyn common::InputStream) -> common::Result<Self::Script> {
    let code = stream.to_string()?;

    self.compile_code(&code)
  }

  /// Compiles the given raw code.
  fn compile_code(&self, code: &str) -> common::Result<Self::Script>;
}

/// Represents a script in a scripting language.
pub trait Script: Sized {
  /// Executes the script.
  fn execute(&mut self) -> common::Result<()>;

  /// Calls the given function with the given arguments.
  fn call(&mut self, name: &str, arguments: &[Variant]) -> common::Result<Variant>;
}
