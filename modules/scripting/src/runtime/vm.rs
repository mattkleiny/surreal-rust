pub use bytecode::*;
pub use compiler::*;

use crate::{ast, ScriptLanguage};

/// A virtual machine, with a stack-based architecture.
///
/// Capable of executing scripts that have been compiled into bytecode.
pub struct VirtualMachine {
  chunk: Chunk,
  stack: Vec<Value>,
  trace_execution: bool,
}

/// An error that can occur when interpreting bytecode.
#[derive(Debug)]
pub enum VirtualMachineError {
  FailedToParse,
  FailedToCompile,
  RuntimeError,
}

impl VirtualMachine {
  /// Creates a new virtual machine.
  pub fn new() -> Self {
    Self {
      chunk: Chunk::default(),
      stack: Vec::new(),
      trace_execution: false,
    }
  }

  /// Creates a new virtual machine with tracing enabled.
  pub fn with_tracing(self) -> Self {
    Self {
      trace_execution: true,
      ..self
    }
  }

  /// Compiles the given code into bytecode and executes it.
  pub fn run<S: ScriptLanguage>(&mut self, code: impl AsRef<str>) -> Result<common::Variant, VirtualMachineError> {
    let module = S::parse_code(code.as_ref()).map_err(|_| VirtualMachineError::FailedToParse)?;
    let chunk = Compiler::compile_module(&module).map_err(|_| VirtualMachineError::FailedToCompile)?;

    self.execute(chunk)
  }

  /// Executes the given chunk of bytecode.
  pub fn execute(&mut self, chunk: Chunk) -> Result<common::Variant, VirtualMachineError> {
    self.chunk = chunk;
    self.advance()
  }

  /// Advances running the virtual machine, executing the existing bytecode.
  pub fn advance(&mut self) -> Result<common::Variant, VirtualMachineError> {
    use common::Variant;

    macro_rules! unary_op {
      ($kind:tt, $operator:tt) => {
        if let Some(Value::$kind(a)) = self.stack.pop() {
          self.stack.push(Value::$kind($operator a));
        } else {
          return Err(VirtualMachineError::RuntimeError);
        }
      };
    }

    macro_rules! binary_op {
      ($kind:tt, $operator:tt) => {
        if let (Some(Value::$kind(a)), Some(Value::$kind(b))) = (self.stack.pop(), self.stack.pop()) {
          self.stack.push(Value::$kind(a $operator b));
        } else {
          return Err(VirtualMachineError::RuntimeError);
        }
      };
    }

    // execute all instructions in the chunk
    while let Some(Instruction { opcode, .. }) = self.chunk.code.pop_front() {
      if self.trace_execution {
        println!("{:?}", opcode.disassemble());
      }

      // execute the instruction
      match opcode {
        // intrinsics
        Opcode::Push(value) => self.stack.push(value),
        Opcode::Return => {
          return match self.stack.pop() {
            Some(value) => Ok(value.into()),
            None => Ok(Variant::Null),
          }
        }
        Opcode::Import => todo!(),

        // arithmetic
        Opcode::Negate => unary_op!(Number, -),
        Opcode::Add => binary_op!(Number, +),
        Opcode::Subtract => binary_op!(Number, -),
        Opcode::Multiply => binary_op!(Number, *),
        Opcode::Divide => binary_op!(Number, /),
        Opcode::Modulo => binary_op!(Number, %),
        Opcode::Power => todo!(),
        Opcode::LeftShift => todo!(),
        Opcode::RightShift => todo!(),

        // comparison
        Opcode::Equal => binary_op!(Boolean, ==),
        Opcode::NotEqual => binary_op!(Boolean, !=),
        Opcode::LessThan => binary_op!(Boolean, <),
        Opcode::LessThanOrEqual => binary_op!(Boolean, <=),
        Opcode::GreaterThan => binary_op!(Boolean, >),
        Opcode::GreaterThanOrEqual => binary_op!(Boolean, >=),

        // logical
        Opcode::And => binary_op!(Boolean, &&),
        Opcode::Or => binary_op!(Boolean, ||),
        Opcode::Not => todo!(),

        // bitwise
        Opcode::BitwiseAnd => todo!(),
        Opcode::BitwiseOr => todo!(),
        Opcode::BitwiseXor => todo!(),
        Opcode::BitwiseNot => todo!(),
      }
    }

    Ok(Variant::Null)
  }

  /// Resets the virtual machine to its initial state.
  pub fn reset(&mut self) {
    self.stack.clear();
  }
}

mod bytecode {
  use std::collections::VecDeque;

  use common::Variant;

  /// A chunk of bytecode to be executed by the virtual machine.
  #[derive(Default, Debug)]
  pub struct Chunk {
    pub code: VecDeque<Instruction>,
  }

  // An instruction with the line number it was found on.
  #[derive(Debug, Clone, PartialEq)]
  pub struct Instruction {
    pub opcode: Opcode,
    pub line: usize,
  }

  /// A value that can be stored on the virtual machine's stack.
  #[derive(Debug, Clone, PartialEq)]
  pub enum Value {
    Nil,
    Number(f64),
    String(String),
    Boolean(bool),
  }

  /// A single instruction in the virtual machine.
  #[repr(u16)]
  #[derive(Debug, Clone, PartialEq)]
  pub enum Opcode {
    // intrinsics
    Push(Value),
    Return,
    Import,

    // arithmetic
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,

    // comparison
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    // logical
    And,
    Or,
    Not,

    // bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
  }

  impl Chunk {
    /// Creates a new chunk from the given bytecode.
    pub fn from_slice(code: &[Instruction]) -> Self {
      Self {
        code: code.into_iter().map(|it| it.to_owned()).collect(),
      }
    }

    /// Disassembles the bytecode into a human-readable format.
    pub fn disassemble(&self) -> String {
      let mut output = String::new();
      let mut line_number = 0;

      for Instruction { line, opcode } in &self.code {
        if line_number == *line {
          output.push_str("|\t");
        } else {
          line_number = *line;
          output.push_str(&format!("{:04}:\t", line_number));
        }

        output.push_str(&opcode.disassemble());
        output.push_str("\n");
      }

      output
    }
  }

  impl Opcode {
    /// Disassembles the opcode into a human-readable format.
    pub fn disassemble(&self) -> String {
      match self {
        // intrinsics
        Opcode::Push(literal) => format!("PUSH {}", literal.disassemble()),
        Opcode::Return => "RETURN".to_string(),
        Opcode::Import => "IMPORT".to_string(),

        // arithmetic
        Opcode::Negate => "NEGATE".to_string(),
        Opcode::Add => "ADD".to_string(),
        Opcode::Subtract => "SUBTRACT".to_string(),
        Opcode::Multiply => "MULTIPLY".to_string(),
        Opcode::Divide => "DIVIDE".to_string(),
        Opcode::Modulo => "MODULO".to_string(),
        Opcode::Power => "POWER".to_string(),
        Opcode::LeftShift => "LEFT_SHIFT".to_string(),
        Opcode::RightShift => "RIGHT_SHIFT".to_string(),

        // comparison
        Opcode::Equal => "EQUAL".to_string(),
        Opcode::NotEqual => "NOT_EQUAL".to_string(),
        Opcode::LessThan => "LESS_THAN".to_string(),
        Opcode::LessThanOrEqual => "LESS_THAN_OR_EQUAL".to_string(),
        Opcode::GreaterThan => "GREATER_THAN".to_string(),
        Opcode::GreaterThanOrEqual => "GREATER_THAN_OR_EQUAL".to_string(),

        // logical
        Opcode::And => "AND".to_string(),
        Opcode::Or => "OR".to_string(),
        Opcode::Not => "NOT".to_string(),

        // bitwise
        Opcode::BitwiseAnd => "BITWISE_AND".to_string(),
        Opcode::BitwiseOr => "BITWISE_OR".to_string(),
        Opcode::BitwiseXor => "BITWISE_XOR".to_string(),
        Opcode::BitwiseNot => "BITWISE_NOT".to_string(),
      }
    }
  }

  impl Value {
    /// Disassembles the value into a human-readable format.
    pub fn disassemble(&self) -> String {
      match self {
        Value::Number(value) => value.to_string(),
        Value::String(value) => value.clone(),
        Value::Boolean(value) => value.to_string(),
        Value::Nil => "NIL".to_string(),
      }
    }
  }

  impl From<Variant> for Value {
    fn from(value: Variant) -> Self {
      match value {
        Variant::Null => Value::Nil,
        Variant::Bool(value) => Value::Boolean(value),
        Variant::U8(value) => Value::Number(value as f64),
        Variant::U16(value) => Value::Number(value as f64),
        Variant::U32(value) => Value::Number(value as f64),
        Variant::U64(value) => Value::Number(value as f64),
        Variant::I8(value) => Value::Number(value as f64),
        Variant::I16(value) => Value::Number(value as f64),
        Variant::I32(value) => Value::Number(value as f64),
        Variant::I64(value) => Value::Number(value as f64),
        Variant::F32(value) => Value::Number(value as f64),
        Variant::F64(value) => Value::Number(value),
        Variant::String(value) => Value::String(value),
        Variant::StringName(value) => Value::String(value.to_string()),
        Variant::Vec2(_) => todo!(),
        Variant::Vec3(_) => todo!(),
        Variant::Vec4(_) => todo!(),
        Variant::Quat(_) => todo!(),
      }
    }
  }

  impl From<Value> for Variant {
    fn from(value: Value) -> Self {
      match value {
        Value::Nil => Variant::Null,
        Value::Number(value) => Variant::F64(value),
        Value::String(value) => Variant::String(value),
        Value::Boolean(value) => Variant::Bool(value),
      }
    }
  }
}

mod compiler {
  use super::*;

  /// Represents an error that occurred while compiling a value.
  #[derive(Debug)]
  pub enum CompileError {}

  /// A compiler for the virtual machine.
  #[derive(Default)]
  pub struct Compiler {
    chunk: bytecode::Chunk,
    line_number: usize,
  }

  impl Compiler {
    /// Compiles the given module into bytecode.
    pub fn compile_module(module: &ast::Module) -> Result<bytecode::Chunk, CompileError> {
      let mut compiler = Compiler::default();

      module.accept(&mut compiler);

      Ok(compiler.finalize())
    }

    /// Pushes a new line onto the chunk.
    pub fn push_line(&mut self) {
      self.line_number += 1;
    }

    /// Pushes a new instruction onto the chunk.
    pub fn push_opcode(&mut self, opcode: bytecode::Opcode) {
      self.chunk.code.push_back(bytecode::Instruction {
        line: self.line_number,
        opcode,
      });
    }

    /// Finalizes the compiler, returning the compiled bytecode.
    pub fn finalize(self) -> bytecode::Chunk {
      self.chunk
    }
  }

  impl ast::Visitor for Compiler {
    fn visit_statement(&mut self, statement: &ast::Statement) {
      use ast::*;

      self.push_line(); // each statement is on a new logical 'line'

      match statement {
        Statement::Expression(expression) => {
          self.visit_expression(expression);
        }
        Statement::Return(Some(expression)) => {
          self.visit_expression(expression);
          self.push_opcode(Opcode::Return);
        }
        Statement::Return(None) => {
          self.push_opcode(Opcode::Push(Value::Nil));
          self.push_opcode(Opcode::Return);
        }
        _ => {}
      }
    }

    fn visit_function(&mut self, function: &ast::Function) {
      for statement in &function.statements {
        self.visit_statement(statement);
      }
    }

    fn visit_expression(&mut self, expression: &ast::Expression) {
      use ast::*;

      match expression {
        Expression::Binary(operator, left, right) => {
          self.visit_expression(left);
          self.visit_expression(right);

          match operator {
            BinaryOperator::Add => self.push_opcode(Opcode::Add),
            BinaryOperator::Subtract => self.push_opcode(Opcode::Subtract),
            BinaryOperator::Multiply => self.push_opcode(Opcode::Multiply),
            BinaryOperator::Divide => self.push_opcode(Opcode::Divide),
            BinaryOperator::Modulo => self.push_opcode(Opcode::Modulo),
            BinaryOperator::Power => self.push_opcode(Opcode::Power),
            BinaryOperator::BitwiseAnd => self.push_opcode(Opcode::BitwiseAnd),
            BinaryOperator::BitwiseOr => self.push_opcode(Opcode::BitwiseOr),
            BinaryOperator::BitwiseXor => self.push_opcode(Opcode::BitwiseXor),
            BinaryOperator::LeftShift => self.push_opcode(Opcode::LeftShift),
            BinaryOperator::RightShift => self.push_opcode(Opcode::RightShift),
            BinaryOperator::Equal => self.push_opcode(Opcode::Equal),
            BinaryOperator::NotEqual => self.push_opcode(Opcode::NotEqual),
            BinaryOperator::LessThan => self.push_opcode(Opcode::LessThan),
            BinaryOperator::LessThanOrEqual => self.push_opcode(Opcode::LessThanOrEqual),
            BinaryOperator::GreaterThan => self.push_opcode(Opcode::GreaterThan),
            BinaryOperator::GreaterThanOrEqual => self.push_opcode(Opcode::GreaterThanOrEqual),
            BinaryOperator::And => self.push_opcode(Opcode::And),
            BinaryOperator::Or => self.push_opcode(Opcode::Or),
          }
        }
        Expression::Unary(operator, inner) => {
          self.visit_expression(inner);

          match operator {
            UnaryOperator::Negate => self.push_opcode(Opcode::Negate),
            UnaryOperator::BitwiseNot => self.push_opcode(Opcode::BitwiseNot),
            UnaryOperator::Not => self.push_opcode(Opcode::Not),
          }
        }
        Expression::Literal(value) => {
          self.visit_literal(value);
        }
      }
    }

    fn visit_literal(&mut self, literal: &ast::Literal) {
      use ast::*;

      self.push_opcode(Opcode::Push(match literal {
        Literal::Number(value) => Value::Number(*value),
        Literal::String(value) => Value::String(value.clone()),
        Literal::Boolean(value) => Value::Boolean(*value),
        Literal::Nil => Value::Nil,
      }));
    }
  }
}

#[cfg(test)]
mod tests {
  use common::Variant;

  use super::*;

  /// Helper macro to create an instruction.
  macro_rules! instruct {
    ($line:literal, $opcode:expr) => {
      bytecode::Instruction {
        line: $line,
        opcode: $opcode,
      }
    };
  }

  #[test]
  fn test_basic_execution_of_chunks() {
    let mut vm = VirtualMachine::new();

    let chunk = Chunk::from_slice(&[
      instruct!(1, Opcode::Push(Value::Number(3.14159))),
      instruct!(1, Opcode::Push(Value::Number(1.28456))),
      instruct!(1, Opcode::Add),
      instruct!(2, Opcode::Return),
      instruct!(3, Opcode::Push(Value::Number(5.0))),
      instruct!(3, Opcode::Negate),
      instruct!(4, Opcode::Return),
    ]);

    let result = vm.execute(chunk).expect("failed to interpret chunk");

    assert_eq!(result, Variant::F64(4.42615));

    let result = vm.advance().expect("failed to run chunk");

    assert_eq!(result, Variant::F64(-5.0));
  }

  #[test]
  fn test_basic_compilation() {
    use ast::*;

    let module = Module {
      functions: vec![Function {
        name: "main".to_string(),
        statements: vec![Statement::Return(Some(Expression::Binary(
          BinaryOperator::Add,
          Box::new(Expression::Literal(Literal::Number(3.14159))),
          Box::new(Expression::Literal(Literal::Number(1.12345))),
        )))],
        parameters: vec![],
      }],
    };

    let chunk = Compiler::compile_module(&module).expect("failed to compile module");
    let mut vm = VirtualMachine::new();

    let result = vm.execute(chunk).expect("failed to interpret chunk");

    assert_eq!(result, Variant::F64(4.26504));
  }
}
