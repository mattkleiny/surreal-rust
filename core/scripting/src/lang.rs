//! Scripting language abstractions

pub mod lox;
pub mod wren;

pub(crate) mod ast {
  //! A shared high-level abstract syntax tree for the scripting runtime

  use common::{ToVariant, Variant};

  /// A block of [`Statement`]s.
  #[derive(Debug, Clone)]
  pub struct Block(pub Vec<Statement>);

  /// A single statement.
  #[derive(Debug, Clone)]
  pub enum Statement {
    Expression(Expression),
    Assignment(String, Expression),
    Return(Expression),
  }

  /// An expression.
  #[derive(Debug, Clone, PartialEq)]
  pub enum Expression {
    Literal(Variant),
    Binary(Box<Expression>, BinaryOp, Box<Expression>),
    Unary(UnaryOp, Box<Expression>),
  }

  /// A literal value.
  #[derive(Debug, Clone, PartialEq)]
  pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
  }

  impl ToVariant for Literal {
    #[inline]
    fn to_variant(&self) -> Variant {
      match self {
        Literal::Integer(value) => value.to_variant(),
        Literal::Float(value) => value.to_variant(),
        Literal::String(value) => value.to_variant(),
      }
    }
  }

  /// Operators for unary expressions.
  #[derive(Debug, Copy, Clone, Eq, PartialEq)]
  pub enum UnaryOp {
    Negate,
  }

  /// Operators for binary expressions.
  #[derive(Debug, Copy, Clone, Eq, PartialEq)]
  pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
  }
}
