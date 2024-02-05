use super::*;

/// The BASIC [`ScriptLanguage`] implementation.
///
/// This is a very simple language that is intended to be used for testing.
/// It is not intended to be used for any real scripting.
pub struct BASIC;

impl ScriptLanguage for BASIC {
  fn name() -> &'static str {
    "BASIC"
  }

  fn file_extensions() -> &'static [&'static str] {
    &["bas", "basic"]
  }

  fn parse_code(code: &str) -> Result<ast::Module, ParserError> {
    parser::parse(code)
  }
}

mod parser {
  use std::collections::VecDeque;

  use super::*;

  /// Parses the given BASIC code into a [`Module`].
  pub fn parse(code: &str) -> Result<ast::Module, ParserError> {
    let mut stream = TokenStream::tokenize(code)?;
    let module = stream.parse_script_module()?;

    Ok(module)
  }

  /// A keyword in a BASIC script.
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

  /// An operator in a BASIC script.
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

  /// A text token in a BASIC script.
  #[derive(Debug, PartialEq)]
  enum Token {
    Number(f64),
    Keyword(Keyword),
    Identifier(String),
    Operator(Operator),
  }

  impl TokenStream {
    pub fn parse_script_module(&mut self) -> Result<ast::Module, ParserError> {
      let mut module = ast::Module::default();
      let mut main = ast::Function {
        name: "main".to_string(),
        parameters: Vec::new(),
        statements: Vec::new(),
      };

      while let Ok(statement) = self.parse_statement() {
        main.statements.push(statement);
      }

      module.functions.push(main);

      Ok(module)
    }

    pub fn parse_statement(&mut self) -> Result<ast::Statement, ParserError> {
      match self.take() {
        Some(Token::Keyword(Keyword::Return)) => {
          if self.peek().is_none() {
            Ok(ast::Statement::Return(None))
          } else {
            Ok(ast::Statement::Return(Some(self.parse_expression()?)))
          }
        }
        _ => self.unexpected_token(),
      }
    }

    pub fn parse_expression(&mut self) -> Result<ast::Expression, ParserError> {
      self.parse_binary_expression()
    }

    pub fn parse_binary_expression(&mut self) -> Result<ast::Expression, ParserError> {
      let left = self.parse_unary_expression()?;

      if let Some(operator) = self.parse_binary_operator() {
        let right = self.parse_unary_expression()?;

        return Ok(ast::Expression::Binary(operator, Box::new(left), Box::new(right)));
      }

      Ok(left)
    }

    fn parse_unary_expression(&mut self) -> Result<ast::Expression, ParserError> {
      if let Some(operator) = self.parse_unary_operator() {
        let inner = self.parse_unary_expression()?;

        return Ok(ast::Expression::Unary(operator, Box::new(inner)));
      }

      self.parse_primary_expression()
    }

    pub fn parse_primary_expression(&mut self) -> Result<ast::Expression, ParserError> {
      match self.take() {
        Some(Token::Number(value)) => Ok(ast::Expression::Literal(ast::Literal::Number(*value))),
        _ => self.unexpected_token(),
      }
    }

    pub fn parse_binary_operator(&mut self) -> Option<ast::BinaryOperator> {
      match self.take_if(|it| matches!(it, Token::Operator(_))) {
        Some(Token::Operator(operator)) => Some(match operator {
          Operator::Plus => ast::BinaryOperator::Add,
          Operator::Minus => ast::BinaryOperator::Subtract,
          Operator::Multiply => ast::BinaryOperator::Multiply,
          Operator::Divide => ast::BinaryOperator::Divide,
          Operator::Modulo => ast::BinaryOperator::Modulo,
          Operator::Equal => ast::BinaryOperator::Equal,
          Operator::LessThan => ast::BinaryOperator::LessThan,
          Operator::GreaterThan => ast::BinaryOperator::GreaterThan,
          Operator::And => ast::BinaryOperator::And,
          Operator::Or => ast::BinaryOperator::Or,
        }),
        _ => None,
      }
    }

    pub fn parse_unary_operator(&mut self) -> Option<ast::UnaryOperator> {
      match self.take_if(|it| matches!(it, Token::Operator(_))) {
        Some(Token::Operator(operator)) => Some(match operator {
          Operator::Minus => ast::UnaryOperator::Negate,
          _ => return None,
        }),
        _ => None,
      }
    }

    /// Tokenizes the given BASIC code into a [`TokenStream`].
    pub fn tokenize(code: &str) -> Result<Self, ParserError> {
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
              ParserError::InvalidSyntax("an invalid number was encountered".to_string())
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
            return Err(ParserError::InvalidSyntax(
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
    use crate::ast::*;

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
      let literal = TokenStream::tokenize(code).unwrap().parse_primary_expression().unwrap();

      assert_eq!(literal, Expression::Literal(Literal::Number(3.14159)));
    }

    #[test]
    fn test_parse_basic_statement() {
      let code = "RETURN 3.14159 + 2.71828";

      let mut stream = TokenStream::tokenize(code).unwrap();
      let statement = stream.parse_statement().unwrap();

      assert_eq!(
        statement,
        Statement::Return(Some(Expression::Binary(
          BinaryOperator::Add,
          Box::new(Expression::Literal(Literal::Number(3.14159))),
          Box::new(Expression::Literal(Literal::Number(2.71828)))
        )))
      );
    }
  }
}
