use std::any::Any;
use std::borrow::Cow;

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

  /// Returns the [`Type`] of this object.
  fn get_type(&self) -> Type;
}

/// Allows accessing [`Type`] information from a type statically.
pub trait TypeOf {
  /// Returns the [`Type`] of this type.
  fn type_of() -> Type;
}

/// Indicates a type name; this is used to identify the type of an object.
///
/// Types can be registered in the [`TypeDatabase`] and interacted with
/// via the reflection subsystem.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Type(Cow<'static, str>);

impl Type {
  /// Retrieves the [`Type`] of the given type.
  #[inline(always)]
  pub fn of<T: TypeOf>() -> Self {
    T::type_of()
  }
}

impl From<&'static str> for Type {
  #[inline(always)]
  fn from(value: &'static str) -> Self {
    Self(Cow::Borrowed(value))
  }
}

impl From<String> for Type {
  #[inline(always)]
  fn from(value: String) -> Self {
    Self(Cow::Owned(value))
  }
}

#[cfg(test)]
mod tests {
  use macros::Object;

  use crate as surreal;

  use super::*;

  #[derive(Object)]
  struct TestObject;

  #[test]
  fn object_should_provide_basic_information() {
    let object = TestObject;

    assert_eq!(object.get_type(), Type::from("TestObject"));
    assert_eq!(Type::of::<TestObject>(), Type::from("TestObject"));
    assert_eq!(TestObject::type_of(), Type::from("TestObject"));
  }
}
