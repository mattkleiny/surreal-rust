//! Runtime reflection utilities and support.
//!
//! Rust does not have a built-in reflection system, so we have to build our
//! own. This module provides a small reflection system for reading and writing
//! data from and to types and fields dynamically at runtime.

use std::any::Any;

use crate::StringName;

/// Allows reflecting over arbitrary types.
pub trait Reflect: Any + Send + Sync {
  /// Gets the name of this type.
  fn type_name(&self) -> StringName {
    let name = std::any::type_name::<Self>();
    let name = name.rsplit_once("::").map(|split| split.1).unwrap_or(name);

    StringName::from(name)
  }
}

/// Blanket implementation of `Reflect` for all types.
impl<T: Any + Send + Sync> Reflect for T {}

/// Allows reflecting over a primitive type.
pub trait Type: Reflect {
  /// Gets the value of this type.
  fn value(&self) -> &dyn Any;
}

/// Blanket implementation of `PrimitiveType` for all copyable types
impl<T: Reflect> Type for T {
  #[inline(always)]
  fn value(&self) -> &dyn Any {
    self
  }
}

/// Allows reflecting over a struct type.
pub trait StructType: Type {
  /// Gets the fields of this struct.
  fn fields(&self) -> &[FieldInfo];

  /// Gets the field with the given name.
  fn field(&self, name: &str) -> Option<&FieldInfo> {
    self.fields().iter().find(|field| field.name == name)
  }

  /// Gets the value of the field with the given name.
  fn get_value<T: 'static>(&self, name: &str) -> Option<&T> {
    self.field(name).and_then(|field| unsafe {
      let value = {
        let ptr = self as *const Self as *const u8;
        let ptr = ptr.add(field.offset);
        let ptr = ptr as *const dyn Any;

        &*ptr
      };

      value.downcast_ref::<T>()
    })
  }
}

/// Description of a field in a struct.
#[derive(Debug)]
pub struct FieldInfo {
  /// The name of the field.
  pub name: &'static str,
  /// The type of the field.
  pub kind: &'static str,
  /// The offset of the field in the struct.
  pub offset: usize,
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
    let value = TestStruct {
      name: "Test".to_string(),
      value: 42,
    };

    let fields = value.fields();

    assert_eq!(fields.len(), 2);

    assert_eq!(fields[0].name, "name");
    assert_eq!(fields[0].kind, "String");

    assert_eq!(fields[1].name, "value");
    assert_eq!(fields[1].kind, "u32");
  }

  // #[test]
  // fn test_struct_fields_should_be_readable() {
  //   let value = TestStruct {
  //     name: "Test".to_string(),
  //     value: 42,
  //   };

  //   let name = value.get_value("name");
  //   let value = value.get_value("value");

  //   assert_eq!(name, Some(&"Test".to_string()));
  //   assert_eq!(value, Some(&42));
  // }

  // #[test]
  // fn test_struct_fields_should_be_writable() {
  //   let mut value = TestStruct {
  //     name: "Test".to_string(),
  //     value: 42,
  //   };

  //   value.set_property("name", "Test2".to_string());
  //   value.set_property("value", 43);

  //   assert_eq!(value.name, "Test2".to_string());
  //   assert_eq!(value.value, 43);
  // }
}
