use std::any::Any;

/// Represents an 'Object'.
///
/// Objects are poly-morphically cast-able at runtime and carry
/// [`Type`] information. They also participate in the reflection
/// system and allow arbitrary access to fields and methods.
pub trait Object: Any {
  // conversions
  fn into_any(self: Box<Self>) -> Box<dyn Any>;

  // de-referencing
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[cfg(test)]
mod tests {
  use macros::Object;

  use super::*;
  use crate as surreal;

  pub trait TestService: Object {}

  #[derive(Object)]
  pub struct TestServiceImpl {}

  impl TestService for TestServiceImpl {}

  #[test]
  fn object_should_allow_conversion_to_other_types() {
    let obj: Box<dyn TestService> = Box::new(TestServiceImpl {});
    let any = obj.into_any();

    assert!(any.is::<TestServiceImpl>());
  }
}
