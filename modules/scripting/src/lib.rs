//! Scripting engine for Surreal.
//!
//! The scripting engine is responsible for parsing and executing scripts.
//! It is designed to be as flexible as possible, allowing for multiple
//! scripting languages to be used.
//!
//! The scripting engine is split into two parts: the language and the runtime.
//!
//! The language is responsible for parsing the script into an abstract syntax
//! tree (AST). The AST is shared between all scripting languages, and is
//! general enough to support multiple paradigms.
//!
//! Integration of languages occurs at the AST level as much as possible, though
//! there are some cases where the runtime must be aware of the language.
//!
//! The runtime is responsible for executing the script. It is designed to be
//! as flexible as possible, allowing for multiple execution models to be used,
//! such as a virtual machine or an interpreter.

// TODO: better error handling across the project

/// Represents an error that occurred while parsing a script.
#[derive(Debug)]
pub enum ScriptError {
  ParseError,
  BytecodeError,
}

#[allow(dead_code)]
pub mod lang {
  //! Language support for the scripting system.
  //!
  //! This module is responsible for parsing scripts into an abstract syntax
  //! tree (AST). This AST is shared between all scripting languages.
  pub use basic::*;
  pub use wren::*;

  use crate::ScriptError;

  pub mod ast;

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

    /// Parses the file at the given path.
    fn parse_path(&self, path: impl common::ToVirtualPath) -> Result<ast::Module, ScriptError> {
      let path = path.to_virtual_path();
      let mut stream = path.open_input_stream().map_err(|_| ScriptError::ParseError)?;

      self.parse_stream(&mut stream)
    }

    /// Parses the given stream.
    fn parse_stream(&self, stream: &mut dyn common::InputStream) -> Result<ast::Module, ScriptError> {
      let code = stream.to_string().map_err(|_| ScriptError::ParseError)?;

      self.parse_code(&code)
    }

    /// Parses the given raw code.
    fn parse_code(&self, code: &str) -> Result<ast::Module, ScriptError>;
  }
}

#[allow(dead_code)]
pub mod runtime {
  //! Runtime support for the scripting system.
  //!
  //! This module is responsible for executing scripts. It is designed to be
  //! as flexible as possible, allowing for multiple execution models to be
  //! used, such as a virtual machine or an interpreter.
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
