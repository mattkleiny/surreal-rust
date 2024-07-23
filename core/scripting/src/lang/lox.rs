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

/// Tokens from the Lox language
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

/// Precedence levels for the Pratt parser
enum Precedence {
  None,
  Assignment,
  Or,
  And,
  Equality,
  Comparison,
  Term,
  Factor,
  Unary,
  Call,
  Primary,
}

/// Pratt parser rules for the Lox language
struct ParseRule {
  prefix: Option<fn()>,
  infix: Option<fn()>,
  precedence: Precedence,
}

macro_rules! rules {
  ($($name:ident => $prefix:expr, $infix:expr, $precedence:expr),*) => {
    &[
      $(ParseRule { prefix: $prefix,infix: $infix, precedence: $precedence }),*
    ]
  };
}

const RULE_TABLE: &[ParseRule] = rules! {
  Rule1 => None, None, Precedence::None,
  Rule2 => None, None, Precedence::Assignment
};
