//! Scripting support for Surreal.

#![allow(dead_code)]

use surreal::{io::VirtualPath, utilities::Variant};
use thiserror::Error;

pub mod basic;
#[cfg(feature = "gdscript")]
pub mod gdscript;
#[cfg(feature = "lua")]
pub mod lua;

surreal::impl_rid!(ScriptId);

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

/// A server abstraction for managing application scripts.
pub trait ScriptServer {
  /// The name of the scripting backend, for editor tooling/etc.
  fn name(&self) -> &str;

  /// The file extensions supported by this script server.
  fn extensions(&self) -> &[&str];

  // script management
  fn script_create(&self) -> Result<ScriptId, ScriptError>;
  fn script_load(&self, script_id: ScriptId, path: &VirtualPath) -> Result<(), ScriptError>;
  fn script_execute(&self, script_id: ScriptId, method: &str, arguments: &[Variant]) -> Result<Variant, ScriptError>;
  fn script_delete(&self, script_id: ScriptId) -> Result<(), ScriptError>;
}

/// Possible error codes from parsing a script.
#[derive(Error, Debug)]
pub enum ParserError {
  #[error("unexpected token: {0}")]
  UnexpectedToken(String),
  #[error("unexpected end of input")]
  UnexpectedEndOfInput,
}

/// Allows for tokenizing a string into a stream of tokens.
pub trait Lexer {
  type Token;

  /// Extracts the next token from the stream.
  fn tokenize(&mut self) -> Result<Self::Token, ParserError>;
}

/// Allows for parsing a stream of tokens into an abstract syntax tree.
pub trait Parser {
  type Token;
  type Expression;

  /// Parses the next expression from the lexer.
  fn parse(&mut self, lexer: &mut impl Lexer<Token = Self::Token>) -> Result<Self::Expression, ParserError>;
}

/// Allows for compiling an abstract syntax tree into a byte code representation.
pub trait Compiler {
  type Expression;

  /// Compiles the given expression into a byte code representation.
  fn compile(&mut self, expression: Self::Expression) -> Result<Program, ParserError>;
}

/// A cursor over a stream of values.
pub trait Cursor {
  type Value;
  type Error;

  fn next(&mut self) -> Result<Self::Value, Self::Error>;
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
#[derive(Debug, Clone)]
pub struct Program(Box<[OpCode]>);

impl Program {
  /// The magic number used to identify the start of a program.
  const MAGIC_NUMBER: [u8; 4] = [0x53, 0x75, 0x72, 0x65];

  /// Creates a program from the given cursor.
  pub fn from_cursor(_cursor: impl Cursor<Value = OpCode>) -> Result<Self, OpCodeError> {
    todo!()
  }

  /// Returns the number of opcodes in the program.
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Returns true if the program is empty.
  pub fn as_slice(&self) -> &[OpCode] {
    &self.0
  }
}

impl TryFrom<Vec<u8>> for Program {
  type Error = OpCodeError;

  /// Attempts to convert a vector of bytes into a byte code representation.
  fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
    // make sure the first few bytes are the magic number
    if value.len() < 4 || value[0..4] != Self::MAGIC_NUMBER {
      return Err(OpCodeError::InvalidMagicNumber);
    }

    todo!()
  }
}

pub struct VirtualMachine {}

impl VirtualMachine {
  pub fn new() -> Self {
    todo!()
  }

  pub fn execute(&mut self, program: Program) -> Result<Variant, OpCodeError> {
    todo!()
  }
}
