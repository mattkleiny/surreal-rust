use super::ScriptRuntime;

/// Allows executing script bytecode.
///
/// This is a virtual machine, with a stack-based architecture.
#[derive(Default)]
pub struct VirtualMachine {
  /// The stack of values.
  stack: Vec<bytecode::OpCode>,
  /// The current instruction pointer.
  instruction_pointer: usize,
  /// The current instruction.
  instruction: Option<Instruction>,
  /// The current frame.
  frame: Option<StackFrame>,
  /// The current call stack.
  call_stack: Vec<StackFrame>,
}

/// A single frame in the call stack.
struct StackFrame {
  /// The instruction pointer for the frame.
  instruction_pointer: usize,
  /// The frame's instruction.
  instruction: Instruction,
  /// The frame's stack.
  stack: Vec<()>,
  /// The frame's locals.
  locals: Vec<()>,
  /// The frame's upvalues.
  upvalues: Vec<()>,
}

/// A single instruction in a script.
#[derive(Debug)]
struct Instruction {
  /// The opcode for the instruction.
  pub opcode: bytecode::OpCode,
}

impl ScriptRuntime for VirtualMachine {}

mod bytecode {
  //! Bytecode representation of a script for the Virtual Machine.

  /// Converts the type into bytecode.
  pub trait ToByteCode {
    fn to_bytecode(&self) -> common::Result<Vec<u8>>;
  }

  /// Converts the bytecode into the type.
  pub trait FromByteCode: Sized {
    fn from_bytecode(bytes: &[u8]) -> common::Result<Self>;
  }

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

  #[derive(Debug)]
  pub enum Literal {
    Number(u64),
    Float(f64),
    String(String),
    Boolean(bool),
    Nil,
  }
}

mod assembly {
  /// Assembler/disassembler for bytecode.
  use super::*;

  /// Allows assembly of a type to bytecode.
  pub trait Assembler: Sized {
    /// Assembles the type into bytecode instructions.
    fn assemble(&self) -> common::Result<Vec<Instruction>>;
  }

  /// Allow assembly of modules to bytecode.
  impl Assembler for crate::lang::Module {
    fn assemble(&self) -> common::Result<Vec<Instruction>> {
      use crate::lang::*;

      struct Assembler {
        instructions: Vec<Instruction>,
      }

      impl Visitor for Assembler {
        fn visit_statement(&mut self, statement: &Statement) {
          match statement {
            Statement::Expression(_) => todo!(),
            Statement::Assignment(_, _) => todo!(),
            Statement::If(_, _, _) => todo!(),
            Statement::While(_, _) => todo!(),
            Statement::For(_, _, _, _) => todo!(),
            Statement::Return(_) => todo!(),
            Statement::Block(_) => todo!(),
            Statement::Continue => todo!(),
            Statement::Break => todo!(),
          }
        }

        fn visit_expression(&mut self, expression: &Expression) {
          match expression {
            Expression::Binary(_, _, _) => todo!(),
            Expression::Unary(_, _) => todo!(),
            Expression::Literal(_) => todo!(),
            Expression::Identifier(_) => todo!(),
            Expression::Call(_, _) => todo!(),
            Expression::Index(_, _) => todo!(),
            Expression::Member(_, _) => todo!(),
          }
        }

        fn visit_literal(&mut self, _literal: &Literal) {
          todo!()
        }
      }

      let mut assembler = Assembler {
        instructions: Vec::new(),
      };

      self.accept(&mut assembler);

      Ok(assembler.instructions)
    }
  }
}
