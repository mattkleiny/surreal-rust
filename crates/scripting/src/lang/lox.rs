//! The Lox language

use super::ast::*;

pub enum ParseError {
  UnexpectedToken,
  UnexpectedEndOfFile,
}

pub fn parse(code: &str) -> Result<Block, ParseError> {
  todo!()
}

fn tokenize(code: &str) -> Vec<Token> {
  todo!()
}

enum Token {
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Semicolon,
  Identifier(String),
  Literal(Literal),
  Invalid(String),
}
