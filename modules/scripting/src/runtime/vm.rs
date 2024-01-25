use common::StringName;

use super::ScriptRuntime;

/// Allows executing script bytecode.
///
/// This is a virtual machine, with a stack-based architecture.
#[derive(Default)]
pub struct VirtualMachine {
  stack: Vec<bytecode::Literal>,
  instruction_pointer: usize,
  instruction: Option<Instruction>,
  frame: Option<StackFrame>,
  call_stack: Vec<StackFrame>,
}

/// A single frame in the call stack.
#[derive(Debug)]
struct StackFrame {
  instruction_pointer: usize,
  instruction: Instruction,
  stack: Vec<bytecode::Literal>,
  locals: Vec<StackLocal>,
}

/// A local variable in a stack frame.
#[derive(Debug)]
struct StackLocal {
  name: StringName,
  value: bytecode::Literal,
}

/// A single instruction in a script.
#[derive(Debug)]
struct Instruction {
  pub opcode: bytecode::OpCode,
}

impl ScriptRuntime for VirtualMachine {}

mod bytecode {
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
  use super::*;

  /// Represents an error that occurred while assembling bytecode.
  #[derive(Debug)]
  pub enum AssemblyError {}

  /// Allows assembly of a type to bytecode.
  pub trait Assembler: Sized {
    /// Assembles the type into bytecode instructions.
    fn assemble(&self) -> Result<Vec<Instruction>, AssemblyError>;
  }

  impl Assembler for crate::lang::ast::Module {
    fn assemble(&self) -> Result<Vec<Instruction>, AssemblyError> {
      use crate::lang::ast::*;

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
