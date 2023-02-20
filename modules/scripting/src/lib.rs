//! Scripting support for Surreal.

#![allow(dead_code)]

use thiserror::Error;

pub mod basic;

surreal::impl_rid!(ScriptId);

/// A language for writing scripts.
///
/// This trait is used to identify a scripting language, and is used to
/// determine which parser and compiler to use for a given script.
pub trait ScriptLanguage {
  type Expression;

  /// The name of the scripting language, for editor tooling/etc.
  fn name() -> &'static str;

  /// The file extensions supported by this scripting language.
  fn extensions(&self) -> &'static [&'static str];

  /// Parses the given source code into an abstract syntax tree.
  fn parse(source: &str) -> Result<Self::Expression, ParserError>;

  /// Compiles the given abstract syntax tree into a byte code representation.
  fn compile(expression: Self::Expression) -> Result<CompiledScript, CompilerError>;
}

/// Possible error codes for script operations.
#[derive(Error, Debug)]
pub enum ScriptError {
  #[error("the given script {0:?} is invalid")]
  InvalidId(ScriptId),
  #[error("the buffer is not large enough to hold the requested data")]
  BufferTooSmall,
  #[error("the given buffer pointer is null")]
  NullPointer,
}

/// Possible error codes from parsing a script.
#[derive(Error, Debug)]
pub enum ParserError {
  #[error("unexpected token: {0}")]
  UnexpectedToken(String),
  #[error("unexpected end of input")]
  UnexpectedEndOfInput,
  #[error("invalid number: {0}")]
  InvalidNumber(String),
}

/// Possible error codes from compiling a script.
#[derive(Error, Debug)]
pub enum CompilerError {
  #[error("unexpected end of input")]
  UnexpectedEndOfInput,
}

/// Possible error codes from working with byte code.
#[derive(Error, Debug)]
pub enum OpCodeError {
  #[error("invalid magic number")]
  InvalidMagicNumber,
  #[error("unexpected end of input")]
  UnexpectedEndOfInput,
}

/// An opcode in a byte code representation.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
  NoOp,
  Push(u8),
  Add,
  Subtract,
  Multiply,
  Divide,
  Negate,
  Return,
}

/// A byte code representation of a compiled script.
///
/// This is the final representation of a script, and is used to execute the
/// script in the virtual machine.
#[derive(Debug, Clone)]
pub struct CompiledScript(Box<[OpCode]>);

impl CompiledScript {
  /// The magic number used to identify the start of a program.
  const MAGIC_NUMBER: [u8; 4] = [0x53, 0x75, 0x72, 0x65];

  /// Returns the number of opcodes in the program.
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Returns the opcodes in the program.
  pub fn as_slice(&self) -> &[OpCode] {
    &self.0
  }
}

impl TryFrom<&[u8]> for CompiledScript {
  type Error = OpCodeError;

  /// Attempts to convert a vector of bytes into a byte code representation.
  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    // make sure the first few bytes are the magic number
    if value.len() < 4 || value[0..4] != Self::MAGIC_NUMBER {
      return Err(OpCodeError::InvalidMagicNumber);
    }

    todo!()
  }
}
