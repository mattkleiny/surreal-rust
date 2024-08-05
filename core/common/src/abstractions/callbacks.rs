use std::{
  fmt::{Debug, Display, Formatter},
  marker::PhantomData,
  panic::RefUnwindSafe,
  sync::Arc,
};

use crate::{FromVariant, ToVariant, Variant};

/// An error when calling a script callback.
#[derive(Debug)]
pub enum CallbackError {
  ExecutionError(String),
  InvalidArgument,
}

impl Display for CallbackError {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      CallbackError::ExecutionError(message) => {
        write!(formatter, "Execution error: {}", message)
      }
      CallbackError::InvalidArgument => {
        write!(formatter, "Invalid argument")
      }
    }
  }
}

/// A boxed callable function.
///
/// This is a wrapper around a boxed function that can be called with a list of
/// [`Variant`] arguments and returns a [`Variant`] result.
#[derive(Clone)]
pub struct Callable(Arc<dyn Fn(&[Variant]) -> Result<Variant, CallbackError>>);

impl Callable {
  /// Creates a new boxed callable function from the given function.
  pub fn from_function(function: impl Fn(&[Variant]) -> Result<Variant, CallbackError> + 'static) -> Self {
    Self(Arc::new(function))
  }

  /// Creates a new boxed callable function from the given [`Callback`].
  pub fn from_callback<R>(callback: impl Callback<R> + 'static) -> Self {
    Self(Arc::new(move |args| callback.call(args)))
  }

  /// Calls the boxed callable function with the given arguments.
  pub fn call(&self, args: &[Variant]) -> Result<Variant, CallbackError> {
    let callable = self.0.as_ref();

    callable(args)
  }
}

impl PartialEq for Callable {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    Arc::ptr_eq(&self.0, &other.0)
  }
}

impl Debug for Callable {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "Callable")
  }
}

/// Represents a function signature that is callable.
pub trait Callback<R>: RefUnwindSafe {
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

impl<R, F> Callback<PhantomData<()>> for F
where
  R: ToVariant,
  F: Fn() -> R + RefUnwindSafe,
{
  fn call(&self, _args: &[Variant]) -> Result<Variant, CallbackError> {
    let result = self();

    Ok(result.to_variant())
  }
}

impl<A1, R, F> Callback<(PhantomData<A1>, R)> for F
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

    let arg1 = A1::from_variant(args[0].clone()).map_err(|_| CallbackError::InvalidArgument)?;

    let result = self(arg1);

    Ok(result.to_variant())
  }
}

impl<A1, A2, R, F> Callback<(PhantomData<A1>, PhantomData<A2>, R)> for F
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

    let arg1 = A1::from_variant(args[0].clone()).map_err(|_| CallbackError::InvalidArgument)?;
    let arg2 = A2::from_variant(args[1].clone()).map_err(|_| CallbackError::InvalidArgument)?;

    let result = self(arg1, arg2);

    Ok(result.to_variant())
  }
}

impl<A1, A2, A3, R, F> Callback<(PhantomData<A1>, PhantomData<A2>, PhantomData<A3>, R)> for F
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

    let arg1 = A1::from_variant(args[0].clone()).map_err(|_| CallbackError::InvalidArgument)?;
    let arg2 = A2::from_variant(args[1].clone()).map_err(|_| CallbackError::InvalidArgument)?;
    let arg3 = A3::from_variant(args[2].clone()).map_err(|_| CallbackError::InvalidArgument)?;

    let result = self(arg1, arg2, arg3);

    Ok(result.to_variant())
  }
}

impl<A1, A2, A3, A4, R, F> Callback<(PhantomData<A1>, PhantomData<A2>, PhantomData<A3>, PhantomData<A4>, R)> for F
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

    let arg1 = A1::from_variant(args[0].clone()).map_err(|_| CallbackError::InvalidArgument)?;
    let arg2 = A2::from_variant(args[1].clone()).map_err(|_| CallbackError::InvalidArgument)?;
    let arg3 = A3::from_variant(args[2].clone()).map_err(|_| CallbackError::InvalidArgument)?;
    let arg4 = A4::from_variant(args[3].clone()).map_err(|_| CallbackError::InvalidArgument)?;

    let result = self(arg1, arg2, arg3, arg4);

    Ok(result.to_variant())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_callable_function_creation_and_execution() {
    let callable = Callable::from_callback(|a: u32, b: u32| a + b);
    let result = callable.call(&[Variant::U32(1), Variant::U32(2)]).unwrap();

    assert_eq!(result, Variant::U32(3));
  }
}
