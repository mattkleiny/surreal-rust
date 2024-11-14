//! Runtime components for script engine.

pub mod compiler;
pub mod isolates;
pub mod machine;

/// A bytecode instruction for the virtual machine.
#[derive(Debug, PartialEq)]
pub enum Opcode {
  NoOp,
  Return,
  Constant(u16),
  Unary(crate::lang::ast::UnaryOp),
  Binary(crate::lang::ast::BinaryOp),
  Literal(common::Variant),
  Print,
}
