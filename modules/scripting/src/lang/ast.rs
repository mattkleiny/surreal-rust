/// A parsed script module.
///
/// Modules are the top-level unit of compilation in Surreal. Each module
/// represents a single compilation unit, and can be imported by other modules
/// (potentially in different languages).
#[derive(Default)]
pub struct Module {
  pub imports: Vec<Import>,
  pub functions: Vec<Function>,
}

/// An import in a script.
pub struct Import {
  pub path: String,
  pub alias: Option<String>,
}

/// A function in a script.
pub struct Function {
  pub name: String,
  pub parameters: Vec<String>,
  pub body: Vec<Statement>,
}

/// A function parameter.
pub struct Parameter {
  pub name: String,
  pub default: Option<Expression>,
}

/// A statement in a script.
#[derive(Debug, PartialEq)]
pub enum Statement {
  Expression(Expression),
  Assignment(String, Expression),
  If(Expression, Vec<Statement>, Vec<Statement>),
  While(Expression, Vec<Statement>),
  For(String, Expression, Expression, Vec<Statement>),
  Return(Expression),
  Block(Vec<Statement>),
  Continue,
  Break,
}

/// An expression in a script.
#[derive(Debug, PartialEq)]
pub enum Expression {
  Binary(BinaryOperator, Box<Expression>, Box<Expression>),
  Unary(UnaryOperator, Box<Expression>),
  Literal(Literal),
  Identifier(String),
  Call(Box<Expression>, Vec<Expression>),
  Index(Box<Expression>, Box<Expression>),
  Member(Box<Expression>, String),
}

/// A literal value in a script.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
  Number(f64),
  String(String),
  Boolean(bool),
  Nil,
}

/// A binary operator in a binary expression.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulo,
  Power,
  BitwiseAnd,
  BitwiseOr,
  BitwiseXor,
  LeftShift,
  RightShift,
  Equal,
  NotEqual,
  LessThan,
  LessThanOrEqual,
  GreaterThan,
  GreaterThanOrEqual,
  And,
  Or,
}

/// A unary operator in a unary expression.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
  Negate,
  BitwiseNot,
  Not,
}

/// Allows visiting the AST nodes of a script.
#[allow(unused_variables)]
pub trait Visitor {
  fn visit_module(&mut self, module: &Module) {
    for import in &module.imports {
      self.visit_import(import);
    }

    for function in &module.functions {
      self.visit_function(function);
    }
  }

  fn visit_import(&mut self, import: &Import) {
    // no-op
  }

  fn visit_function(&mut self, function: &Function) {
    for statement in &function.body {
      self.visit_statement(statement);
    }
  }

  fn visit_statement(&mut self, statement: &Statement) {
    match statement {
      Statement::Expression(expression) => self.visit_expression(expression),
      _ => {}
    }
  }

  fn visit_expression(&mut self, expression: &Expression) {
    match expression {
      Expression::Literal(literal) => self.visit_literal(literal),
      _ => {}
    }
  }

  fn visit_literal(&mut self, literal: &Literal) {
    // no-op
  }
}

impl Module {
  #[inline(always)]
  pub fn accept(&self, visitor: &mut dyn Visitor) {
    visitor.visit_module(self);
  }
}

impl Import {
  #[inline(always)]
  pub fn accept(&self, visitor: &mut dyn Visitor) {
    visitor.visit_import(self);
  }
}

impl Function {
  #[inline(always)]
  pub fn accept(&self, visitor: &mut dyn Visitor) {
    visitor.visit_function(self);
  }
}

impl Statement {
  #[inline(always)]
  pub fn accept(&self, visitor: &mut dyn Visitor) {
    visitor.visit_statement(self);
  }
}

impl Expression {
  #[inline(always)]
  pub fn accept(&self, visitor: &mut dyn Visitor) {
    visitor.visit_expression(self);
  }
}

impl Literal {
  #[inline(always)]
  pub fn accept(&self, visitor: &mut dyn Visitor) {
    visitor.visit_literal(self);
  }
}

/// Implements support for a token stream over the given token type.
///
/// A token stream is a queue of tokens, with the ability to peek at the next
/// token, useful for building recursive-descent style parsers.
macro_rules! impl_token_stream {
  ($token:ty as $ident:ident) => {
    #[allow(dead_code)]
    struct $ident {
      tokens: std::collections::VecDeque<$token>,
      last_token: Option<$token>,
    }

    #[allow(dead_code)]
    impl $ident {
      /// Peek at the next token in the stream.
      pub fn peek(&self) -> Option<&Token> {
        self.tokens.front()
      }

      /// Take the next token from the stream.
      pub fn take(&mut self) -> Option<&Token> {
        self.last_token = self.tokens.pop_front();
        self.last_token.as_ref()
      }

      /// Returns an error indicating that an unexpected token was encountered.
      pub fn unexpected_token<R>(&self) -> Result<R, ScriptParseError> {
        Err(ScriptParseError::InvalidSyntax(format!(
          "unexpected token encountered: {:?}",
          self.peek()
        )))
      }
    }
  };
}

pub(crate) use impl_token_stream;
