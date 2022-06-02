//! The virtual machine backend for the scripting system.
//!
//! This virtual machine is a stack-based bytecode interpreter with support
//! for the superset of all languages that might be used in the scripting system.

/// A virtual machine that can execute `BytecodeProgram`s.
pub struct VirtualMachine {}

/// Represents a bytecode program that can be executed in the virtual machine.
pub struct BytecodeProgram {
  opcodes: Vec<Opcode>,
}

/// Represents a single opcode in a bytecode program.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Opcode {
  NOP,
}
