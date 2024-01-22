//! Shady language support for the shader system

use super::*;

/// The Shady [`ShaderLanguage`] implementation.
pub struct Shady;

impl ShaderProgram {
  /// Loads a [`ShaderProgram`] from the given raw GLSL shader code.
  pub fn from_shady(graphics: &GraphicsEngine, code: &str) -> common::Result<Self> {
    Self::from_code::<Shady>(graphics, code)
  }

  /// Loads a [`ShaderProgram`] from the given raw shady shader code file.
  pub fn from_shady_path<'a>(graphics: &GraphicsEngine, path: impl Into<VirtualPath<'a>>) -> common::Result<Self> {
    Self::from_path::<Shady>(graphics, path)
  }

  /// Loads a [`ShaderProgram`] from the given raw shady stream.
  pub fn from_shady_stream<'a>(graphics: &GraphicsEngine, stream: &mut dyn InputStream) -> common::Result<Self> {
    Self::from_stream::<Shady>(graphics, stream)
  }
}

impl ShaderLanguage for Shady {
  /// Parses the given raw Shady source and compiles it shader kernels.
  fn parse_kernels(source_code: &str) -> common::Result<Vec<ShaderKernel>> {
    let module = parser::parse(source_code)?;
    let kernels = compiler::compile(module)?;

    Ok(kernels)
  }
}

mod compiler {
  //! A compiler for the Shady language
  //!
  //! This compiler will transpile Shady code into GLSL code that can be used
  //! with the graphics engine.
  use super::*;

  /// Compiles the given Shady module into a list of [`ShaderKernel`]s.
  pub fn compile(_module: parser::Module) -> common::Result<Vec<ShaderKernel>> {
    // TODO: implement compiler for Shady

    todo!()
  }
}

mod parser {
  //! A parser for the Shady language
  //!
  //! This parser will parse Shady code into a list of statements that can be
  //! compiled into [`ShaderKernel`]s.
  use std::collections::VecDeque;

  use super::*;

  /// Parses the given Shady source code into a module.
  pub fn parse(code: &str) -> common::Result<Module> {
    Module::parse(code)
  }

  /// A trait for types that can be parsed from a string.
  trait Parseable: Sized {
    /// Parse the given code into a result.
    fn parse(code: &str) -> common::Result<Self>;
  }

  /// Possible types of Shady modules.
  #[derive(Debug, PartialEq)]
  pub enum ModuleKind {
    Standard,
    Canvas,
    Sprite,
    Model,
  }

  /// A type of kernel in Shady.
  #[derive(Debug, PartialEq)]
  pub struct Module {
    pub kind: ModuleKind,
    pub kernels: Vec<Kernel>,
  }

  impl Parseable for Module {
    fn parse(code: &str) -> common::Result<Self> {
      let mut stream = TokenStream::parse(code)?;
      let module = stream.parse_module()?;

      Ok(module)
    }
  }

  /// Possible types of Shady kernels.
  #[derive(Debug, PartialEq)]
  pub enum KernelKind {
    Vertex,
    Fragment,
  }

  /// A Shady kernel.
  #[derive(Debug, PartialEq)]
  pub struct Kernel {
    pub kind: KernelKind,
    pub name: String,
    pub statements: Vec<Statement>,
  }

  impl Parseable for Kernel {
    fn parse(code: &str) -> common::Result<Self> {
      let mut stream = TokenStream::parse(code)?;
      let kernel = stream.parse_kernel()?;

      Ok(kernel)
    }
  }

  /// A parameter in a Shady function.
  #[derive(Debug, PartialEq)]
  pub struct Parameter {
    pub name: String,
    pub primitive: Primitive,
  }

  /// A cardinality of a vector or matrix in Shady.
  pub type Cardinality = u8;

  /// A kind of primitive in Shady, along with it's cardinality.
  #[derive(Debug, PartialEq)]
  pub struct Primitive(PrimitiveKind, Cardinality);

  /// A kind of primitive in Shady.
  #[derive(Debug, PartialEq)]
  pub enum PrimitiveKind {
    Integer,
    Float,
    Boolean,
    Vector,
    Matrix,
    Sampler,
  }

  /// Possible types of Shady statements.
  #[derive(Debug, PartialEq)]
  pub enum Statement {
    Assignment(String, Expression),
    Return(Expression),
    Function(String, Vec<Parameter>, Vec<Statement>),
  }

  impl Parseable for Statement {
    fn parse(code: &str) -> common::Result<Self> {
      let mut stream = TokenStream::parse(code)?;
      let statement = stream.parse_statement()?;

      Ok(statement)
    }
  }

  /// Possible types of Shady expressions.
  #[derive(Debug, PartialEq)]
  pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
  }

  impl Parseable for Expression {
    fn parse(code: &str) -> common::Result<Self> {
      let mut stream = TokenStream::parse(code)?;
      let expression = stream.parse_expression()?;

      Ok(expression)
    }
  }

  /// A unary operator in a Shady expression.
  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  pub enum UnaryOperator {
    Not,
  }

  /// A binary operator in a Shady expression.
  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    LessThan,
    GreaterThan,
    And,
    Or,
  }

  /// A literal value.
  #[derive(Debug, PartialEq)]
  pub enum Literal {
    Integer(i32),
    Float(f32),
    Boolean(bool),
  }

  /// A token in the token stream.
  #[derive(Debug, PartialEq)]
  pub enum Token {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator),
    Keyword(String),
    Identifier(String),
    Semicolon,
    Colon,
    LeftBrace,
    RightBrace,
    LeftParenthesis,
    RightParenthesis,
    Comma,
  }

  /// A helper for parsing and working with a stream of tokens.
  ///
  /// This is a recursive descent context for the parser.
  pub struct TokenStream {
    tokens: VecDeque<Token>,
    last_token: Option<Token>,
  }

  impl TokenStream {
    /// Peek at the next token in the stream.
    pub fn peek(&self) -> Option<&Token> {
      self.tokens.front()
    }

    /// Peek at the next token in the stream, checking against the predicate.
    pub fn matches(&self, predicate: impl FnOnce(&Token) -> bool) -> bool {
      match self.peek() {
        Some(token) => predicate(token),
        None => false,
      }
    }

    /// Take the next token from the stream.
    pub fn take(&mut self) -> Option<&Token> {
      self.last_token = self.tokens.pop_front();
      println!("took token: {:?}", self.last_token);

      self.last_token.as_ref()
    }

    /// Takes the next token, checking if it matches the predicate.
    pub fn take_if(&mut self, predicate: impl FnOnce(&Token) -> bool) -> Option<&Token> {
      if predicate(self.peek()?) {
        self.take()
      } else {
        None
      }
    }

    /// Takes the next token, expecting it to match the predicate.
    pub fn take_expect(&mut self, token: Token) -> common::Result<&Token> {
      if self.take_if(|it| *it == token).is_none() {
        return self.unexpected_token();
      }

      Ok(self.last_token.as_ref().unwrap())
    }

    /// Parses a module from the token stream.
    pub fn parse_module(&mut self) -> common::Result<Module> {
      match self.peek() {
        Some(Token::Keyword(keyword)) if keyword == "#shader_type" => self.parse_shader_type_module(),
        _ => self.parse_module_of_kind(ModuleKind::Standard),
      }
    }

    /// Parses a shader type module from the token stream.
    pub fn parse_shader_type_module(&mut self) -> common::Result<Module> {
      self.take_expect(Token::Keyword("#shader_type".to_string()))?;

      let kind = match self.take() {
        Some(Token::Identifier(identifier)) if identifier == "canvas" => ModuleKind::Canvas,
        Some(Token::Identifier(identifier)) if identifier == "sprite" => ModuleKind::Sprite,
        Some(Token::Identifier(identifier)) if identifier == "model" => ModuleKind::Model,
        Some(Token::Identifier(identifier)) if identifier == "standard" => ModuleKind::Standard,

        _ => return self.unexpected_token(),
      };

      self.parse_module_of_kind(kind)
    }

    /// Parses a standard module from the token stream.
    pub fn parse_module_of_kind(&mut self, kind: ModuleKind) -> common::Result<Module> {
      let mut kernels = Vec::new();

      while self.matches(|it| matches!(it, Token::Keyword(_))) {
        kernels.push(self.parse_kernel()?);
      }

      Ok(Module { kind, kernels })
    }

    /// Parses a kernel from the token stream.
    pub fn parse_kernel(&mut self) -> common::Result<Kernel> {
      match self.peek() {
        Some(Token::Keyword(keyword)) if keyword == "fn" => self.parse_function_kernel(),
        _ => self.unexpected_token(),
      }
    }

    /// Parses a function kernel from the token stream.
    pub fn parse_function_kernel(&mut self) -> common::Result<Kernel> {
      if let Statement::Function(name, parameters, statements) = self.parse_statement()? {
        if parameters.len() != 0 {
          return Err(common::anyhow!("function kernels cannot have parameters"));
        }

        let kind = match name.as_ref() {
          "vertex" => KernelKind::Vertex,
          "fragment" => KernelKind::Fragment,
          _ => return Err(common::anyhow!("invalid kernel name: {}", name)),
        };

        Ok(Kernel { kind, name, statements })
      } else {
        self.unexpected_token()
      }
    }

    /// Parses a statement from the token stream.
    pub fn parse_statement(&mut self) -> common::Result<Statement> {
      match self.take_if(|it| matches!(it, Token::Keyword(_))) {
        Some(Token::Keyword(keyword)) if keyword == "let" => self.parse_let_statement(),
        Some(Token::Keyword(keyword)) if keyword == "return" => self.parse_return_statement(),
        Some(Token::Keyword(keyword)) if keyword == "fn" => self.parse_function_statement(),
        _ => self.unexpected_token(),
      }
    }

    /// Parses a let statement from the token stream.
    pub fn parse_let_statement(&mut self) -> common::Result<Statement> {
      let identifier = match self.take() {
        Some(Token::Identifier(identifier)) => identifier.clone(),
        _ => return self.unexpected_token(),
      };

      self.take_expect(Token::BinaryOperator(BinaryOperator::Equal))?;
      let expression = self.parse_expression()?;
      self.take_expect(Token::Semicolon)?;

      Ok(Statement::Assignment(identifier, expression))
    }

    /// Parses a return statement from the token stream.
    pub fn parse_return_statement(&mut self) -> common::Result<Statement> {
      let expression = self.parse_expression()?;

      self.take_expect(Token::Semicolon)?;

      Ok(Statement::Return(expression))
    }

    /// Parses a function statement from the token stream.
    pub fn parse_function_statement(&mut self) -> common::Result<Statement> {
      let name = match self.take() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => return self.unexpected_token(),
      };

      self.take_expect(Token::LeftParenthesis)?;

      let mut parameters = Vec::new();

      while !self.matches(|it| matches!(it, Token::RightParenthesis)) {
        parameters.push(self.parse_parameter()?);

        self.take_if(|it| matches!(it, Token::Comma));
      }

      self.take_expect(Token::RightParenthesis)?;
      self.take_expect(Token::LeftBrace)?;

      let mut statements = Vec::new();

      while !self.matches(|it| matches!(it, Token::RightBrace)) {
        statements.push(self.parse_statement()?);
      }

      self.take_expect(Token::RightBrace)?;

      Ok(Statement::Function(name, parameters, statements))
    }

    /// Parses a parameter from the token stream.
    pub fn parse_parameter(&mut self) -> common::Result<Parameter> {
      let primitive = self.parse_primitive()?;
      let name = match self.take() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => return self.unexpected_token(),
      };

      Ok(Parameter { name, primitive })
    }

    /// Parses an expression from the token stream.
    pub fn parse_expression(&mut self) -> common::Result<Expression> {
      self.parse_binary_expression()
    }

    /// Parses a binary expression from the token stream.
    pub fn parse_binary_expression(&mut self) -> common::Result<Expression> {
      let mut expression = self.parse_unary_expression()?;

      while let Ok(operator) = self.parse_binary_operator() {
        let right = self.parse_unary_expression()?;

        expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
      }

      Ok(expression)
    }

    /// Parses a unary expression from the token stream.
    pub fn parse_unary_expression(&mut self) -> common::Result<Expression> {
      if let Ok(operator) = self.parse_unary_operator() {
        let expression = self.parse_unary_expression()?;

        return Ok(Expression::Unary(operator, Box::new(expression)));
      }

      self.parse_primary_expression()
    }

    /// Parses a primary expression from the token stream.
    pub fn parse_primary_expression(&mut self) -> common::Result<Expression> {
      match self.take() {
        Some(Token::Integer(value)) => Ok(Expression::Literal(Literal::Integer(*value))),
        Some(Token::Float(value)) => Ok(Expression::Literal(Literal::Float(*value))),
        Some(Token::Boolean(value)) => Ok(Expression::Literal(Literal::Boolean(*value))),
        Some(Token::Identifier(name)) => Ok(Expression::Identifier(name.clone())),
        _ => self.unexpected_token(),
      }
    }

    /// Parses a binary operator from the token stream.
    pub fn parse_binary_operator(&mut self) -> common::Result<BinaryOperator> {
      match self.take_if(|it| matches!(it, Token::BinaryOperator(_))) {
        Some(Token::BinaryOperator(operator)) => Ok(*operator),
        _ => self.unexpected_token(),
      }
    }

    /// Parses a unary operator from the token stream.
    pub fn parse_unary_operator(&mut self) -> common::Result<UnaryOperator> {
      match self.take_if(|it| matches!(it, Token::UnaryOperator(_))) {
        Some(Token::UnaryOperator(operator)) => Ok(*operator),
        _ => self.unexpected_token(),
      }
    }

    /// Parses a primitive from the token stream.
    pub fn parse_primitive(&mut self) -> common::Result<Primitive> {
      match self.take() {
        Some(Token::Keyword(keyword)) if keyword == "int" => Ok(Primitive(PrimitiveKind::Integer, 1)),
        Some(Token::Keyword(keyword)) if keyword == "float" => Ok(Primitive(PrimitiveKind::Float, 1)),
        Some(Token::Keyword(keyword)) if keyword == "bool" => Ok(Primitive(PrimitiveKind::Boolean, 1)),
        Some(Token::Keyword(keyword)) if keyword == "vec2" => Ok(Primitive(PrimitiveKind::Vector, 2)),
        Some(Token::Keyword(keyword)) if keyword == "vec3" => Ok(Primitive(PrimitiveKind::Vector, 3)),
        Some(Token::Keyword(keyword)) if keyword == "vec4" => Ok(Primitive(PrimitiveKind::Vector, 4)),
        Some(Token::Keyword(keyword)) if keyword == "mat2" => Ok(Primitive(PrimitiveKind::Matrix, 2)),
        Some(Token::Keyword(keyword)) if keyword == "mat3" => Ok(Primitive(PrimitiveKind::Matrix, 3)),
        Some(Token::Keyword(keyword)) if keyword == "mat4" => Ok(Primitive(PrimitiveKind::Matrix, 4)),
        Some(Token::Keyword(keyword)) if keyword == "sampler1D" => Ok(Primitive(PrimitiveKind::Sampler, 1)),
        Some(Token::Keyword(keyword)) if keyword == "sampler2D" => Ok(Primitive(PrimitiveKind::Sampler, 2)),
        Some(Token::Keyword(keyword)) if keyword == "sampler3D" => Ok(Primitive(PrimitiveKind::Sampler, 3)),
        _ => return self.unexpected_token(),
      }
    }

    /// Creates an error for an unexpected token.
    pub fn unexpected_token<R>(&self) -> common::Result<R> {
      Err(common::anyhow!("unexpected token encountered: {:?}", self.peek()))
    }
  }

  impl Parseable for TokenStream {
    fn parse(code: &str) -> common::Result<Self> {
      const KEYWORDS: [&str; 22] = [
        "true",
        "false",
        "let",
        "return",
        "if",
        "else",
        "while",
        "for",
        "fn",
        "int",
        "float",
        "bool",
        "vec2",
        "vec3",
        "vec4",
        "mat2",
        "mat3",
        "mat4",
        "sampler1D",
        "sampler2D",
        "sampler3D",
        "#shader_type",
      ];

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

            if number.contains('.') {
              tokens.push_back(Token::Float(number.parse()?));
            } else {
              tokens.push_back(Token::Integer(number.parse()?));
            }
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

            if KEYWORDS.contains(&string.as_ref()) {
              match string.as_ref() {
                "true" => tokens.push_back(Token::Boolean(true)),
                "false" => tokens.push_back(Token::Boolean(false)),
                _ => tokens.push_back(Token::Keyword(string)),
              }
            } else {
              tokens.push_back(Token::Identifier(string));
            }
          }

          // parse operators
          '+' => tokens.push_back(Token::BinaryOperator(BinaryOperator::Add)),
          '-' => tokens.push_back(Token::BinaryOperator(BinaryOperator::Subtract)),
          '*' => tokens.push_back(Token::BinaryOperator(BinaryOperator::Multiply)),
          '/' => tokens.push_back(Token::BinaryOperator(BinaryOperator::Divide)),
          '%' => tokens.push_back(Token::BinaryOperator(BinaryOperator::Modulo)),
          '^' => tokens.push_back(Token::BinaryOperator(BinaryOperator::Power)),
          '=' => tokens.push_back(Token::BinaryOperator(BinaryOperator::Equal)),
          '!' => tokens.push_back(Token::UnaryOperator(UnaryOperator::Not)),
          '<' => tokens.push_back(Token::BinaryOperator(BinaryOperator::LessThan)),
          '>' => tokens.push_back(Token::BinaryOperator(BinaryOperator::GreaterThan)),
          '&' => tokens.push_back(Token::BinaryOperator(BinaryOperator::And)),
          '|' => tokens.push_back(Token::BinaryOperator(BinaryOperator::Or)),
          ';' => tokens.push_back(Token::Semicolon),
          ':' => tokens.push_back(Token::Colon),
          '(' => tokens.push_back(Token::LeftParenthesis),
          ')' => tokens.push_back(Token::RightParenthesis),
          '{' => tokens.push_back(Token::LeftBrace),
          '}' => tokens.push_back(Token::RightBrace),
          ',' => tokens.push_back(Token::Comma),

          // parse other tokens
          _ => panic!("unexpected token: {}", c),
        }
      }

      Ok(TokenStream {
        tokens,
        last_token: None,
      })
    }
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_tokens_from_expression() {
      let code = r"1 + 2 * 3.14159 - 4";

      let stream = TokenStream::parse(code).unwrap();

      assert_eq!(
        stream.tokens,
        vec![
          Token::Integer(1),
          Token::BinaryOperator(BinaryOperator::Add),
          Token::Integer(2),
          Token::BinaryOperator(BinaryOperator::Multiply),
          Token::Float(3.14159),
          Token::BinaryOperator(BinaryOperator::Subtract),
          Token::Integer(4),
        ]
      );
    }

    #[test]
    fn test_parse_keywords_and_identifiers_from_expression() {
      let code = r"let x = 1 + 2;";

      let stream = TokenStream::parse(code).unwrap();

      assert_eq!(
        stream.tokens,
        vec![
          Token::Keyword("let".to_string()),
          Token::Identifier("x".to_string()),
          Token::BinaryOperator(BinaryOperator::Equal),
          Token::Integer(1),
          Token::BinaryOperator(BinaryOperator::Add),
          Token::Integer(2),
          Token::Semicolon,
        ]
      );
    }

    #[test]
    fn test_parse_basic_expression() {
      let code = r"1 + 2";

      let expression = Expression::parse(code).unwrap();

      assert_eq!(
        expression,
        Expression::Binary(
          Box::new(Expression::Literal(Literal::Integer(1))),
          BinaryOperator::Add,
          Box::new(Expression::Literal(Literal::Integer(2))),
        )
      );
    }

    #[test]
    fn test_parse_basic_statement() {
      let code = r"let x = 1 + 2;";

      let statements = Statement::parse(code).unwrap();

      assert_eq!(
        statements,
        Statement::Assignment(
          "x".to_string(),
          Expression::Binary(
            Box::new(Expression::Literal(Literal::Integer(1))),
            BinaryOperator::Add,
            Box::new(Expression::Literal(Literal::Integer(2))),
          ),
        )
      );
    }

    #[test]
    fn test_parse_basic_function() {
      let code = r"
        fn add(int a, int b) {
          return a + b;
        }
      ";

      let function = Statement::parse(code).unwrap();

      assert_eq!(
        function,
        Statement::Function(
          "add".to_string(),
          vec![
            Parameter {
              name: "a".to_string(),
              primitive: Primitive(PrimitiveKind::Integer, 1)
            },
            Parameter {
              name: "b".to_string(),
              primitive: Primitive(PrimitiveKind::Integer, 1)
            },
          ],
          vec![Statement::Return(Expression::Binary(
            Box::new(Expression::Identifier("a".to_string())),
            BinaryOperator::Add,
            Box::new(Expression::Identifier("b".to_string())),
          ))],
        )
      );
    }

    #[test]
    fn test_parse_basic_kernel() {
      let code = r"
        fn fragment() {
          return 1 + 2;
        }
      ";

      let kernel = Kernel::parse(code).unwrap();

      assert_eq!(
        kernel,
        Kernel {
          kind: KernelKind::Fragment,
          name: "fragment".to_string(),
          statements: vec![Statement::Return(Expression::Binary(
            Box::new(Expression::Literal(Literal::Integer(1))),
            BinaryOperator::Add,
            Box::new(Expression::Literal(Literal::Integer(2))),
          ))],
        }
      );
    }

    #[test]
    fn test_parse_module() {
      let code = r"
        #shader_type canvas

        fn fragment() {
          return 1 + 2;
        }
      ";

      let module = Module::parse(code).unwrap();

      assert_eq!(
        module,
        Module {
          kind: ModuleKind::Canvas,
          kernels: vec![Kernel {
            kind: KernelKind::Fragment,
            name: "fragment".to_string(),
            statements: vec![Statement::Return(Expression::Binary(
              Box::new(Expression::Literal(Literal::Integer(1))),
              BinaryOperator::Add,
              Box::new(Expression::Literal(Literal::Integer(2))),
            ))],
          }],
        }
      );
    }

    #[test]
    fn test_parse_full_shady_program() {
      let code = include_str!("../embedded/sprite-standard.shady");

      let module = parse(code).unwrap();

      assert_eq!(module.kernels.len(), 1);
    }
  }
}
