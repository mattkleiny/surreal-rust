//! Scripting engine for Surreal

pub use callbacks::*;
pub use lang::*;
pub use values::*;

mod callbacks;
mod lang;
mod values;

use common::ToVirtualPath;

/// An error that can occur during script execution.
#[derive(Debug)]
pub enum ScriptError {
  ExecutionError(String),
  ConversionError(String),
}

/// Represents a runtime that allows script execution.
pub trait ScriptRuntime {
  /// Evaluates the given code and returns the result.
  fn eval(&self, code: &str) -> Result<ScriptValue, ScriptError>;

  /// Evaluates the given code and returns the result as the specified type.
  fn eval_as<R: FromScriptValue>(&self, code: &str) -> Result<R, ScriptError> {
    self.eval(code).map(|it| R::from_script_value(&it))
  }

  /// Loads a script from the given path.
  fn load(&self, path: impl ToVirtualPath) -> Result<ScriptValue, ScriptError> {
    let path = path.to_virtual_path();
    let script = path
      .read_all_text()
      .map_err(|_| ScriptError::ExecutionError("Failed to load script".to_string()))?;

    self.eval(&script)
  }

  /// Adds a callback that can be called from scripts.
  fn add_callback<F>(&mut self, name: &str, callback: impl ScriptCallback<F> + 'static);
}
