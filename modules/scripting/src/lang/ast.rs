//! Abstract syntax tree for the Surreal scripting language frontend.
//!
//! All languages lift to this AST, which is then compilable and executable by
//! the runtime.

/// A parsed script module.
///
/// Modules are the top-level unit of compilation in Surreal. Each module
/// contains a set of functions, and can be import and be imported by other
/// modules (potentially in different languages).
///
/// The module is the root of the AST, and is the entry point for compilation
/// and visitation.
#[derive(Default)]
pub struct Module {
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
  pub statements: Vec<Statement>,
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
  Return(Option<Expression>),
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
    for function in &module.functions {
      self.visit_function(function);
    }
  }

  fn visit_function(&mut self, function: &Function) {
    for statement in &function.statements {
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
      Expression::Binary(_, left, right) => {
        self.visit_expression(&left);
        self.visit_expression(&right);
      }
      Expression::Unary(_, inner) => {
        self.visit_expression(&inner);
      }
      Expression::Literal(literal) => self.visit_literal(literal),
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

      /// Takes the next token, checking if it matches the predicate.
      pub fn take_if(&mut self, predicate: impl FnOnce(&Token) -> bool) -> Option<&Token> {
        if predicate(self.peek()?) {
          self.take()
        } else {
          None
        }
      }

      /// Returns an error indicating that an unexpected token was encountered.
      pub fn unexpected_token<R>(&self) -> Result<R, ParserError> {
        Err(ParserError::InvalidSyntax(format!(
          "unexpected token encountered: {:?}",
          self.peek()
        )))
      }
    }
  };
}

pub(crate) use impl_token_stream;
