use common::Variant;

use crate::ast::{BinaryOp, UnaryOp};

/// A possible error that can occur during virtual machine execution.
#[derive(Debug)]
pub enum VirtualMachineError {
  InvalidModule(String),
  InvalidInstruction,
  InvalidConstantIndex(TableIndex),
  InvalidValueIndex(TableIndex),
  StackOverflow,
  StackUnderflow,
  CallStackOverflow,
}

/// Configuration for the [`VirtualMachine`].
pub struct VirtualMachineConfig {
  pub max_stack_size: usize,
}

impl Default for VirtualMachineConfig {
  fn default() -> Self {
    Self { max_stack_size: 256 }
  }
}

/// A bytecode-interpreting Virtual Machine.
#[derive(Default)]
pub struct VirtualMachine {
  stack: Vec<Variant>,
  constants: Table<Variant>,
  locals: Table<Variant>,
  config: VirtualMachineConfig,
}

/// A bytecode instruction for the virtual machine.
#[derive(Debug)]
pub enum Instruction {
  NoOp,
  Return,
  Constant(TableIndex),
  Local(TableIndex),
  Unary(UnaryOp),
  Binary(BinaryOp),
  Print,
}

/// An index into a [`Table`].
type TableIndex = u16;

/// A table of [`V`] for the virtual machine.
#[repr(transparent)]
struct Table<V> {
  values: Vec<V>,
}

impl VirtualMachine {
  /// Creates a new virtual machine with the given configuration.
  pub fn new(config: VirtualMachineConfig) -> Self {
    VirtualMachine {
      stack: Vec::with_capacity(config.max_stack_size),
      constants: Table::default(),
      locals: Table::default(),
      config,
    }
  }

  /// Pushes a value onto the stack.
  pub fn push(&mut self, value: Variant) -> Result<(), VirtualMachineError> {
    if self.stack.len() >= self.config.max_stack_size {
      return Err(VirtualMachineError::StackOverflow);
    }

    self.stack.push(value);
    Ok(())
  }

  /// Pops a value from the stack.
  pub fn pop(&mut self) -> Result<Variant, VirtualMachineError> {
    self.stack.pop().ok_or(VirtualMachineError::StackUnderflow)
  }

  /// Executes the given [`Instruction`]s.
  pub fn execute(&mut self, instructions: &[Instruction]) -> Result<Option<Variant>, VirtualMachineError> {
    for instruction in instructions {
      if let Some(result) = self.interpret(instruction)? {
        return Ok(Some(result));
      }
    }

    Ok(None)
  }

  /// Interpret the given [`Instruction`].
  fn interpret(&mut self, instruction: &Instruction) -> Result<Option<Variant>, VirtualMachineError> {
    match instruction {
      Instruction::NoOp => {}
      Instruction::Return => {
        if let Ok(value) = self.pop() {
          return Ok(Some(value));
        }

        return Ok(None);
      }
      Instruction::Constant(index) => {
        let constant = self.get_constant(*index)?;

        self.push(constant.clone())?;
      }
      Instruction::Local(index) => {
        let value = self.get_local(*index)?;

        self.push(value.clone())?;
      }
      Instruction::Unary(operator) => match operator {
        UnaryOp::Negate => {
          let value = self.pop()?;

          let result = value.neg().map_err(|_| VirtualMachineError::InvalidInstruction)?;

          self.push(result)?;
        }
      },
      Instruction::Binary(operator) => match operator {
        BinaryOp::Add => {
          let a = self.pop()?;
          let b = self.pop()?;

          let result = a.add(&b).map_err(|_| VirtualMachineError::InvalidInstruction)?;

          self.push(result)?;
        }
        BinaryOp::Subtract => {
          let a = self.pop()?;
          let b = self.pop()?;

          let result = a.sub(&b).map_err(|_| VirtualMachineError::InvalidInstruction)?;

          self.push(result)?;
        }
        BinaryOp::Multiply => {
          let a = self.pop()?;
          let b = self.pop()?;

          let result = a.mul(&b).map_err(|_| VirtualMachineError::InvalidInstruction)?;

          self.push(result)?;
        }
        BinaryOp::Divide => {
          let a = self.pop()?;
          let b = self.pop()?;

          let result = a.div(&b).map_err(|_| VirtualMachineError::InvalidInstruction)?;

          self.push(result)?;
        }
        BinaryOp::Modulo => todo!(),
        BinaryOp::Equal => todo!(),
        BinaryOp::NotEqual => todo!(),
        BinaryOp::LessThan => todo!(),
        BinaryOp::LessThanOrEqual => todo!(),
        BinaryOp::GreaterThan => todo!(),
        BinaryOp::GreaterThanOrEqual => todo!(),
        BinaryOp::And => todo!(),
        BinaryOp::Or => todo!(),
      },
      Instruction::Print => {
        let value = self.pop()?;

        println!("{:?}", value);
      }
    }

    Ok(None)
  }

  /// Gets the constant value at the given index.
  fn get_constant(&self, index: TableIndex) -> Result<&Variant, VirtualMachineError> {
    let value = self.constants.get(index);

    value.ok_or(VirtualMachineError::InvalidConstantIndex(index))
  }

  /// Gets the local value at the given index.
  fn get_local(&self, index: TableIndex) -> Result<&Variant, VirtualMachineError> {
    let value = self.locals.get(index);

    value.ok_or(VirtualMachineError::InvalidValueIndex(index))
  }
}

impl<V> Default for Table<V> {
  fn default() -> Self {
    Self { values: Vec::new() }
  }
}

impl<V> Table<V> {
  /// Adds a value to the table and returns its index.
  pub fn add(&mut self, value: V) -> TableIndex {
    let index = self.values.len();

    self.values.push(value);

    index as TableIndex
  }

  /// Gets a value from the table at the given index.
  pub fn get(&self, index: TableIndex) -> Option<&V> {
    if index as usize >= self.values.len() {
      return None;
    }

    self.values.get(index as usize)
  }

  /// Sets a value in the table at the given index.
  pub fn set(&mut self, index: TableIndex, value: V) {
    if index as usize >= self.values.len() {
      return;
    }

    self.values[index as usize] = value;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_execute_simple_instructions() {
    let mut vm = VirtualMachine::default();

    let instructions = [
      Instruction::Constant(vm.constants.add(Variant::from(42))),
      Instruction::Unary(UnaryOp::Negate),
      Instruction::Local(vm.locals.add(Variant::from(42))),
      Instruction::Binary(BinaryOp::Add),
      Instruction::Return,
      Instruction::Print,
    ];

    let result = vm.execute(&instructions).unwrap().unwrap();

    assert_eq!(result, Variant::from(0));
  }
}
