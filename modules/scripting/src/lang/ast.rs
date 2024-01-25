//! Shared internal AST for the scripting languages.

/// A parsed script module.
///
/// Modules are the top-level unit of compilation in Surreal. Each module
/// represents a single compilation unit, and can be imported by other modules
/// (potentially in different languages).
pub struct ScriptModule {}

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
