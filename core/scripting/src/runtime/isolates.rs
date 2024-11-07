use crate::runtime::machine::{VirtualMachine, VirtualMachineConfig};

/// An isolate is a separate instance of the scripting runtime.
///
/// Each isolate has its own context, which includes its own global object,
/// built-in objects, and execution stack. There is no shared state between
/// isolates, and they cannot directly interact with each other.
///
/// Isolates are used to run scripts in parallel, and can be used to sandbox
/// untrusted code. Consider Isolates as the main entry point for running
/// scripts.
#[derive(Default)]
pub struct Isolate {
  _virtual_machine: VirtualMachine,
}

/// Configuration for creating a new isolate.
#[derive(Default, Debug)]
pub struct IsolateConfig {
  _virtual_machine: VirtualMachineConfig,
  _stack_size: usize,
  _thread_count: usize,
  _memory_limit: usize,
}

impl Isolate {
  /// Creates a new isolate.
  pub fn new(config: IsolateConfig) -> Self {
    Self {
      _virtual_machine: VirtualMachine::new(config._virtual_machine),
    }
  }
}
