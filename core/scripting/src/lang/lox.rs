//! The Lox language

use common::ToVariant;

use crate::lang::ast::*;

#[derive(Debug)]
pub enum ParseError {
  UnexpectedToken,
  UnexpectedEndOfFile,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  // Grouping tokens
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,

  // Punctuation
  Comma,
  Dot,
  Semicolon,

  // Basic tokens
  Identifier(String),
  Literal(Literal),
  Operator(Operator),
  Keyword(Keyword),
  Invalid(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
  // Arithmetic
  Plus,
  Minus,
  Star,
  Slash,

  // Comparison
  Equal,
  EqualEqual,
  Bang,
  BangEqual,
  Less,
  LessEqual,
  Greater,
  GreaterEqual,

  // Logical
  And,
  Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
  If,
  Else,
  While,
  For,
  Fun,
  Return,
  True,
  False,
  Nil,
  Print,
  Var,
  Class,
  This,
  Super,
}

/// Parses a list of tokens into an AST [`Expression`].
pub fn parse(code: &str) -> Result<Expression, ParseError> {
  Parser::from_code(code).parse_expression()
}

struct Parser {
  tokens: Vec<Token>,
}

impl Parser {
  fn from_code(code: &str) -> Self {
    Self { tokens: tokenise(code) }
  }

  fn parse_expression(&mut self) -> Result<Expression, ParseError> {
    self.parse_binary()
  }

  fn parse_binary(&mut self) -> Result<Expression, ParseError> {
    self.parse_precedence(0)
  }

  fn parse_precedence(&mut self, min_precedence: u8) -> Result<Expression, ParseError> {
    let mut expr = self.parse_unary()?;

    while let Some(token) = self.peek() {
      let (op, precedence) = match token {
        Token::Operator(op) => match op {
          // Precedence 1: Or
          Operator::Or => (BinaryOp::Or, 1),

          // Precedence 2: And
          Operator::And => (BinaryOp::And, 2),

          // Precedence 3: Equality
          Operator::EqualEqual => (BinaryOp::Equal, 3),
          Operator::BangEqual => (BinaryOp::NotEqual, 3),

          // Precedence 4: Comparison
          Operator::Less => (BinaryOp::LessThan, 4),
          Operator::LessEqual => (BinaryOp::LessThanOrEqual, 4),
          Operator::Greater => (BinaryOp::GreaterThan, 4),
          Operator::GreaterEqual => (BinaryOp::GreaterThanOrEqual, 4),

          // Precedence 5: Terms
          Operator::Plus => (BinaryOp::Add, 5),
          Operator::Minus => (BinaryOp::Subtract, 5),

          // Precedence 6: Factors
          Operator::Star => (BinaryOp::Multiply, 6),
          Operator::Slash => (BinaryOp::Divide, 6),

          _ => break,
        },
        _ => break,
      };

      if precedence < min_precedence {
        break;
      }

      self.advance();
      let right = self.parse_precedence(precedence + 1)?;
      expr = Expression::Binary(Box::new(expr), op, Box::new(right));
    }

    Ok(expr)
  }

  fn parse_unary(&mut self) -> Result<Expression, ParseError> {
    if let Some(token) = self.peek() {
      match token {
        Token::Operator(Operator::Minus) => {
          self.advance();
          let expr = self.parse_unary()?;
          return Ok(Expression::Unary(UnaryOp::Negate, Box::new(expr)));
        }
        Token::Operator(Operator::Bang) => {
          self.advance();
          let expr = self.parse_unary()?;
          return Ok(Expression::Unary(UnaryOp::Negate, Box::new(expr)));
        }
        _ => {}
      }
    }

    self.parse_primary()
  }

  fn parse_primary(&mut self) -> Result<Expression, ParseError> {
    match self.advance() {
      Some(Token::Literal(Literal::Integer(value))) => Ok(Expression::Literal(value.to_variant())),
      Some(Token::Literal(Literal::Float(value))) => Ok(Expression::Literal(value.to_variant())),
      Some(Token::Literal(Literal::String(value))) => Ok(Expression::Literal(value.to_variant())),
      Some(Token::Keyword(Keyword::True)) => Ok(Expression::Literal(true.to_variant())),
      Some(Token::Keyword(Keyword::False)) => Ok(Expression::Literal(false.to_variant())),
      Some(Token::Keyword(Keyword::Nil)) => Ok(Expression::Literal(().to_variant())),
      _ => Err(ParseError::UnexpectedToken),
    }
  }

  fn peek(&self) -> Option<&Token> {
    self.tokens.first()
  }

  fn advance(&mut self) -> Option<Token> {
    self.tokens.drain(..1).next()
  }
}

fn tokenise(code: &str) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut characters = code.char_indices().peekable();

  while let Some((position, character)) = characters.next() {
    let token = match character {
      ' ' | '\t' | '\r' | '\n' => continue,
      '(' => Token::LeftParen,
      ')' => Token::RightParen,
      '{' => Token::LeftBrace,
      '}' => Token::RightBrace,
      ',' => Token::Comma,
      '.' => Token::Dot,
      ';' => Token::Semicolon,

      '+' => Token::Operator(Operator::Plus),
      '-' => Token::Operator(Operator::Minus),
      '*' => Token::Operator(Operator::Star),
      '/' => match characters.next_if_eq(&(position + 1, '/')) {
        Some(_) => {
          // Skip line comments
          while let Some((_, ch)) = characters.next() {
            if ch == '\n' {
              break;
            }
          }
          continue;
        }
        None => Token::Operator(Operator::Slash),
      },

      '=' => match characters.next_if_eq(&(position + 1, '=')) {
        Some(_) => Token::Operator(Operator::EqualEqual),
        None => Token::Operator(Operator::Equal),
      },
      '!' => match characters.next_if_eq(&(position + 1, '=')) {
        Some(_) => Token::Operator(Operator::BangEqual),
        None => Token::Operator(Operator::Bang),
      },
      '<' => match characters.next_if_eq(&(position + 1, '=')) {
        Some(_) => Token::Operator(Operator::LessEqual),
        None => Token::Operator(Operator::Less),
      },
      '>' => match characters.next_if_eq(&(position + 1, '=')) {
        Some(_) => Token::Operator(Operator::GreaterEqual),
        None => Token::Operator(Operator::Greater),
      },

      '"' => {
        let mut value = String::new();
        let mut last_matched = '\0';

        while let Some((_, ch)) = characters.next() {
          last_matched = ch;
          if ch == '"' {
            break;
          }
          value.push(ch);
        }

        match last_matched {
          '"' => Token::Literal(Literal::String(value)),
          _ => Token::Invalid(format!("Unterminated string: {}", value)),
        }
      }

      ch if ch.is_digit(10) => {
        let mut value = String::new();
        let mut is_float = false;
        value.push(ch);

        while let Some((_, ch)) = characters.peek() {
          if !ch.is_digit(10) {
            if *ch == '.' && !is_float {
              value.push(*ch);
              is_float = true;
              characters.next();
              continue;
            }
            break;
          }
          value.push(*ch);
          characters.next();
        }

        match is_float {
          true => value
            .parse()
            .map(|v| Token::Literal(Literal::Float(v)))
            .unwrap_or(Token::Invalid(format!("Invalid float: {}", value))),
          false => value
            .parse()
            .map(|v| Token::Literal(Literal::Integer(v)))
            .unwrap_or(Token::Invalid(format!("Invalid integer: {}", value))),
        }
      }

      ch if ch.is_alphabetic() || ch == '_' => {
        let mut value = String::new();
        value.push(ch);

        while let Some((_, ch)) = characters.peek() {
          if !ch.is_alphanumeric() && *ch != '_' {
            break;
          }
          value.push(*ch);
          characters.next();
        }

        match value.as_str() {
          "if" => Token::Keyword(Keyword::If),
          "else" => Token::Keyword(Keyword::Else),
          "while" => Token::Keyword(Keyword::While),
          "for" => Token::Keyword(Keyword::For),
          "fun" => Token::Keyword(Keyword::Fun),
          "return" => Token::Keyword(Keyword::Return),
          "true" => Token::Keyword(Keyword::True),
          "false" => Token::Keyword(Keyword::False),
          "nil" => Token::Keyword(Keyword::Nil),
          "print" => Token::Keyword(Keyword::Print),
          "var" => Token::Keyword(Keyword::Var),
          "class" => Token::Keyword(Keyword::Class),
          "this" => Token::Keyword(Keyword::This),
          "super" => Token::Keyword(Keyword::Super),
          _ => Token::Identifier(value),
        }
      }

      _ => Token::Invalid(character.to_string()),
    };

    tokens.push(token);
  }

  tokens
}

#[cfg(test)]
mod tests {
  use common::Variant;

  use super::*;

  macro_rules! tokenise_test {
    ($name:ident, $($input:expr => $expected:expr),+ $(,)?) => {
      #[test]
      fn $name() {
        $(
          let tokens = tokenise($input);
          assert_eq!(tokens, $expected, "Input: {}", $input);
        )+
      }
    };
  }

  tokenise_test!(test_tokenise_operators,
    "+" => vec![Token::Operator(Operator::Plus)],
    "-" => vec![Token::Operator(Operator::Minus)],
    "*" => vec![Token::Operator(Operator::Star)],
    "/" => vec![Token::Operator(Operator::Slash)]
  );

  tokenise_test!(test_tokenise_literals,
    "123" => vec![Token::Literal(Literal::Integer(123))],
    "3.14" => vec![Token::Literal(Literal::Float(3.14))],
    "\"hello\"" => vec![Token::Literal(Literal::String("hello".to_string()))]
  );

  tokenise_test!(test_tokenise_keywords,
    "if" => vec![Token::Keyword(Keyword::If)],
    "else" => vec![Token::Keyword(Keyword::Else)],
    "while" => vec![Token::Keyword(Keyword::While)],
    "true" => vec![Token::Keyword(Keyword::True)],
    "false" => vec![Token::Keyword(Keyword::False)],
    "nil" => vec![Token::Keyword(Keyword::Nil)]
  );

  tokenise_test!(test_tokenise_identifiers,
    "foo" => vec![Token::Identifier("foo".to_string())],
    "bar_baz" => vec![Token::Identifier("bar_baz".to_string())],
    "_test" => vec![Token::Identifier("_test".to_string())]
  );

  tokenise_test!(test_tokenise_grouping,
    "()" => vec![Token::LeftParen, Token::RightParen],
    "{}" => vec![Token::LeftBrace, Token::RightBrace]
  );

  macro_rules! parse_test {
    ($name:ident, $($input:expr => $expected:expr),+ $(,)?) => {
      #[test]
      fn $name() {
        $(
          let result = parse($input);

          assert!(result.is_ok(), "Failed to parse: {}", $input);
          assert_eq!(result.unwrap(), $expected, "Input: {}", $input);
        )+
      }
    };
  }

  parse_test!(test_parse_literals,
    "123" => Expression::Literal(Variant::I64(123)),
    "3.14" => Expression::Literal(Variant::F64(3.14)),
    "\"hello\"" => Expression::Literal(Variant::String("hello".to_string()))
  );

  parse_test!(test_parse_binary_expressions,
    "1 + 2" => Expression::Binary(
      Box::new(Expression::Literal(Variant::I64(1))),
      BinaryOp::Add,
      Box::new(Expression::Literal(Variant::I64(2)))
    )
  );

  parse_test!(test_parse_unary_expressions,
    "-5" => Expression::Unary(
      UnaryOp::Negate,
      Box::new(Expression::Literal(Variant::I64(5)))
    )
  );
}
