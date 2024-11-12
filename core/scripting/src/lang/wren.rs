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

  tokenise_test!(simple_numerical_statements,
    "1 + 2" => 3,
    "-123.456" => 2
  );

  tokenise_test!(string_literals,
    r#""Hello, world!""# => 1
  );

  tokenise_test!(keywords,
    "if (true) { return 1; } else { return 2; }" => 15,
    "class Example { construct new() { this.value = null; } }" => 16
  );

  tokenise_test!(complex_expressions,
    "1 + 2 * 3 // This is a comment" => 5,
    "(a + b) * (c - d) / e % f" => 15
  );

  tokenise_test!(edge_cases,
    "" => 0,
    "   \t\n  " => 0,
    "@#$%" => 4
  );

  tokenise_test!(complex_identifiers,
    "_valid_123 notKeyword_if if_not_keyword" => 3,
    "very_long_identifier_with_numbers_123_456_789" => 1
  );

  tokenise_test!(operators,
    "a << 2 >> 3 & 4 | 5 ^ 6" => 11,
    "a <= b >= c == d != e" => 9,
    "1..5 1...6" => 6
  );
}
