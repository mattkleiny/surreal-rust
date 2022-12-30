use std::any::Any;

/// Represents a type that convertible to other types.
pub trait Object: Any {
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Blanket implementation of [`Object`] for all [`Any`]-types.
impl<A: Any> Object for A {
  fn as_any(&self) -> &dyn Any {
    self as &dyn Any
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as &mut dyn Any
  }
}
