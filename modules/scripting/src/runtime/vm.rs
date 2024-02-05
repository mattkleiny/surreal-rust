use common::Variant;

use crate::ast;

/// A virtual machine, with a stack-based architecture.
///
/// Capable of executing scripts that have been compiled into bytecode.
pub struct VirtualMachine {
  chunk: bytecode::Chunk,
  stack: Vec<bytecode::Value>,
  trace_execution: bool,
}

/// An error that can occur when interpreting bytecode.
#[derive(Debug)]
pub enum VirtualMachineError {
  UnexpectedValue,
}

impl VirtualMachine {
  /// Creates a new virtual machine.
  pub fn new() -> Self {
    Self {
      chunk: bytecode::Chunk::default(),
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

  /// Interprets the given chunk of bytecode.
  pub fn interpret(&mut self, chunk: bytecode::Chunk) -> Result<Variant, VirtualMachineError> {
    self.chunk = chunk;

    self.run()
  }

  /// Continues running the virtual machine, executing the bytecode.
  pub fn run(&mut self) -> Result<Variant, VirtualMachineError> {
    use bytecode::*;

    macro_rules! unary_op {
    ($operator:tt) => {
      if let Some(Value::Number(a)) = self.stack.pop() {
        self.stack.push(Value::Number($operator a));
      } else {
        return Err(VirtualMachineError::UnexpectedValue);
      }
    };
  }

    macro_rules! binary_op {
    ($operator:tt) => {
      if let (Some(Value::Number(a)), Some(Value::Number(b))) = (self.stack.pop(), self.stack.pop()) {
        self.stack.push(Value::Number(a $operator b));
      } else {
        return Err(VirtualMachineError::UnexpectedValue);
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
        Opcode::Return => {
          return match self.stack.pop() {
            Some(Value::Number(value)) => Ok(Variant::F64(value)),
            Some(Value::String(value)) => Ok(Variant::String(value)),
            Some(Value::Boolean(value)) => Ok(Variant::Bool(value)),
            Some(Value::Nil) => Ok(Variant::Null),
            None => Ok(Variant::Null),
          }
        }
        Opcode::Push(value) => self.stack.push(value),
        Opcode::Negate => unary_op!(-),
        Opcode::Add => binary_op!(+),
        Opcode::Subtract => binary_op!(-),
        Opcode::Multiply => binary_op!(*),
        Opcode::Divide => binary_op!(/),
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
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
  }

  /// A single instruction in the virtual machine.
  #[derive(Debug, Clone, PartialEq)]
  pub enum Opcode {
    Return,
    Push(Value),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
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
        Opcode::Return => "RETURN".to_string(),
        Opcode::Push(literal) => format!("PUSH {}", literal.disassemble()),
        Opcode::Negate => "NEGATE".to_string(),
        Opcode::Add => "ADD".to_string(),
        Opcode::Subtract => "SUBTRACT".to_string(),
        Opcode::Multiply => "MULTIPLY".to_string(),
        Opcode::Divide => "DIVIDE".to_string(),
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
}

mod compiler {
  use super::*;

  /// Represents an error that occurred while compiling a value.
  #[derive(Debug)]
  pub enum CompileError {}

  /// A compiler for the virtual machine.
  pub struct Compiler {
    chunk: bytecode::Chunk,
    line_number: usize,
    trace_compilation: bool,
  }

  impl Compiler {
    /// Creates a new compiler.
    pub fn new() -> Self {
      Self {
        chunk: bytecode::Chunk::default(),
        line_number: 0,
        trace_compilation: false,
      }
    }

    /// Enables tracing for the compiler.
    pub fn with_tracing(self) -> Self {
      Self {
        trace_compilation: true,
        ..self
      }
    }

    /// Compiles the given target into bytecode.
    pub fn compile(&mut self, target: &ast::Module) -> Result<(), CompileError> {
      target.accept(self);

      Ok(())
    }

    /// Pushes a new line onto the chunk.
    pub fn push_line(&mut self) {
      self.line_number += 1;
    }

    /// Pushes a new instruction onto the chunk.
    pub fn push(&mut self, line: usize, opcode: bytecode::Opcode) {
      if self.trace_compilation {
        println!("{:?}", opcode.disassemble());
      }

      self.chunk.code.push_back(bytecode::Instruction { line, opcode });
    }

    /// Finalizes the compiler, returning the compiled bytecode.
    pub fn finalize(self) -> bytecode::Chunk {
      self.chunk
    }
  }

  impl ast::Visitor for Compiler {
    fn visit_statement(&mut self, statement: &ast::Statement) {
      self.push_line();

      match statement {
        ast::Statement::Expression(expression) => self.visit_expression(expression),
        _ => {}
      }
    }

    fn visit_literal(&mut self, literal: &ast::Literal) {
      self.push(
        self.line_number,
        bytecode::Opcode::Push(match literal {
          ast::Literal::Number(value) => bytecode::Value::Number(*value),
          ast::Literal::String(value) => bytecode::Value::String(value.clone()),
          ast::Literal::Boolean(value) => bytecode::Value::Boolean(*value),
          ast::Literal::Nil => bytecode::Value::Nil,
        }),
      );
    }
  }
}

#[cfg(test)]
mod tests {
  use self::compiler::Compiler;
  use super::{bytecode::*, *};

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
  fn test_basic_disassembly_of_chunks() {
    let chunk = Chunk::from_slice(&[
      instruct!(1, Opcode::Push(Value::Number(1.0))),
      instruct!(1, Opcode::Push(Value::Number(3.14159))),
      instruct!(2, Opcode::Return),
    ]);

    println!("{}", chunk.disassemble());
  }

  #[test]
  fn test_basic_execution_of_chunks() {
    let mut vm = VirtualMachine::new().with_tracing();

    let chunk = Chunk::from_slice(&[
      instruct!(1, Opcode::Push(Value::Number(3.14159))),
      instruct!(1, Opcode::Push(Value::Number(1.28456))),
      instruct!(1, Opcode::Add),
      instruct!(2, Opcode::Return),
      instruct!(3, Opcode::Push(Value::Number(5.0))),
      instruct!(3, Opcode::Negate),
      instruct!(4, Opcode::Return),
    ]);

    let result = vm.interpret(chunk).expect("failed to interpret chunk");

    assert_eq!(result, Variant::F64(4.42615));

    let result = vm.run().expect("failed to run chunk");

    assert_eq!(result, Variant::F64(-5.0));
  }

  #[test]
  fn test_basic_compilation() {
    use ast::*;

    let mut compiler = Compiler::new();

    let module = Module {
      functions: vec![Function {
        name: "main".to_string(),
        statements: vec![Statement::Expression(Expression::Binary(
          BinaryOperator::Add,
          Box::new(Expression::Literal(Literal::Number(3.14159))),
          Box::new(Expression::Literal(Literal::Number(1.12345))),
        ))],
        parameters: vec![],
      }],
      imports: vec![],
    };

    compiler.compile(&module).expect("failed to compile module");

    let chunk = compiler.finalize();

    println!("{}", chunk.disassemble());
  }
}
