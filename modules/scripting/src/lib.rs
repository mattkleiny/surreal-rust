//! Scripting engine for Surreal.
//!
//! The scripting engine is split into two parts: `lang`uage and the `runtime`.
//!
//! * The `lang` module is responsible for parsing a script into an abstract
//!   syntax tree (AST). The AST is shared between all scripting languages, and
//!   is general enough to support multiple paradigms; it is not tied to any
//!   specific language.
//!
//! * The `runtime` module is responsible for executing the scripts. It is a
//!   lightweight, stack-based, virtual machine with a simple instruction set.
//!   The runtime is designed to be easy to embed in other applications, and is
//!   not tied to any specific language.

pub use lang::*;
pub use runtime::*;

mod lang {
  //! Language frontend for the scripting engine.
  pub use basic::*;

  pub mod ast;
  mod basic;

  /// Represents an error that occurred while parsing a script.
  #[derive(Debug)]
  pub enum ParserError {
    FailedToReadStream,
    InvalidSyntax(String),
  }

  /// Represents a scripting language for Surreal.
  pub trait ScriptLanguage {
    /// Returns the name of the scripting language.
    fn name() -> &'static str;

    /// Returns the file extension for the scripting language, sans the dot.
    fn file_extensions() -> &'static [&'static str];

    /// Parses the file at the given path.
    fn parse_path(path: impl common::ToVirtualPath) -> Result<ast::Module, ParserError> {
      let path = path.to_virtual_path();

      let mut stream = path.open_input_stream().map_err(|_| ParserError::FailedToReadStream)?;

      Self::parse_stream(&mut stream)
    }

    /// Parses the given stream.
    fn parse_stream(stream: &mut dyn common::InputStream) -> Result<ast::Module, ParserError> {
      let code = stream.to_string().map_err(|_| ParserError::FailedToReadStream)?;

      Self::parse_code(&code)
    }

    /// Parses the given raw code.
    fn parse_code(code: &str) -> Result<ast::Module, ParserError>;
  }
}

mod runtime {
  //! Runtime for the scripting engine.
  pub use vm::*;

  mod vm;
}
