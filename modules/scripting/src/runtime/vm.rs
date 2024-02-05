use std::collections::VecDeque;

use common::Variant;

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
  UnexpectedValue,
}

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

  /// Interprets the given chunk of bytecode.
  pub fn interpret(&mut self, chunk: Chunk) -> Result<Variant, VirtualMachineError> {
    self.chunk = chunk;

    self.run()
  }

  /// Continues running the virtual machine, executing the bytecode.
  pub fn run(&mut self) -> Result<Variant, VirtualMachineError> {
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

#[cfg(test)]
mod tests {
  use super::*;

  /// Helper macro to create an instruction.
  macro_rules! instruct {
    ($line:literal, $opcode:expr) => {
      Instruction {
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
}
