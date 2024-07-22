//! Scripting language abstractions

pub mod basic;
pub mod lisp;
pub mod wren;

mod ast {
  //! A shared high-level abstract syntax tree for the scripting runtime

  pub enum Statement {
    Expression(Expression),
    Assignment(String, Expression),
    Return(Expression),
  }

  pub enum Expression {
    Literal(Literal),
    Binary(Box<Expression>, BinaryOp, Box<Expression>),
    Unary(UnaryOp, Box<Expression>),
  }

  pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
  }

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

  pub enum UnaryOp {
    Negate,
    Not,
  }
}
