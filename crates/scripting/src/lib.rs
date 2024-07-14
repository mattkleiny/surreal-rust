//! Scripting engine for Surreal

pub use interop::*;
pub use lang::*;

mod interop;
mod lang;

/// An error that can occur during script execution.
#[derive(Debug)]
pub enum ScriptError {
  /// An error occurred during script execution.
  ExecutionError(String),
  /// An error occurred while converting a value.
  ConversionError(String),
}

/// A callback that can be called from a script.
pub trait ScriptCallback<R: FromScriptValue> {
  /// Returns the number of arguments the callback expects.
  fn argument_count(&self) -> usize;

  /// Calls the callback with the given arguments.
  fn call(&self, args: &[ScriptValue]) -> Result<R, ScriptError>;
}

/// Represents a runtime that allows script execution.
pub trait ScriptRuntime {
  /// Evaluates the given code and returns the result.
  fn eval(&self, code: &str) -> Result<ScriptValue, ScriptError>;

  /// Evaluates the given code and returns the result as the specified type.
  fn eval_as<R: FromScriptValue>(&self, code: &str) -> Result<R, ScriptError>;

  /// Adds a callback that can be called from scripts.
  fn add_callback<R: ToScriptValue + FromScriptValue>(
    &mut self,
    name: &str,
    callback: impl ScriptCallback<R> + 'static,
  ) -> Result<(), ScriptError>;
}
