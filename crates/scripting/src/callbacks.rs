use std::{marker::PhantomData, panic::RefUnwindSafe};

use common::Variant;

use super::*;

/// A callback that can be called from a script.
pub trait ScriptCallback<R>: RefUnwindSafe {
  /// Calls the callback with the given arguments.
  fn call(&self, args: &[ScriptValue]) -> Result<ScriptValue, ScriptError>;
}

/// Implements the `ScriptCallback` trait for functions with up to 5 arguments.
macro_rules! impl_callback {
  (@call $len:literal $self:ident $args:ident ) => {
    $self()
  };

  (@call $len:literal $self:ident $args:ident $( $arg:ident ),* ) => {
    {
      let mut iter = $args.into_iter();

      $self($($arg::from_script_value(iter.next().unwrap()),)*)
    }
  };

  [ $(  $len:literal : ( $( $arg:ident, )* ), )* ] => {
    $(
      impl<
          $( $arg, )*
          R,
          F,
      > ScriptCallback<std::marker::PhantomData<(
          $( &$arg, )*
          &R,
          &F,
      )>> for F
      where
          $( $arg: FromScriptValue, )*
          R: ToScriptValue,
          F: Fn( $( $arg, )*  ) -> R + RefUnwindSafe,
      {
        fn call(&self, args: &[ScriptValue]) -> Result<ScriptValue, ScriptError> {
          if args.len() != $len {
            return Err(ScriptError::ExecutionError(format!("Invalid argument count: Expected {}, got {}", $len, args.len())));
          }

          // recursive
          let result = impl_callback!(@call $len self args $($arg),* );

          Ok(result.to_script_value())
        }
      }
    )*
  };
}

impl_callback![
    0: (),
    1: (A1,),
    2: (A1, A2,),
    3: (A1, A2, A3,),
    4: (A1, A2, A3, A4,),
    5: (A1, A2, A3, A4, A5,),
];

/// Allows calling a function with no arguments.
impl<F> ScriptCallback<PhantomData<&F>> for F
where
  F: Fn() + RefUnwindSafe,
{
  fn call(&self, _args: &[ScriptValue]) -> Result<ScriptValue, ScriptError> {
    self();

    Ok(ScriptValue::from(Variant::Null))
  }
}
