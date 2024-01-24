//! Runtime reflection utilities and support.
//!
//! Rust does not have a built-in reflection system, so we have to build our
//! own. This module provides a small reflection system for reading and writing
//! data from and to types and fields dynamically at runtime.

use crate::StringName;

/// An empty list of properties.
const EMPTY_PROPERTIES: Vec<Property> = vec![];

/// Description of a reflected property.
#[derive(Debug)]
pub struct Property {
  /// The name of the property.
  pub name: &'static str,
  /// The type of the property.
  pub kind: &'static str,
}

/// Allows for the reflection of types.
pub trait Reflect {
  /// Gets the name of this type.
  fn name(&self) -> StringName {
    let name = std::any::type_name::<Self>();
    let name = name.rsplit_once("::").map(|split| split.1).unwrap_or(name);

    StringName::from(name)
  }

  /// Gets the properties of this type.
  fn properties(&self) -> Vec<Property>;
}

/// Implements reflection for the given primitive type.
macro_rules! impl_reflect_primitive {
  ($type:ty) => {
    impl Reflect for $type {
      #[inline(always)]
      fn properties(&self) -> Vec<Property> {
        EMPTY_PROPERTIES
      }
    }
  };
}

impl_reflect_primitive!(bool);
impl_reflect_primitive!(char);
impl_reflect_primitive!(u8);
impl_reflect_primitive!(u16);
impl_reflect_primitive!(u32);
impl_reflect_primitive!(u64);
impl_reflect_primitive!(i8);
impl_reflect_primitive!(i16);
impl_reflect_primitive!(i32);
impl_reflect_primitive!(i64);
impl_reflect_primitive!(f32);
impl_reflect_primitive!(f64);
impl_reflect_primitive!(String);
impl_reflect_primitive!(StringName);

#[cfg(test)]
mod tests {
  use macros::Reflect;

  use super::*;

  #[derive(Reflect)]
  #[allow(dead_code)]
  struct TestStruct {
    name: String,
    value: u32,
  }

  #[test]
  fn test_name_should_match_expected_type() {
    let value = TestStruct {
      name: "Test".to_string(),
      value: 42,
    };

    assert_eq!(0u32.name(), "u32");
    assert_eq!(0u64.name(), "u64");
    assert_eq!(0i32.name(), "i32");
    assert_eq!(0i64.name(), "i64");
    assert_eq!(value.name(), "TestStruct")
  }

  #[test]
  fn test_struct_fields_should_be_expanded() {
    let value = TestStruct {
      name: "Test".to_string(),
      value: 42,
    };

    let properties = value.properties();

    assert_eq!(properties.len(), 2);
    assert_eq!(properties[0].name, "name");
    assert_eq!(properties[0].kind, "String");
    assert_eq!(properties[1].name, "value");
    assert_eq!(properties[1].kind, "u32");

    println!("{:#?}", properties)
  }
}
