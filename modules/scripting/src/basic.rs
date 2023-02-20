//! BASIC language support for Surreal
//!
//! This module provides support for the BASIC language. It is a simple language
//! that is easy to learn and use. It is also a good starting point for learning
//! how to write a compiler and to prove out the scripting infrastructure.

use super::*;

/// The BASIC [`ScriptLanguage`].
pub struct BASIC {}

impl ScriptLanguage for BASIC {
  type Expression = Expression;

  fn name() -> &'static str {
    "BASIC"
  }

  fn extensions(&self) -> &'static [&'static str] {
    &["bas"]
  }

  fn parse(source: &str) -> Result<Self::Expression, ParserError> {
    let _tokens = tokenize(source)?;

    todo!()
  }

  fn compile(_expression: Self::Expression) -> Result<CompiledScript, CompilerError> {
    todo!()
  }
}

/// A single token in the BASIC language.
#[derive(Debug, PartialEq)]
enum Token {
  // single characters
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  SemiColon,
  Comma,
  Dot,
  Minus,
  Plus,
  Star,

  // one or two characters
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Less,
  LessEqual,
  Greater,
  GreaterEqual,

  Literal(Literal),
  Keyword(Keyword),
}

/// A keyword in the BASIC language.
#[derive(Debug, PartialEq)]
enum Keyword {
  If,
  Then,
  Else,
  End,
  For,
  To,
  Step,
  Next,
  While,
  Repeat,
  Until,
  Function,
  Sub,
  Return,
  Exit,
  Dim,
  As,
  Print,
  Input,
  Let,
  True,
  False,
  And,
  Or,
  Not,
}

impl std::str::FromStr for Keyword {
  type Err = ParserError;

  /// Parses a keyword from the given string.
  fn from_str(string: &str) -> Result<Self, Self::Err> {
    todo!()
  }
}

/// An expression in the BASIC language.
#[derive(Debug, PartialEq)]
pub enum Expression {
  Literal(Literal),
  Variable(String),
  BinaryOperation {
    operation: BinaryOperation,
    left: Box<Expression>,
    right: Box<Expression>,
  },
  UnaryOperation {
    operation: UnaryOperation,
    operand: Box<Expression>,
  },
}

/// A literal value in the BASIC language.
#[derive(Debug, PartialEq)]
pub enum Literal {
  Integer(i64),
  Float(f64),
  String(String),
}

/// A binary operation in the BASIC language.
#[derive(Debug, PartialEq)]
pub enum BinaryOperation {
  Add,
  Subtract,
  Multiply,
  Divide,
}

/// A unary operation in the BASIC language.
#[derive(Debug, PartialEq)]
pub enum UnaryOperation {
  Negate,
}

/// Tokenizes the given source code into a list of [`Token`]s.`
fn tokenize(source: &str) -> Result<Vec<Token>, ParserError> {
  let mut tokens = Vec::new();
  let mut characters = source.chars().peekable();

  macro_rules! emit {
    // emits a new token into the output
    ($token:expr) => {
      tokens.push($token)
    };
    // emits one of two new token into the output depending on the peeked character
    ($token1:expr, $token2:expr, $peek:expr) => {
      if let Some($peek) = characters.peek() {
        characters.next(); // consume peeked character
        emit!($token2);
      } else {
        emit!($token1);
      }
    };
  }

  while let Some(character) = characters.next() {
    match character {
      // single characters
      '(' => emit!(Token::LeftParen),
      ')' => emit!(Token::RightParen),
      '{' => emit!(Token::LeftBrace),
      '}' => emit!(Token::RightBrace),
      ';' => emit!(Token::SemiColon),
      ',' => emit!(Token::Comma),
      '.' => emit!(Token::Dot),
      '-' => emit!(Token::Minus),
      '+' => emit!(Token::Plus),
      '*' => emit!(Token::Star),

      // one or two characters
      '!' => emit!(Token::Bang, Token::BangEqual, '='),
      '=' => emit!(Token::Equal, Token::EqualEqual, '='),
      '<' => emit!(Token::Less, Token::LessEqual, '='),
      '>' => emit!(Token::Greater, Token::GreaterEqual, '='),

      // numbers
      '0'..='9' => {
        let mut is_float = false;
        let mut number = String::new();

        number.push(character);

        while let Some(character) = characters.next() {
          if character.is_ascii_digit() {
            number.push(character);
          } else if character == '.' {
            is_float = true;
            if let Some(next) = characters.peek() {
              if next.is_ascii_digit() {
                number.push(character);
                continue;
              }
            }
            break;
          } else {
            break;
          }
        }

        let number = match is_float {
          true => Literal::Float(number.parse().map_err(|_| ParserError::InvalidNumber(number))?),
          false => Literal::Integer(number.parse().map_err(|_| ParserError::InvalidNumber(number))?),
        };

        emit!(Token::Literal(number));
      }

      // TODO: keywords
      // TODO: strings

      // ignore whitespace
      ' ' | '\t' | '\n' => {}

      _ => return Err(ParserError::UnexpectedToken(character.to_string())),
    }
  }

  Ok(tokens)
}

/// Parses the given list of [`Token`]s into an [`Expression`].
fn parse(tokens: &[Token]) -> Result<Expression, ParserError> {
  let mut tokens = tokens.iter().peekable();

  while let Some(token) = tokens.next() {
    todo!()
  }

  todo!()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic_parser_should_work() {
    let program = "1 + 2";
    let expression = BASIC::parse(program).expect("Failed to parse program");

    assert_eq!(
      expression,
      Expression::BinaryOperation {
        operation: BinaryOperation::Add,
        left: Box::new(Expression::Literal(Literal::Integer(1))),
        right: Box::new(Expression::Literal(Literal::Integer(2))),
      }
    );
  }
}
