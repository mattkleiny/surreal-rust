use super::ScriptRuntime;

/// Allows executing script bytecode.
pub struct ScriptVirtualMachine {}

impl ScriptRuntime for ScriptVirtualMachine {}

/// A single instruction in a script.
#[derive(Debug)]
struct Instruction {
  pub opcode: bytecode::OpCode,
  pub line: u32,
}

impl Instruction {
  /// Converts the given bytecode into an instruction.
  pub fn from_bytes(_bytes: &[u8]) -> common::Result<Self> {
    todo!()
  }

  /// Converts the instruction into bytecode.
  pub fn into_bytes(self) -> Vec<u8> {
    todo!()
  }
}

mod assembly {
  /// Assembler/disassembler for bytecode.
  use super::*;

  /// Allows assembly of a type to bytecode.
  pub trait Assembler: Sized {
    /// Assembles the type into bytecode.
    fn assemble(&self) -> common::Result<Vec<Instruction>>;
  }

  /// Allow assembly of [`ScriptModule`]s to bytecode.
  impl Assembler for crate::lang::ScriptModule {
    fn assemble(&self) -> common::Result<Vec<Instruction>> {
      todo!()
    }
  }
}

mod bytecode {
  //! Bytecode representation of a script for the Virtual Machine.

  /// A single opcode in a script.
  #[derive(Debug)]
  pub enum OpCode {
    NoOp,
    Stack(StackOp),
    Math(MathOp),
  }

  #[derive(Debug)]
  pub enum StackOp {
    Push(Literal),
    Pop,
  }

  #[derive(Debug)]
  pub enum Literal {
    Number(u64),
    Float(f64),
    String(String),
    Boolean(bool),
    Nil,
  }

  #[derive(Debug)]
  pub enum MathOp {
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
}
