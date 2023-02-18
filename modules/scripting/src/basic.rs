//! BASIC language support for Surreal
//!
//! This module provides support for the BASIC language. It is a simple language
//! that is easy to learn and use. It is also a good starting point for learning
//! how to write a compiler and to prove out the scripting infrastructure.

use super::*;

/// The BASIC [`ScriptLanguage`].
pub struct BASIC {}

impl ScriptLanguage for BASIC {
  type Expression = parser::Expression;

  fn name() -> &'static str {
    "BASIC"
  }

  fn extensions(&self) -> &'static [&'static str] {
    &["bas"]
  }

  fn parse(source: &str) -> Result<Self::Expression, ParserError> {
    parser::parse(source)
  }

  fn compile(expression: Self::Expression) -> Result<CompiledScript, CompilerError> {
    compiler::compile(expression)
  }
}

mod parser {
  use super::*;

  pub fn parse(_source: &str) -> Result<Expression, ParserError> {
    struct Lexer {}
    struct Parser {}

    todo!()
  }

  #[derive(Debug, PartialEq, Eq)]
  enum Token {
    Literal(String),
    Variable(String),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    OpenParen,
    CloseParen,
  }

  #[derive(Debug, PartialEq)]
  pub enum Expression {
    Literal(Literal),
    Variable(String),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
  }

  #[derive(Debug, PartialEq)]
  pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
  }

  #[derive(Debug, PartialEq)]
  pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
  }

  #[derive(Debug, PartialEq)]
  pub enum UnaryOperation {
    Negate,
  }
}

mod compiler {
  use super::*;

  pub fn compile(_expression: parser::Expression) -> Result<CompiledScript, CompilerError> {
    struct Compiler {}

    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic_parser_should_work() {
    let program = "1 + 2";
    let expression = BASIC::parse(program).expect("Failed to parse program");

    assert_eq!(expression, parser::Expression::BinaryOperation(parser::BinaryOperation::Add));
  }

  #[test]
  fn basic_compiler_should_work() {
    let program = "1 + 2";
    let expression = BASIC::parse(program).expect("Failed to parse program");
    let _program = BASIC::compile(expression).expect("Failed to compile program");
  }
}
