use super::ScriptLanguage;

/// The BASIC [`ScriptLanguage`] implementation.
///
/// This is a very simple language that is intended to be used for testing.
/// It is not intended to be used for any real scripting.
pub struct Basic;

impl ScriptLanguage for Basic {
  fn name(&self) -> &'static str {
    "BASIC"
  }

  fn file_extensions(&self) -> &[&'static str] {
    &["bas", "basic"]
  }

  fn compile_code(&self, code: &str) -> common::Result<()> {
    let _module = parser::parse(code)?;

    todo!()
  }
}

mod parser {
  //! Parser for the BASIC language.
  use std::collections::VecDeque;

  use crate::lang::ast;

  /// Parses the given BASIC code into a [`Module`].
  pub fn parse(code: &str) -> common::Result<ast::Module> {
    let mut stream = TokenStream::tokenize(code)?;
    let module = stream.parse_script_module()?;

    Ok(module)
  }

  #[derive(Debug, PartialEq)]
  enum Token {
    Number(f64),
    StringLiteral(String),
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
    /// Parses a [`ScriptModule`] from the [`TokenStream`].
    pub fn parse_script_module(&mut self) -> common::Result<ast::Module> {
      todo!()
    }

    pub fn parse_statement(&mut self) -> common::Result<ast::Statement> {
      todo!()
    }

    pub fn parse_expression(&mut self) -> common::Result<ast::Expression> {
      todo!()
    }

    pub fn parse_literal(&mut self) -> common::Result<ast::Literal> {
      todo!()
    }

    /// Tokenizes the given BASIC code into a [`TokenStream`].
    pub fn tokenize(code: &str) -> common::Result<Self> {
      // tokenize the code
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

            tokens.push_back(Token::Number(number.parse()?));
          }

          // parse keywords and identifiers
          c if c.is_ascii_alphabetic() || c == '#' => {
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

            tokens.push_back(match string.as_str() {
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
          _ => common::bail!("unexpected token: {}", c),
        }
      }

      Ok(TokenStream {
        tokens,
        last_token: None,
      })
    }
  }

  crate::lang::ast::impl_token_stream!(Token as TokenStream);

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
  }
}
