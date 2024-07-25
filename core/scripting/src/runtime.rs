mod callbacks;
mod compiler;
mod isolates;
mod machine;

/// A bytecode instruction for the virtual machine.
#[derive(Debug, PartialEq)]
enum Opcode {
  NoOp,
  Return,
  Constant(u16),
  Unary(crate::lang::ast::UnaryOp),
  Binary(crate::lang::ast::BinaryOp),
  Literal(common::Variant),
  Print,
}
