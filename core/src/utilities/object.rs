use std::any::{Any, TypeId};

/// Represents a type that convertible to other types.
pub trait Object: Any {
  /// Retrieves the [`TypeId`] of this object.
  fn type_id(&self) -> TypeId {
    TypeId::of::<Self>()
  }

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
