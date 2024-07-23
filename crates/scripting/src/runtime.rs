//! The runtime for the scripting system.

/// A virtual machine for script execution, capable of interpreting bytecode.
pub struct VirtualMachine {}

/// An operation code for the virtual machine.
#[repr(C)]
#[derive(Debug)]
pub enum OpCode {
  NoOp,
}
