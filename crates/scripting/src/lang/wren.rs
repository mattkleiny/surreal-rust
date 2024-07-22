//! The Wren language

use super::ast::*;

pub mod parser {
  //! Parser for the Wren language
  use super::*;

  #[derive(Debug)]
  pub enum ParseError {
    UnexpectedToken,
    UnexpectedEndOfFile,
  }

  pub fn parse(_code: &str) -> Result<Vec<Statement>, ParseError> {
    todo!()
  }

  enum Token {
    Identifier(String),
    Literal(Literal),
    Operator(String),
    Keyword(String),
    Punctuation(String),
  }
}
