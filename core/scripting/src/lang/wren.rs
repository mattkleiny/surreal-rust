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

enum Keyword {
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
        None => Token::Operator(Operator::Divide),
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
  use super::*;

  #[test]
  fn it_should_tokenise_simple_numerical_statements() {
    let code = "1 + 2";
    let tokens = tokenise(code);
    assert_eq!(tokens.len(), 3);

    let code = "-123.456";
    let tokens = tokenise(code);
    assert_eq!(tokens.len(), 2);
  }

  #[test]
  fn it_should_tokenise_string_literals() {
    let code = r#""Hello, world!""#;
    let tokens = tokenise(code);
    assert_eq!(tokens.len(), 1);

    let code = r#""Special chars: \n \t \" \\ ""#;
    let tokens = tokenise(code);
    assert_eq!(tokens.len(), 1);

    let code = r#""""Multi
    line
    string""""#;
    let tokens = tokenise(code);
    assert_eq!(tokens.len(), 1);
  }

  #[test]
  fn it_should_tokenise_keywords() {
    let code = "if (true) { return 1; } else { return 2; }";
    let tokens = tokenise(code);
    assert_eq!(tokens.len(), 15);

    let code = "class Example { construct new() { this.value = null; } }";
    let tokens = tokenise(code);
    assert_eq!(tokens.len(), 14);
  }

  #[test]
  fn it_should_tokenise_more_complex_expressions() {
    let code = "1 + 2 * 3 // This is a comment";
    let tokens = tokenise(code);
    assert_eq!(tokens.len(), 5);

    let code = "(a + b) * (c - d) / e % f";
    let tokens = tokenise(code);
    assert_eq!(tokens.len(), 13);
  }

  #[test]
  fn it_should_handle_edge_cases() {
    // Empty input
    let tokens = tokenise("");
    assert_eq!(tokens.len(), 0);

    // Only whitespace
    let tokens = tokenise("   \t\n  ");
    assert_eq!(tokens.len(), 0);

    // Invalid characters
    let code = "@#$%";
    let tokens = tokenise(code);
    assert_eq!(tokens.len(), 4);
  }

  #[test]
  fn it_should_tokenise_complex_identifiers() {
    let code = "_valid_123 notKeyword_if if_not_keyword";
    let tokens = tokenise(code);
    assert_eq!(tokens.len(), 3);

    let code = "very_long_identifier_with_numbers_123_456_789";
    let tokens = tokenise(code);
    assert_eq!(tokens.len(), 1);
  }

  #[test]
  fn it_should_handle_operators() {
    let code = "a << 2 >> 3 & 4 | 5 ^ 6";
    let tokens = tokenise(code);
    assert_eq!(tokens.len(), 11);

    let code = "a <= b >= c == d != e";
    let tokens = tokenise(code);
    assert_eq!(tokens.len(), 9);

    let code = "1..5 1...6"; // Range and RangeInclusive operators
    let tokens = tokenise(code);
    assert_eq!(tokens.len(), 4);
  }
}
