//! The virtual machine backend for the scripting system.
//!
//! This virtual machine is a stack-based bytecode interpreter with support
//! for the superset of all languages that might be used in the scripting system.

use std::fmt::Debug;

/// A virtual machine that can execute `BytecodeChunk`s.
#[derive(Default)]
pub struct VirtualMachine {}

impl VirtualMachine {
  /// Creates a new virtual machine.
  pub fn new() -> Self {
    Self {}
  }

  /// Interprets the given chunk of bytecode.
  pub fn execute(&mut self, program: &BytecodeChunk) -> crate::Result<()> {
    let mut program_counter = 0;
    let mut stack = Vec::with_capacity(256);

    while program_counter < program.len() {
      if let Some((opcode, _)) = program.get(program_counter) {
        program_counter += 1;

        // debug printing
        println!("{:#06x}", program_counter);
        println!("{:?}", stack);

        match opcode {
          OpCode::Return => {
            stack.pop();
            return Ok(());
          }
          OpCode::Constant(value) => {
            stack.push(*value);
          }
          OpCode::Negate => {
            let value = stack.pop().unwrap();
            stack.push(-value);
          }
          OpCode::Add => {
            let right = stack.pop().unwrap();
            let left = stack.pop().unwrap();

            stack.push(left + right);
          }
          OpCode::Subtract => {
            let right = stack.pop().unwrap();
            let left = stack.pop().unwrap();

            stack.push(left - right);
          }
          OpCode::Multiply => {
            let right = stack.pop().unwrap();
            let left = stack.pop().unwrap();

            stack.push(left * right);
          }
          OpCode::Divide => {
            let right = stack.pop().unwrap();
            let left = stack.pop().unwrap();

            stack.push(left / right);
          }
        }
      } else {
        break;
      }
    }

    Err(anyhow::anyhow!("An unexpected end of termination occurred"))
  }
}

/// Represents a line:column position in a source file.
#[derive(Copy, Clone)]
pub struct TokenPos {
  pub line: u16,
  pub column: u16,
}

impl Debug for TokenPos {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", self.line, self.column)
  }
}

/// Represents a chunk of bytecode that can be executed in the `VirtualMachine`.
#[derive(Default)]
pub struct BytecodeChunk {
  opcodes: Vec<(OpCode, TokenPos)>,
}

impl BytecodeChunk {
  /// Creates an empty chunk.
  pub fn new() -> Self {
    Self {
      opcodes: Vec::new(),
    }
  }

  /// Is the chunk empty?
  pub fn is_empty(&self) -> bool {
    self.opcodes.is_empty()
  }

  /// The number of opcodes in the chunk.
  pub fn len(&self) -> usize {
    self.opcodes.len()
  }

  /// Borrows the opcode at the given index.
  pub fn get(&self, index: usize) -> Option<&(OpCode, TokenPos)> {
    self.opcodes.get(index)
  }

  /// Pushes a new opcode onto the chunk.
  pub fn push(&mut self, opcode: OpCode, position: TokenPos) {
    self.opcodes.push((opcode, position));
  }
}

impl Debug for BytecodeChunk {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (index, (opcode, pos)) in self.opcodes.iter().enumerate() {
      writeln!(
        formatter,
        "{:#06x} {:?} at ({:}:{:})",
        index, opcode, pos.line, pos.column
      )?;
    }

    Ok(())
  }
}

/// Represents a single opcode in a bytecode program.
#[derive(Debug)]
pub enum OpCode {
  // variables and control flow
  Return,
  Constant(f64),

  // basic math
  Negate,
  Add,
  Subtract,
  Multiply,
  Divide,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn virtual_machine_should_execute_simple_bytecode_chunk() {
    let mut chunk = BytecodeChunk::new();
    let position = TokenPos { line: 1, column: 1 };

    chunk.push(OpCode::Constant(2.), position);
    chunk.push(OpCode::Negate, position);
    chunk.push(OpCode::Constant(3.), position);
    chunk.push(OpCode::Multiply, position);
    chunk.push(OpCode::Return, position);

    let mut vm = VirtualMachine::new();

    vm.execute(&chunk)
      .expect("Failed to execute simple program");
  }
}
