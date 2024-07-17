use std::{marker::PhantomData, panic::RefUnwindSafe};

use super::*;

/// A callback that can be called from a script.
pub trait ScriptCallback<R>: RefUnwindSafe {
  /// Calls the callback with the given arguments.
  fn call(&self, args: &[ScriptValue]) -> Result<ScriptValue, ScriptError>;
}

// Blanket callback implementation for Rust functions and closures.
//
// All implementations follow a pattern of converting the arguments to the
// expected types, calling the function, and converting the result back to a
// `ScriptValue`.
//
// We use a PhantomData tuple to specify the expected argument types and
// constrain unique implementations of the generic type.

impl<R, F> ScriptCallback<PhantomData<()>> for F
where
  R: ToScriptValue,
  F: Fn() -> R + RefUnwindSafe,
{
  fn call(&self, _args: &[ScriptValue]) -> Result<ScriptValue, ScriptError> {
    let result = self();

    Ok(result.to_script_value())
  }
}

impl<A1, R, F> ScriptCallback<(PhantomData<A1>, R)> for F
where
  A1: FromScriptValue,
  R: ToScriptValue,
  F: Fn(A1) -> R + RefUnwindSafe,
{
  fn call(&self, args: &[ScriptValue]) -> Result<ScriptValue, ScriptError> {
    if args.len() != 1 {
      return Err(ScriptError::ExecutionError(format!(
        "Invalid argument count: Expected 1, got {}",
        args.len()
      )));
    }

    let arg1 = A1::from_script_value(&args[0]);

    let result = self(arg1);

    Ok(result.to_script_value())
  }
}

impl<A1, A2, R, F> ScriptCallback<(PhantomData<A1>, PhantomData<A2>, R)> for F
where
  A1: FromScriptValue,
  A2: FromScriptValue,
  R: ToScriptValue,
  F: Fn(A1, A2) -> R + RefUnwindSafe,
{
  fn call(&self, args: &[ScriptValue]) -> Result<ScriptValue, ScriptError> {
    if args.len() != 2 {
      return Err(ScriptError::ExecutionError(format!(
        "Invalid argument count: Expected 2, got {}",
        args.len()
      )));
    }

    let arg1 = A1::from_script_value(&args[0]);
    let arg2 = A2::from_script_value(&args[1]);

    let result = self(arg1, arg2);

    Ok(result.to_script_value())
  }
}

impl<A1, A2, A3, R, F> ScriptCallback<(PhantomData<A1>, PhantomData<A2>, PhantomData<A3>, R)> for F
where
  A1: FromScriptValue,
  A2: FromScriptValue,
  A3: FromScriptValue,
  R: ToScriptValue,
  F: Fn(A1, A2, A3) -> R + RefUnwindSafe,
{
  fn call(&self, args: &[ScriptValue]) -> Result<ScriptValue, ScriptError> {
    if args.len() != 3 {
      return Err(ScriptError::ExecutionError(format!(
        "Invalid argument count: Expected 3, got {}",
        args.len()
      )));
    }

    let arg1 = A1::from_script_value(&args[0]);
    let arg2 = A2::from_script_value(&args[1]);
    let arg3 = A3::from_script_value(&args[2]);

    let result = self(arg1, arg2, arg3);

    Ok(result.to_script_value())
  }
}

impl<A1, A2, A3, A4, R, F> ScriptCallback<(PhantomData<A1>, PhantomData<A2>, PhantomData<A3>, PhantomData<A4>, R)> for F
where
  A1: FromScriptValue,
  A2: FromScriptValue,
  A3: FromScriptValue,
  A4: FromScriptValue,
  R: ToScriptValue,
  F: Fn(A1, A2, A3, A4) -> R + RefUnwindSafe,
{
  fn call(&self, args: &[ScriptValue]) -> Result<ScriptValue, ScriptError> {
    if args.len() != 4 {
      return Err(ScriptError::ExecutionError(format!(
        "Invalid argument count: Expected 4, got {}",
        args.len()
      )));
    }

    let arg1 = A1::from_script_value(&args[0]);
    let arg2 = A2::from_script_value(&args[1]);
    let arg3 = A3::from_script_value(&args[2]);
    let arg4 = A4::from_script_value(&args[3]);

    let result = self(arg1, arg2, arg3, arg4);

    Ok(result.to_script_value())
  }
}
