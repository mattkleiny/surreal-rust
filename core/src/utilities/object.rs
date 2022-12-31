use std::any::Any;

/// Represents a type that convertible to other types.
pub trait Object: Any {
  // conversions
  fn into_any(self: Box<Self>) -> Box<dyn Any>;

  // de-referencing
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}
