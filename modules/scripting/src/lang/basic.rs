use super::*;

/// The BASIC [`ScriptLanguage`] implementation.
///
/// This is a very simple language that is intended to be used for testing.
/// It is not intended to be used for any real scripting.
pub struct BASIC;

impl ScriptLanguage for BASIC {
  fn name(&self) -> &'static str {
    "BASIC"
  }

  fn file_extensions(&self) -> &[&'static str] {
    &["bas", "basic"]
  }

  fn parse_code(&self, code: &str) -> Result<ast::Module, ScriptParseError> {
    let _module = parser::parse(code)?;

    todo!()
  }
}

mod parser {
  use std::collections::VecDeque;

  use super::*;

  /// Parses the given BASIC code into a [`Module`].
  pub fn parse(code: &str) -> Result<ast::Module, ScriptParseError> {
    let mut stream = TokenStream::tokenize(code)?;
    let module = stream.parse_script_module()?;

    Ok(module)
  }

  #[derive(Debug, PartialEq)]
  enum Token {
    Number(f64),
    String(String),
    Keyword(Keyword),
    Identifier(String),
    Operator(Operator),
  }

  #[derive(Debug, PartialEq)]
  enum Keyword {
    Rem,
    Let,
    Print,
    Input,
    If,
    Then,
    Else,
    For,
    To,
    Step,
    Next,
    Goto,
    Gosub,
    Return,
    End,
  }

  #[derive(Debug, PartialEq)]
  enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    LessThan,
    GreaterThan,
    And,
    Or,
  }

  impl TokenStream {
    /// Parses a script module from the token stream.
    pub fn parse_script_module(&mut self) -> Result<ast::Module, ScriptParseError> {
      todo!()
    }

    /// Parses a statement from the token stream.
    pub fn parse_statement(&mut self) -> Result<ast::Statement, ScriptParseError> {
      todo!()
    }

    /// Parses an expression from the token stream.
    pub fn parse_expression(&mut self) -> Result<ast::Expression, ScriptParseError> {
      self.parse_primary_expression()
    }

    /// Parses a primary expression from the token stream.
    pub fn parse_primary_expression(&mut self) -> Result<ast::Expression, ScriptParseError> {
      todo!()
    }

    /// Parses a literal value from the token stream.
    pub fn parse_literal(&mut self) -> Result<ast::Literal, ScriptParseError> {
      match self.take() {
        Some(Token::Number(value)) => Ok(ast::Literal::Number(*value)),
        Some(Token::String(value)) => Ok(ast::Literal::String(value.clone())),
        _ => self.unexpected_token(),
      }
    }

    /// Tokenizes the given BASIC code into a [`TokenStream`].
    pub fn tokenize(code: &str) -> Result<Self, ScriptParseError> {
      let mut tokens = VecDeque::<Token>::new();
      let mut chars = code.chars().peekable();

      while let Some(c) = chars.next() {
        match c {
          // skip whitespace
          c if c.is_whitespace() => continue,

          // parse numbers
          c if c.is_numeric() => {
            let mut number = String::new();

            number.push(c);

            while let Some(c) = chars.peek() {
              if c.is_numeric() || *c == '.' {
                number.push(*c);
                chars.next();
              } else {
                break;
              }
            }

            tokens.push_back(Token::Number(number.parse().map_err(|_| {
              ScriptParseError::InvalidSyntax("an invalid number was encountered".to_string())
            })?));
          }

          // parse keywords and identifiers
          c if c.is_ascii_alphabetic() => {
            let mut string = String::new();

            string.push(c);

            while let Some(c) = chars.peek() {
              if c.is_ascii_alphanumeric() || *c == '_' {
                string.push(*c);
                chars.next();
              } else {
                break;
              }
            }

            tokens.push_back(match string.to_uppercase().as_str() {
              "REM" => Token::Keyword(Keyword::Rem),
              "LET" => Token::Keyword(Keyword::Let),
              "PRINT" => Token::Keyword(Keyword::Print),
              "INPUT" => Token::Keyword(Keyword::Input),
              "IF" => Token::Keyword(Keyword::If),
              "THEN" => Token::Keyword(Keyword::Then),
              "ELSE" => Token::Keyword(Keyword::Else),
              "FOR" => Token::Keyword(Keyword::For),
              "TO" => Token::Keyword(Keyword::To),
              "STEP" => Token::Keyword(Keyword::Step),
              "NEXT" => Token::Keyword(Keyword::Next),
              "GOTO" => Token::Keyword(Keyword::Goto),
              "GOSUB" => Token::Keyword(Keyword::Gosub),
              "RETURN" => Token::Keyword(Keyword::Return),
              "END" => Token::Keyword(Keyword::End),
              _ => Token::Identifier(string),
            });
          }

          // parse operators
          '+' => tokens.push_back(Token::Operator(Operator::Plus)),
          '-' => tokens.push_back(Token::Operator(Operator::Minus)),
          '*' => tokens.push_back(Token::Operator(Operator::Multiply)),
          '/' => tokens.push_back(Token::Operator(Operator::Divide)),
          '%' => tokens.push_back(Token::Operator(Operator::Modulo)),
          '=' => tokens.push_back(Token::Operator(Operator::Equal)),
          '<' => tokens.push_back(Token::Operator(Operator::LessThan)),
          '>' => tokens.push_back(Token::Operator(Operator::GreaterThan)),
          '&' => tokens.push_back(Token::Operator(Operator::And)),
          '|' => tokens.push_back(Token::Operator(Operator::Or)),

          // parse other tokens
          _ => {
            return Err(ScriptParseError::InvalidSyntax(
              "an unexpected character was encountered".to_string(),
            ))
          }
        }
      }

      Ok(TokenStream {
        tokens,
        last_token: None,
      })
    }
  }

  ast::impl_token_stream!(Token as TokenStream);

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_tokens() {
      let code = r"
        LET x = 1
        PRINT x
      ";

      let mut stream = TokenStream::tokenize(code).unwrap();

      assert_eq!(stream.take(), Some(&Token::Keyword(Keyword::Let)));
      assert_eq!(stream.take(), Some(&Token::Identifier("x".into())));
      assert_eq!(stream.take(), Some(&Token::Operator(Operator::Equal)));
      assert_eq!(stream.take(), Some(&Token::Number(1.0)));

      assert_eq!(stream.take(), Some(&Token::Keyword(Keyword::Print)));
      assert_eq!(stream.take(), Some(&Token::Identifier("x".into())));
    }

    #[test]
    fn test_parse_literal_number() {
      let code = "3.14159";
      let literal = TokenStream::tokenize(code).unwrap().parse_literal().unwrap();

      assert_eq!(literal, ast::Literal::Number(3.14159));
    }

    #[test]
    fn test_parse_literal_string() {
      let code = r#""Hello, world!""#;
      let literal = TokenStream::tokenize(code).unwrap().parse_literal().unwrap();

      assert_eq!(literal, ast::Literal::String("Hello, world!".into()));
    }
  }
}
