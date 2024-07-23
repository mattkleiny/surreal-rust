use common::Variant;

use crate::Callback;

/// A possible error that can occur during virtual machine execution.
#[derive(Debug)]
pub enum VirtualMachineError {
  InvalidOpCode(OpCode),
  InvalidModule(String),
}

/// Configuration for the [`VirtualMachine`].
pub struct VirtualMachineConfig {
  pub max_stack_size: usize,
  pub max_call_stack_size: usize,
}

impl Default for VirtualMachineConfig {
  fn default() -> Self {
    Self {
      max_stack_size: 1024,
      max_call_stack_size: 1024,
    }
  }
}

/// A bytecode-interpreting Virtual Machine.
#[derive(Default)]
pub struct VirtualMachine {
  stack: Vec<Variant>,
  call_stack: Vec<Variant>,
  config: VirtualMachineConfig,
}

impl VirtualMachine {
  /// Creates a new virtual machine with the given configuration.
  pub fn new(config: VirtualMachineConfig) -> Self {
    VirtualMachine {
      stack: Vec::with_capacity(config.max_stack_size),
      call_stack: Vec::with_capacity(config.max_call_stack_size),
      config,
    }
  }

  /// Adds a callback to the virtual machine with the given name.
  pub fn add_callback<R>(&mut self, _name: &str, _callback: impl Callback<R>) {
    todo!()
  }

  /// Evaluates the given module with the provided arguments.
  pub fn evaluate(&mut self, _module: &str, _arguments: &[Variant]) -> Result<Option<Variant>, VirtualMachineError> {
    todo!()
  }
}

/// An operation code for the virtual machine.
#[repr(C)]
#[derive(Debug)]
pub enum OpCode {
  NoOp,
}
