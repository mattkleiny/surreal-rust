//! Runtime reflection utilities and support.
//!
//! Rust does not have a built-in reflection system, so we have to build our
//! own. This module provides a small reflection system for reading and writing
//! data from and to types and fields dynamically at runtime.

use std::any::Any;

use crate::StringName;

/// Allows reflecting over arbitrary types.
pub trait Reflect: Any {
  /// Gets the name of this type.
  fn type_name(&self) -> StringName {
    let name = std::any::type_name::<Self>();
    let name = name.rsplit_once("::").map(|split| split.1).unwrap_or(name);

    StringName::from(name)
  }
}

/// Allows reflecting over a primitive type.
pub trait Type: Reflect {}

/// Blanket implementation of `Reflect` for all types.
impl<T: Any> Reflect for T {}

/// Blanket implementation of `Type` for all basic types
impl<T: Reflect> Type for T {}

/// Allows reflecting over a struct type.
pub trait StructType: Type {
  /// Gets the fields of this struct.
  fn fields() -> &'static [FieldInfo];

  /// Gets the field with the given name.
  fn field(name: &str) -> Option<&FieldInfo> {
    Self::fields().iter().find(|field| field.name == name)
  }

  fn get_field(&self, _name: &str) -> Option<&dyn Type>;
  fn set_field(&mut self, _name: &str, _value: &dyn Type);
}

/// Description of a field in a struct.
#[derive(Debug)]
pub struct FieldInfo {
  /// The name of the field.
  pub name: &'static str,
  /// The type of the field.
  pub kind: &'static str,
}

/// Allows reflecting over an enum type.
pub trait EnumType: Type {
  /// Gets the variants of this enum.
  fn variants(&self) -> &[VariantInfo];
}

/// Description of a variant in an enum.
#[derive(Debug)]
pub struct VariantInfo {
  /// The name of the variant.
  pub name: &'static str,
}

#[cfg(test)]
mod tests {
  use macros::Reflect;

  use super::*;

  #[allow(dead_code)]
  #[derive(Reflect)]
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

    assert_eq!(0u32.type_name(), "u32");
    assert_eq!(0u64.type_name(), "u64");
    assert_eq!(0i32.type_name(), "i32");
    assert_eq!(0i64.type_name(), "i64");
    assert_eq!(value.type_name(), "TestStruct");
  }

  #[test]
  fn test_struct_fields_should_be_queryable() {
    let fields = TestStruct::fields();

    assert_eq!(fields.len(), 2);

    assert_eq!(fields[0].name, "name");
    assert_eq!(fields[0].kind, "String");

    assert_eq!(fields[1].name, "value");
    assert_eq!(fields[1].kind, "u32");
  }
}
