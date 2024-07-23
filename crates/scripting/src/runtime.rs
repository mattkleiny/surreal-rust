pub use compiler::*;
pub use machine::*;

mod compiler;
mod machine;

/// A bytecode instruction for the virtual machine.
#[derive(Debug, PartialEq)]
pub enum Opcode {
  NoOp,
  Return,
  Constant(TableIndex),
  Unary(crate::ast::UnaryOp),
  Binary(crate::ast::BinaryOp),
  Literal(crate::ast::Literal),
  Print,
}
