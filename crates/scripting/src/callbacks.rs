use std::{marker::PhantomData, panic::RefUnwindSafe};

use common::{FromVariant, ToVariant, Variant};

/// An error when calling a script callback.
pub enum CallbackError {
  ExecutionError(String),
}

/// A callback that can be called from a script.
pub trait ScriptCallback<R>: RefUnwindSafe {
  /// Calls the callback with the given arguments.
  fn call(&self, args: &[Variant]) -> Result<Variant, CallbackError>;
}

// Blanket callback implementation for Rust functions and closures.
//
// All implementations follow a pattern of converting the arguments to the
// expected types, calling the function, and converting the result back to a
// `Variant`.
//
// We use a PhantomData tuple to specify the expected argument types and
// constrain unique implementations of the generic type.

impl<R, F> ScriptCallback<PhantomData<()>> for F
where
  R: ToVariant,
  F: Fn() -> R + RefUnwindSafe,
{
  fn call(&self, _args: &[Variant]) -> Result<Variant, CallbackError> {
    let result = self();

    Ok(result.to_variant())
  }
}

impl<A1, R, F> ScriptCallback<(PhantomData<A1>, R)> for F
where
  A1: FromVariant,
  R: ToVariant,
  F: Fn(A1) -> R + RefUnwindSafe,
{
  fn call(&self, args: &[Variant]) -> Result<Variant, CallbackError> {
    if args.len() != 1 {
      return Err(CallbackError::ExecutionError(format!(
        "Invalid argument count: Expected 1, got {}",
        args.len()
      )));
    }

    let arg1 = A1::from_variant(args[0].clone());

    let result = self(arg1);

    Ok(result.to_variant())
  }
}

impl<A1, A2, R, F> ScriptCallback<(PhantomData<A1>, PhantomData<A2>, R)> for F
where
  A1: FromVariant,
  A2: FromVariant,
  R: ToVariant,
  F: Fn(A1, A2) -> R + RefUnwindSafe,
{
  fn call(&self, args: &[Variant]) -> Result<Variant, CallbackError> {
    if args.len() != 2 {
      return Err(CallbackError::ExecutionError(format!(
        "Invalid argument count: Expected 2, got {}",
        args.len()
      )));
    }

    let arg1 = A1::from_variant(args[0].clone());
    let arg2 = A2::from_variant(args[1].clone());

    let result = self(arg1, arg2);

    Ok(result.to_variant())
  }
}

impl<A1, A2, A3, R, F> ScriptCallback<(PhantomData<A1>, PhantomData<A2>, PhantomData<A3>, R)> for F
where
  A1: FromVariant,
  A2: FromVariant,
  A3: FromVariant,
  R: ToVariant,
  F: Fn(A1, A2, A3) -> R + RefUnwindSafe,
{
  fn call(&self, args: &[Variant]) -> Result<Variant, CallbackError> {
    if args.len() != 3 {
      return Err(CallbackError::ExecutionError(format!(
        "Invalid argument count: Expected 3, got {}",
        args.len()
      )));
    }

    let arg1 = A1::from_variant(args[0].clone());
    let arg2 = A2::from_variant(args[1].clone());
    let arg3 = A3::from_variant(args[2].clone());

    let result = self(arg1, arg2, arg3);

    Ok(result.to_variant())
  }
}

impl<A1, A2, A3, A4, R, F> ScriptCallback<(PhantomData<A1>, PhantomData<A2>, PhantomData<A3>, PhantomData<A4>, R)> for F
where
  A1: FromVariant,
  A2: FromVariant,
  A3: FromVariant,
  A4: FromVariant,
  R: ToVariant,
  F: Fn(A1, A2, A3, A4) -> R + RefUnwindSafe,
{
  fn call(&self, args: &[Variant]) -> Result<Variant, CallbackError> {
    if args.len() != 4 {
      return Err(CallbackError::ExecutionError(format!(
        "Invalid argument count: Expected 4, got {}",
        args.len()
      )));
    }

    let arg1 = A1::from_variant(args[0].clone());
    let arg2 = A2::from_variant(args[1].clone());
    let arg3 = A3::from_variant(args[2].clone());
    let arg4 = A4::from_variant(args[3].clone());

    let result = self(arg1, arg2, arg3, arg4);

    Ok(result.to_variant())
  }
}
