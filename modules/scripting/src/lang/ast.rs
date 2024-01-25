/// A parsed script module.
///
/// Modules are the top-level unit of compilation in Surreal. Each module
/// represents a single compilation unit, and can be imported by other modules
/// (potentially in different languages).
pub struct Module {
  statements: Vec<Statement>,
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
#[derive(Debug, PartialEq)]
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

/// Implements support for a token stream over the given token type.
///
/// A token stream is a queue of tokens, with the ability to peek at the next
/// token, useful for building recursive-descent style parsers.
macro_rules! impl_token_stream {
  ($token:ty as $ident:ident) => {
    struct $ident {
      tokens: std::collections::VecDeque<$token>,
      last_token: Option<$token>,
    }

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
    }
  };
}

pub(crate) use impl_token_stream;

/// Allows visiting a script's AST.
#[allow(unused_variables)]
pub trait Visitor {
  fn visit_module(&mut self, module: &Module) {
    for statement in &module.statements {
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
