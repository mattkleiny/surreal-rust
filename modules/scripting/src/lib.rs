//! Scripting engine for Surreal.

#[allow(dead_code)]
pub mod lang {
  //! Language support for the scripting system.
  pub use ast::*;
  pub use basic::*;
  pub use wren::*;

  mod ast;
  mod basic;
  mod wren;

  /// Represents a scripting language for Surreal.
  pub trait ScriptLanguage {
    /// Returns the name of the scripting language.
    fn name(&self) -> &'static str;

    /// Returns the file extension for the scripting language, sans the dot.
    ///
    /// For example, the file extension for Lua is "lua".
    fn file_extensions(&self) -> &[&'static str];

    /// Compiles the file at the given path.
    fn compile_path(&self, path: impl common::ToVirtualPath) -> common::Result<()> {
      let path = path.to_virtual_path();
      let mut stream = path.open_input_stream()?;

      self.compile_stream(&mut stream)
    }

    /// Parses the given stream.
    fn compile_stream(&self, stream: &mut dyn common::InputStream) -> common::Result<()> {
      let code = stream.to_string()?;

      self.compile_code(&code)
    }

    /// Parses the given raw code.
    fn compile_code(&self, code: &str) -> common::Result<()>;
  }
}

#[allow(dead_code)]
pub mod runtime {
  //! Runtime support for the scripting system.
  pub use interpret::*;
  pub use vm::*;

  mod interpret;
  mod vm;

  /// A runtime capable of executing a script.
  ///
  /// This is either a virtual machine or an interpreter, depending on the
  /// desired execution model.
  pub trait ScriptRuntime {}
}
