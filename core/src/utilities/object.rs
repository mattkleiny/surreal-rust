use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;

use macros::Singleton;

use crate as surreal;

/// Represents an 'Object'.
///
/// Objects are polymorphically cast-able at runtime and carry
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

  /// Retrieves the [`TypeMetadata`] for the type.
  pub fn metadata(&self) -> Option<&TypeMetadata> {
    TypeDatabase::instance().get(self)
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

/// Metadata about a [`Type`].
#[derive(Debug)]
pub struct TypeMetadata {}

/// A static [`Type`] database.
///
/// The database carries information about [`Object`]s and allows reflection and
/// introspection of all available objects in a project (that have been registered).
#[derive(Singleton, Default, Debug)]
pub struct TypeDatabase {
  types: HashMap<Type, TypeMetadata>,
}

impl TypeDatabase {
  /// Registers the given [`Type`] in the database.
  pub fn register<T: TypeOf>(&mut self) {
    let key = Type::of::<T>();
    let value = TypeMetadata {};

    self.types.insert(key, value);
  }

  /// Tries to get the [`TypeMetadata`] for the given [`Type`].
  pub fn get(&self, key: &Type) -> Option<&TypeMetadata> {
    self.types.get(key)
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

  #[test]
  fn type_database_should_register_types() {
    let database = TypeDatabase::instance();

    database.register::<TestObject>();

    println!("{:#?}", database);
  }
}
