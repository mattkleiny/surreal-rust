//! The Wren language

use crate::lang::ast::*;

#[derive(Debug)]
pub enum ParseError {
  UnexpectedToken,
  UnexpectedEndOfFile,
}

enum Token {
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Semicolon,
  Identifier(String),
  Literal(Literal),
  Operator(Operator),
  Keyword(Keyword),
  Invalid(String),
}

enum Operator {
  Add,
  Subtract,
  Multiply,
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
}

enum Keyword {
  If,
  Else,
  While,
  For,
  Return,
  True,
  False,
}

/// Parses a string of Wren code into an AST [`Block`].
pub fn parse(code: &str) -> Result<Block, ParseError> {
  let mut parser = Parser::from_code(code);
  let expression = parser.parse_expression()?;

  Ok(Block(vec![Statement::Expression(expression)]))
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
    todo!()
  }
}

/// Tokenises a string of Wren code into a list of [`Token`]s.
fn tokenise(code: &str) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut characters = code.char_indices().peekable();

  while let Some((position, character)) = characters.next() {
    let token = match character {
      ' ' => continue, // Ignore whitespace
      ';' => Token::Semicolon,
      '(' => Token::LeftParen,
      ')' => Token::RightParen,
      '{' => Token::LeftBrace,
      '}' => Token::RightBrace,
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
        None => Token::Operator(Operator::Divide),
      },
      '<' => match characters.next_if_eq(&(position + 1, '=')) {
        Some(_) => Token::Operator(Operator::LessThanOrEqual),
        None => Token::Operator(Operator::LessThan),
      },
      '>' => match characters.next_if_eq(&(position + 1, '=')) {
        Some(_) => Token::Operator(Operator::GreaterThanOrEqual),
        None => Token::Operator(Operator::GreaterThan),
      },
      '+' => Token::Operator(Operator::Add),
      '-' => Token::Operator(Operator::Subtract),
      '*' => Token::Operator(Operator::Multiply),
      '%' => Token::Operator(Operator::Modulo),
      '=' => match characters.next_if_eq(&(position + 1, '=')) {
        Some(_) => Token::Operator(Operator::EqualEqual),
        None => Token::Operator(Operator::Equal),
      },
      '!' => match characters.next_if_eq(&(position + 1, '=')) {
        Some(_) => Token::Operator(Operator::NotEqual),
        None => Token::Operator(Operator::Not),
      },
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
            if *character == '.' {
              value.push(*character);
              is_floating_point = true;
            } else {
              break;
            }
          }

          value.push(*character);
          characters.next();
        }

        match is_floating_point {
          true => Token::Literal(Literal::Float(value.parse().unwrap())),
          false => Token::Literal(Literal::Integer(value.parse().unwrap())),
        }
      }
      _ if character.is_alphabetic() => {
        // parse keywords and identifiers
        let mut value = String::new();

        value.push(character);

        while let Some((_, character)) = characters.peek() {
          if !character.is_alphanumeric() {
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
  use super::*;

  #[test]
  fn it_should_tokenise_simple_numerical_statements() {
    let code = "1 + 2";
    let tokens = tokenise(code);

    assert_eq!(tokens.len(), 3);
  }

  #[test]
  fn it_should_tokenise_string_literals() {
    let code = r#""Hello, world!""#;
    let tokens = tokenise(code);

    assert_eq!(tokens.len(), 1);
  }

  #[test]
  fn it_should_tokenise_keywords() {
    let code = "if (true) { return 1; } else { return 2; }";
    let tokens = tokenise(code);

    assert_eq!(tokens.len(), 15);
  }

  #[test]
  fn it_should_tokenise_more_complex_expressions() {
    let code = "1 + 2 * 3 // This is a comment";
    let tokens = tokenise(code);

    assert_eq!(tokens.len(), 5);
  }
}
