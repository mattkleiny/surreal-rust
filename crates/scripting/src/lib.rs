//! Scripting engine for Surreal

pub use callbacks::*;
pub use lang::*;
pub use values::*;

mod callbacks;
mod lang;
mod values;

/// An error that can occur during script execution.
#[derive(Debug)]
pub enum ScriptError {
  /// An error occurred during script execution.
  ExecutionError(String),
  /// An error occurred while converting a value.
  ConversionError(String),
}

/// Represents a runtime that allows script execution.
pub trait ScriptRuntime {
  /// Evaluates the given code and returns the result.
  fn eval(&self, code: &str) -> Result<ScriptValue, ScriptError>;

  /// Evaluates the given code and returns the result as the specified type.
  fn eval_as<R: FromScriptValue>(&self, code: &str) -> Result<R, ScriptError>;

  /// Adds a callback that can be called from scripts.
  fn add_callback<R>(&mut self, name: &str, callback: impl ScriptCallback<R> + 'static);
}
