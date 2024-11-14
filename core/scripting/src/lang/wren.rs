//! The Wren language

use common::ToVariant;

use crate::lang::ast::*;

#[derive(Debug)]
pub enum ParseError {
  UnexpectedToken,
  UnexpectedEndOfFile,
}

#[derive(Debug, Clone)]
pub enum Token {
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  LeftBracket,
  RightBracket,
  Comma,
  Dot,
  Semicolon,
  Identifier(String),
  Literal(Literal),
  Operator(Operator),
  Keyword(Keyword),
  Invalid(String),
}

#[derive(Debug, Clone)]
pub enum Operator {
  Plus,
  Minus,
  Star,
  Divide,
  Modulo,
  Equal,
  EqualEqual,
  Not,
  NotEqual,
  LessThan,
  LessThanOrEqual,
  GreaterThan,
  GreaterThanOrEqual,
  And,
  Or,
  BitAnd,
  BitOr,
  BitXor,
  LeftShift,
  RightShift,
  Range,
  RangeInclusive,
}

#[derive(Debug, Clone)]
pub enum Keyword {
  If,
  Else,
  While,
  For,
  Return,
  True,
  False,
  Class,
  Is,
  Null,
  Import,
  As,
  In,
  Break,
  Continue,
  Static,
  Var,
  Foreign,
  Construct,
  This,
  Super,
}

/// A parser for Wren code.
struct Parser {
  tokens: Vec<Token>,
}

impl Parser {
  /// Creates a new parser from a list of tokens.
  fn from_code(code: &str) -> Self {
    Self { tokens: tokenise(code) }
  }

  /// Parses an expression from the parser.
  fn parse_expression(&mut self) -> Result<Expression, ParseError> {
    // Start with lowest precedence operators
    self.parse_binary()
  }

  /// Parses a binary expression.
  fn parse_binary(&mut self) -> Result<Expression, ParseError> {
    self.parse_precedence(0)
  }

  /// Parses a binary expression with the given minimum precedence level.
  fn parse_precedence(&mut self, min_precedence: u8) -> Result<Expression, ParseError> {
    let mut expr = self.parse_unary()?;

    while let Some(token) = self.peek() {
      let (op, precedence) = match token {
        Token::Operator(op) => match op {
          // Precedence 1: Comparison operators
          Operator::LessThan => (BinaryOp::LessThan, 1),
          Operator::LessThanOrEqual => (BinaryOp::LessThanOrEqual, 1),
          Operator::GreaterThan => (BinaryOp::GreaterThan, 1),
          Operator::GreaterThanOrEqual => (BinaryOp::GreaterThanOrEqual, 1),

          // Precedence 2: Addition/subtraction
          Operator::Plus => (BinaryOp::Add, 2),
          Operator::Minus => (BinaryOp::Subtract, 2),

          // Precedence 3: Multiplication/division
          Operator::Star => (BinaryOp::Multiply, 3),
          Operator::Divide => (BinaryOp::Divide, 3),

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

  /// Parses a unary expression.
  fn parse_unary(&mut self) -> Result<Expression, ParseError> {
    if let Some(token) = self.peek() {
      match token {
        Token::Operator(Operator::Minus) => {
          self.advance();
          let expr = self.parse_unary()?;
          return Ok(Expression::Unary(UnaryOp::Negate, Box::new(expr)));
        }
        _ => {}
      }
    }

    self.parse_primary()
  }

  /// Parses a primary expression (literals).
  fn parse_primary(&mut self) -> Result<Expression, ParseError> {
    match self.advance() {
      Some(Token::Literal(Literal::Integer(value))) => Ok(Expression::Literal(value.to_variant())),
      Some(Token::Literal(Literal::Float(value))) => Ok(Expression::Literal(value.to_variant())),
      Some(Token::Literal(Literal::String(value))) => Ok(Expression::Literal(value.to_variant())),
      _ => Err(ParseError::UnexpectedToken),
    }
  }

  /// Returns the next token without consuming it.
  fn peek(&self) -> Option<&Token> {
    self.tokens.first()
  }

  /// Consumes and returns the next token.
  fn advance(&mut self) -> Option<Token> {
    self.tokens.drain(..1).next()
  }
}

/// Parses a string of Wren code into an AST [`Expression`].
pub fn parse(code: &str) -> Result<Expression, ParseError> {
  Parser::from_code(code).parse_expression()
}

/// Tokenises a string of Wren code into a list of [`Token`]s.
fn tokenise(code: &str) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut characters = code.char_indices().peekable();

  while let Some((position, character)) = characters.next() {
    let token = match character {
      ' ' | '\t' | '\r' | '\n' => continue, // Ignore whitespace
      ';' => Token::Semicolon,
      '(' => Token::LeftParen,
      ')' => Token::RightParen,
      '{' => Token::LeftBrace,
      '}' => Token::RightBrace,
      '[' => Token::LeftBracket,
      ']' => Token::RightBracket,
      ',' => Token::Comma,
      '.' => match characters.next_if_eq(&(position + 1, '.')) {
        Some(_) => match characters.next_if_eq(&(position + 2, '.')) {
          Some(_) => Token::Operator(Operator::RangeInclusive),
          None => Token::Operator(Operator::Range),
        },
        None => Token::Dot,
      },
      '/' => match characters.next_if_eq(&(position + 1, '/')) {
        Some(_) => {
          // ignore single line comments
          for (_, character) in characters.by_ref() {
            if character == '\n' {
              break;
            }
          }
          continue;
        }
        None => match characters.next_if_eq(&(position + 1, '*')) {
          Some(_) => {
            // ignore block comments
            let mut depth = 1;
            while depth > 0 {
              match characters.next() {
                Some((_, '*')) => {
                  if let Some((_, '/')) = characters.next() {
                    depth -= 1;
                  }
                }
                Some((_, '/')) => {
                  if let Some((_, '*')) = characters.next() {
                    depth += 1;
                  }
                }
                None => break, // Unterminated block comment
                _ => {}
              }
            }
            continue;
          }
          None => Token::Operator(Operator::Divide),
        },
      },
      '<' => match characters.next_if_eq(&(position + 1, '=')) {
        Some(_) => Token::Operator(Operator::LessThanOrEqual),
        None => match characters.next_if_eq(&(position + 1, '<')) {
          Some(_) => Token::Operator(Operator::LeftShift),
          None => Token::Operator(Operator::LessThan),
        },
      },
      '>' => match characters.next_if_eq(&(position + 1, '=')) {
        Some(_) => Token::Operator(Operator::GreaterThanOrEqual),
        None => match characters.next_if_eq(&(position + 1, '>')) {
          Some(_) => Token::Operator(Operator::RightShift),
          None => Token::Operator(Operator::GreaterThan),
        },
      },
      '+' => Token::Operator(Operator::Plus),
      '-' => Token::Operator(Operator::Minus),
      '*' => Token::Operator(Operator::Star),
      '%' => Token::Operator(Operator::Modulo),
      '=' => match characters.next_if_eq(&(position + 1, '=')) {
        Some(_) => Token::Operator(Operator::EqualEqual),
        None => Token::Operator(Operator::Equal),
      },
      '!' => match characters.next_if_eq(&(position + 1, '=')) {
        Some(_) => Token::Operator(Operator::NotEqual),
        None => Token::Operator(Operator::Not),
      },
      '&' => match characters.next_if_eq(&(position + 1, '&')) {
        Some(_) => Token::Operator(Operator::And),
        None => Token::Operator(Operator::BitAnd),
      },
      '|' => match characters.next_if_eq(&(position + 1, '|')) {
        Some(_) => Token::Operator(Operator::Or),
        None => Token::Operator(Operator::BitOr),
      },
      '^' => Token::Operator(Operator::BitXor),
      // parse string literals
      '"' => {
        let mut last_matched = '\0';
        let mut value = String::new();

        for (_, character) in characters.by_ref() {
          last_matched = character;

          if character == '"' {
            break;
          }

          value.push(character);
        }

        match last_matched {
          '"' => Token::Literal(Literal::String(value)),
          _ => Token::Invalid(format!("Unterminated string: {}", value)),
        }
      }
      // parse number literals
      _ if character.is_numeric() => {
        let mut value = String::new();
        let mut is_floating_point = false;

        value.push(character);

        while let Some((_, character)) = characters.peek() {
          if !character.is_numeric() {
            if *character == '.' && !is_floating_point {
              value.push(*character);
              is_floating_point = true;
              characters.next();
              continue;
            }
            break;
          }

          value.push(*character);
          characters.next();
        }

        match is_floating_point {
          true => value
            .parse()
            .map(|value| Token::Literal(Literal::Float(value)))
            .unwrap_or(Token::Invalid(format!("Invalid number: {}", value))),
          false => value
            .parse()
            .map(|value| Token::Literal(Literal::Integer(value)))
            .unwrap_or(Token::Invalid(format!("Invalid integer: {}", value))),
        }
      }
      _ if character.is_alphabetic() || character == '_' => {
        // parse keywords and identifiers
        let mut value = String::new();

        value.push(character);

        while let Some((_, character)) = characters.peek() {
          if !character.is_alphanumeric() && *character != '_' {
            break;
          }

          value.push(*character);
          characters.next();
        }

        match value.as_str() {
          "if" => Token::Keyword(Keyword::If),
          "else" => Token::Keyword(Keyword::Else),
          "while" => Token::Keyword(Keyword::While),
          "for" => Token::Keyword(Keyword::For),
          "return" => Token::Keyword(Keyword::Return),
          "true" => Token::Keyword(Keyword::True),
          "false" => Token::Keyword(Keyword::False),
          "class" => Token::Keyword(Keyword::Class),
          "is" => Token::Keyword(Keyword::Is),
          "null" => Token::Keyword(Keyword::Null),
          "import" => Token::Keyword(Keyword::Import),
          "as" => Token::Keyword(Keyword::As),
          "in" => Token::Keyword(Keyword::In),
          "break" => Token::Keyword(Keyword::Break),
          "continue" => Token::Keyword(Keyword::Continue),
          "static" => Token::Keyword(Keyword::Static),
          "var" => Token::Keyword(Keyword::Var),
          "foreign" => Token::Keyword(Keyword::Foreign),
          "construct" => Token::Keyword(Keyword::Construct),
          "this" => Token::Keyword(Keyword::This),
          "super" => Token::Keyword(Keyword::Super),
          _ => Token::Identifier(value),
        }
      }
      _ => Token::Invalid(format!("{}", character)),
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
    ($name:ident, $($input:expr => $count:expr),+ $(,)?) => {
      #[test]
      fn $name() {
        $(
          let tokens = tokenise($input);
          assert_eq!(tokens.len(), $count, "Input: {}", $input);
        )+
      }
    };
  }

  tokenise_test!(it_should_tokenise_simple_numerical_statements,
    "1 + 2" => 3,
    "-123.456" => 2
  );

  tokenise_test!(it_should_tokenise_string_literals,
    r#""Hello, world!""# => 1
  );

  tokenise_test!(it_should_tokenise_keywords,
    "if (true) { return 1; } else { return 2; }" => 15,
    "class Example { construct new() { this.value = null; } }" => 16
  );

  tokenise_test!(it_should_tokenise_complex_expressions,
    "1 + 2 * 3 // This is a comment" => 5,
    "(a + b) * (c - d) / e % f" => 15
  );

  tokenise_test!(it_should_tokenise_edge_cases,
    "" => 0,
    "   \t\n  " => 0,
    "@#$%" => 4
  );

  tokenise_test!(it_should_tokenise_complex_identifiers,
    "_valid_123 notKeyword_if if_not_keyword" => 3,
    "very_long_identifier_with_numbers_123_456_789" => 1
  );

  tokenise_test!(it_should_tokenise_operators,
    "a << 2 >> 3 & 4 | 5 ^ 6" => 11,
    "a <= b >= c == d != e" => 9,
    "1..5 1...6" => 6
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

  parse_test!(it_should_parse_simple_expressions,
    "123" => Expression::Literal(Variant::I64(123)),
    "3.14" => Expression::Literal(Variant::F64(3.14)),
    r#""test""# => Expression::Literal(Variant::String("test".to_string()))
  );

  parse_test!(it_should_parse_binary_expressions,
    "1 + 2" => Expression::Binary(
      Box::new(Expression::Literal(Variant::I64(1))),
      BinaryOp::Add,
      Box::new(Expression::Literal(Variant::I64(2)))
    ),
    "3 * 4" => Expression::Binary(
      Box::new(Expression::Literal(Variant::I64(3))),
      BinaryOp::Multiply,
      Box::new(Expression::Literal(Variant::I64(4)))
    )
  );

  parse_test!(it_should_parse_unary_expressions,
    "-5" => Expression::Unary(
      UnaryOp::Negate,
      Box::new(Expression::Literal(Variant::I64(5)))
    )
  );

  parse_test!(it_should_parse_complex_expressions,
    "1 + 2 * 3" => Expression::Binary(
      Box::new(Expression::Literal(Variant::I64(1))),
      BinaryOp::Add,
      Box::new(Expression::Binary(
        Box::new(Expression::Literal(Variant::I64(2))),
        BinaryOp::Multiply,
        Box::new(Expression::Literal(Variant::I64(3)))
      ))
    )
  );

  macro_rules! parse_file_test {
    ($name:ident, $path:expr) => {
      #[test]
      fn $name() {
        use std::fs;
        let contents = fs::read_to_string($path).expect("Failed to read file");
        let result = parse(&contents);

        assert!(
          result.is_ok(),
          "Failed to parse file {}: {:?}",
          $path,
          result.err()
        );
      }
    };
  }

  parse_file_test!(it_should_parse_test01, "assets/scripts/wren/test01.wren");
  parse_file_test!(it_should_parse_test02, "assets/scripts/wren/test02.wren");
  parse_file_test!(it_should_parse_test03, "assets/scripts/wren/test03.wren");
}
