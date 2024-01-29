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

impl ScriptRuntime for VirtualMachine {
  fn call_function(
    &mut self,
    _name: impl AsRef<str>,
    _parameters: &[common::Variant],
  ) -> Result<Vec<common::Variant>, super::ScriptExecuteError> {
    todo!()
  }
}

mod bytecode {
  pub trait ToByteCode {
    fn to_bytecode(&self, buffer: &mut Vec<u8>);
  }

  #[derive(Debug)]
  pub enum OpCode {
    NoOp,
    Push(Literal),
    Pop,
    Math(MathOp),
    JumpIfTrue(usize),
    JumpIfFalse(usize),
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

  impl ToByteCode for &[super::Instruction] {
    fn to_bytecode(&self, buffer: &mut Vec<u8>) {
      buffer.extend_from_slice(&self.len().to_le_bytes());

      for instruction in self.iter() {
        instruction.to_bytecode(buffer);
      }
    }
  }

  impl ToByteCode for super::Instruction {
    fn to_bytecode(&self, buffer: &mut Vec<u8>) {
      self.opcode.to_bytecode(buffer);
    }
  }

  impl ToByteCode for OpCode {
    fn to_bytecode(&self, buffer: &mut Vec<u8>) {
      match self {
        OpCode::NoOp => {
          buffer.push(0x00);
        }
        OpCode::Push(literal) => {
          buffer.push(0x01);
          literal.to_bytecode(buffer);
        }
        OpCode::Pop => buffer.push(0x02),
        OpCode::Math(math_op) => {
          buffer.push(0x03);
          math_op.to_bytecode(buffer);
        }
        OpCode::JumpIfTrue(offset) => {
          buffer.push(0x04);
          buffer.extend_from_slice(&offset.to_le_bytes());
        }
        OpCode::JumpIfFalse(offset) => {
          buffer.push(0x05);
          buffer.extend_from_slice(&offset.to_le_bytes());
        }
      }
    }
  }

  impl ToByteCode for MathOp {
    fn to_bytecode(&self, buffer: &mut Vec<u8>) {
      match self {
        MathOp::Add => buffer.push(0x00),
        MathOp::Subtract => buffer.push(0x01),
        MathOp::Multiply => buffer.push(0x02),
        MathOp::Divide => buffer.push(0x03),
        MathOp::Modulo => buffer.push(0x04),
        MathOp::Power => buffer.push(0x05),
        MathOp::BitwiseAnd => buffer.push(0x06),
        MathOp::BitwiseOr => buffer.push(0x07),
        MathOp::BitwiseXor => buffer.push(0x08),
        MathOp::LeftShift => buffer.push(0x09),
        MathOp::RightShift => buffer.push(0x0A),
        MathOp::Equal => buffer.push(0x0B),
        MathOp::NotEqual => buffer.push(0x0C),
        MathOp::LessThan => buffer.push(0x0D),
        MathOp::LessThanOrEqual => buffer.push(0x0E),
        MathOp::GreaterThan => buffer.push(0x0F),
        MathOp::GreaterThanOrEqual => buffer.push(0x10),
        MathOp::And => buffer.push(0x11),
        MathOp::Or => buffer.push(0x12),
      }
    }
  }

  impl ToByteCode for Literal {
    fn to_bytecode(&self, buffer: &mut Vec<u8>) {
      match self {
        Literal::Nil => buffer.push(0x00),
        Literal::Number(number) => {
          buffer.push(0x01);
          buffer.extend_from_slice(&number.to_le_bytes());
        }
        Literal::Boolean(bool) => {
          buffer.push(0x02);
          buffer.push(if *bool { 0x01 } else { 0x00 });
        }
        Literal::Float(float) => {
          buffer.push(0x03);
          buffer.extend_from_slice(&float.to_le_bytes());
        }
        Literal::String(string) => {
          buffer.push(0x04);
          buffer.extend_from_slice(&string.len().to_le_bytes());
          buffer.extend_from_slice(string.as_bytes());
        }
      }
    }
  }
}

mod assembly {
  use super::*;

  /// Represents an error that occurred while assembling bytecode.
  #[derive(Debug)]
  pub enum AssemblyError {}

  /// Allows assembly of a type to a VM instruction.
  pub trait Assembler: Sized {
    /// Assembles the type into VM instruction.
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
