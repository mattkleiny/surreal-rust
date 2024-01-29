use super::ScriptRuntime;

/// Allows interpreting scripts.
///
/// This is a simple tree-walk interpreter.
#[derive(Default)]
pub struct Interpreter {}

impl ScriptRuntime for Interpreter {
  fn call_function(
    &mut self,
    _name: impl AsRef<str>,
    _parameters: &[common::Variant],
  ) -> Result<Vec<common::Variant>, super::ScriptExecuteError> {
    todo!()
  }
}
