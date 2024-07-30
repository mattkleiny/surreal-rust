use std::{marker::PhantomData, panic::RefUnwindSafe};

use crate::runtime::{FromScriptValue, ScriptValue, ToScriptValue};

/// An error when calling a script callback.
#[derive(Debug)]
pub enum CallbackError {
  ExecutionError(String),
}

/// A callback that can be called from a script.
pub trait Callback<R>: RefUnwindSafe {
  /// Calls the callback with the given arguments.
  fn call(&self, args: &[ScriptValue]) -> Result<ScriptValue, CallbackError>;
}

// Blanket callback implementation for Rust functions and closures.
//
// All implementations follow a pattern of converting the arguments to the
// expected types, calling the function, and converting the result back to a
// `ScriptValue`.
//
// We use a PhantomData tuple to specify the expected argument types and
// constrain unique implementations of the generic type.

impl<R, F> Callback<PhantomData<()>> for F
where
  R: ToScriptValue,
  F: Fn() -> R + RefUnwindSafe,
{
  fn call(&self, _args: &[ScriptValue]) -> Result<ScriptValue, CallbackError> {
    let result = self();

    Ok(result.to_script_value())
  }
}

impl<A1, R, F> Callback<(PhantomData<A1>, R)> for F
where
  A1: FromScriptValue,
  R: ToScriptValue,
  F: Fn(A1) -> R + RefUnwindSafe,
{
  fn call(&self, args: &[ScriptValue]) -> Result<ScriptValue, CallbackError> {
    if args.len() != 1 {
      return Err(CallbackError::ExecutionError(format!(
        "Invalid argument count: Expected 1, got {}",
        args.len()
      )));
    }

    let arg1 = A1::from_script_value(&args[0].clone());

    let result = self(arg1);

    Ok(result.to_script_value())
  }
}

impl<A1, A2, R, F> Callback<(PhantomData<A1>, PhantomData<A2>, R)> for F
where
  A1: FromScriptValue,
  A2: FromScriptValue,
  R: ToScriptValue,
  F: Fn(A1, A2) -> R + RefUnwindSafe,
{
  fn call(&self, args: &[ScriptValue]) -> Result<ScriptValue, CallbackError> {
    if args.len() != 2 {
      return Err(CallbackError::ExecutionError(format!(
        "Invalid argument count: Expected 2, got {}",
        args.len()
      )));
    }

    let arg1 = A1::from_script_value(&args[0].clone());
    let arg2 = A2::from_script_value(&args[1].clone());

    let result = self(arg1, arg2);

    Ok(result.to_script_value())
  }
}

impl<A1, A2, A3, R, F> Callback<(PhantomData<A1>, PhantomData<A2>, PhantomData<A3>, R)> for F
where
  A1: FromScriptValue,
  A2: FromScriptValue,
  A3: FromScriptValue,
  R: ToScriptValue,
  F: Fn(A1, A2, A3) -> R + RefUnwindSafe,
{
  fn call(&self, args: &[ScriptValue]) -> Result<ScriptValue, CallbackError> {
    if args.len() != 3 {
      return Err(CallbackError::ExecutionError(format!(
        "Invalid argument count: Expected 3, got {}",
        args.len()
      )));
    }

    let arg1 = A1::from_script_value(&args[0].clone());
    let arg2 = A2::from_script_value(&args[1].clone());
    let arg3 = A3::from_script_value(&args[2].clone());

    let result = self(arg1, arg2, arg3);

    Ok(result.to_script_value())
  }
}

impl<A1, A2, A3, A4, R, F> Callback<(PhantomData<A1>, PhantomData<A2>, PhantomData<A3>, PhantomData<A4>, R)> for F
where
  A1: FromScriptValue,
  A2: FromScriptValue,
  A3: FromScriptValue,
  A4: FromScriptValue,
  R: ToScriptValue,
  F: Fn(A1, A2, A3, A4) -> R + RefUnwindSafe,
{
  fn call(&self, args: &[ScriptValue]) -> Result<ScriptValue, CallbackError> {
    if args.len() != 4 {
      return Err(CallbackError::ExecutionError(format!(
        "Invalid argument count: Expected 4, got {}",
        args.len()
      )));
    }

    let arg1 = A1::from_script_value(&args[0].clone());
    let arg2 = A2::from_script_value(&args[1].clone());
    let arg3 = A3::from_script_value(&args[2].clone());
    let arg4 = A4::from_script_value(&args[3].clone());

    let result = self(arg1, arg2, arg3, arg4);

    Ok(result.to_script_value())
  }
}
