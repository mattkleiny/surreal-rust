//! Runtime reflection utilities and support.
//!
//! Rust does not have a built-in reflection system, so we have to build our
//! own. This module provides a small reflection system for reading and writing
//! data from and to types and fields dynamically at runtime.

use crate::StringName;

/// Description of a reflected property.
#[derive(Debug)]
pub struct Property {
  /// The name of the property.
  pub name: &'static str,
  /// The type of the property.
  pub kind: &'static str,
  /// Accesses an instance of this property on the given value.
  pub accessor: fn(Address) -> Address,
}

/// Allows for the reflection of types.
pub trait Reflect: Sized {
  /// Gets the name of this type.
  fn name(&self) -> StringName {
    let name = std::any::type_name::<Self>();
    let name = name.rsplit_once("::").map(|split| split.1).unwrap_or(name);

    StringName::from(name)
  }

  /// Gets the properties of this type.
  fn properties(&self) -> &[Property];

  /// Attempts to find the property with the given name.
  fn property(&self, name: &'static str) -> Option<&Property> {
    self.properties().iter().find(|property| property.name == name)
  }

  /// Attempts to get the value of the given property.
  fn get_property<T>(&self, name: &'static str) -> Option<&T> {
    self.property(name).map(|_property| {
      todo!();
    })
  }

  /// Attempts to set the value of the given property.
  fn set_property<T>(&mut self, name: &'static str, _value: T) {
    self.property(name).map(|_property| {
      todo!();
    });
  }
}

/// Implements reflection for the given primitive type.
macro_rules! impl_reflect {
  ($type:ty) => {
    impl Reflect for $type {
      #[inline(always)]
      fn properties(&self) -> &[Property] {
        &[]
      }
    }
  };
}

impl_reflect!(bool);
impl_reflect!(char);
impl_reflect!(u8);
impl_reflect!(u16);
impl_reflect!(u32);
impl_reflect!(u64);
impl_reflect!(i8);
impl_reflect!(i16);
impl_reflect!(i32);
impl_reflect!(i64);
impl_reflect!(f32);
impl_reflect!(f64);
impl_reflect!(String);
impl_reflect!(StringName);

/// The address of a value.
///
/// This is an opaque type that represents the address of a value in memory.
/// This is used to implement reflection for types that do not implement
/// `Reflect` themselves, allowing us to read and write data from and to them
/// dynamically at runtime.
///
/// # Safety
/// This type is unsafe because it allows for the creation of invalid pointers.
/// It is up to the user to ensure that the address is valid.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(usize);

impl Address {
  #[inline(always)]
  pub fn from_ptr<T>(ptr: *const T) -> Self {
    Self(ptr as usize)
  }

  #[inline(always)]
  pub fn from_mut<T>(ptr: *mut T) -> Self {
    Self(ptr as usize)
  }

  #[inline(always)]
  pub fn as_ptr<T>(&self) -> *const T {
    self.0 as *const T
  }

  #[inline(always)]
  pub fn as_mut_ptr<T>(&self) -> *mut T {
    self.0 as *mut T
  }
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
  }

  #[test]
  fn test_struct_fields_should_be_readable() {
    let value = TestStruct {
      name: "Test".to_string(),
      value: 42,
    };

    let name = value.get_property("name");
    let value = value.get_property("value");

    assert_eq!(name, Some(&"Test".to_string()));
    assert_eq!(value, Some(&42));
  }
}
