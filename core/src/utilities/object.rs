use std::any::Any;

// TODO: remove the need for this

/// Represents a type that convertible to other types.
pub trait Object: Any {
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[macro_export]
macro_rules! impl_object {
  ($name:ty) => {
    impl crate::utilities::Object for $name {
      #[inline(always)]
      fn as_any(&self) -> &dyn std::any::Any {
        self
      }

      #[inline(always)]
      fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
      }
    }
  };
}
