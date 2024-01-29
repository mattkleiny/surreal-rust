use common::{StringName, Variant};

use super::ScriptRuntime;

/// Allows interpreting scripts.
///
/// This is a simple tree-walk interpreter.
#[derive(Default)]
pub struct Interpreter {
  call_stack: Vec<StackFrame>,
}

/// A single frame in the call stack.
#[derive(Debug)]
struct StackFrame {
  locals: Vec<StackLocal>,
}

/// A local variable in a stack frame.
#[derive(Debug)]
struct StackLocal {
  name: StringName,
  value: Variant,
}

impl ScriptRuntime for Interpreter {
  fn call_function(
    &mut self,
    _name: impl AsRef<str>,
    _parameters: &[common::Variant],
  ) -> Result<Vec<common::Variant>, super::ScriptExecuteError> {
    todo!()
  }
}
