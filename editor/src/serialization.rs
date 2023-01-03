//! Property and object serialization tools.

use std::collections::HashMap;

use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};

/// A dynamic object that can be serialized/deserialized.
#[derive(Serialize, Deserialize, Default)]
pub struct SerializedObject {
  properties: HashMap<String, SerializedValue>,
}

impl SerializedObject {
  /// Reads a [`SerializedProperty`] from the object.
  pub fn read_property<T: From<SerializedValue>>(&self, property: &SerializedProperty<T>) -> surreal::Result<T> {
    self
      .properties
      .get(property.path)
      .map(|value| value.clone().into())
      .ok_or_else(|| surreal::anyhow!("Property not found {}", property.path))
  }

  /// Writes a [`SerializedProperty`] to the object.
  pub fn write_property<T: Into<SerializedValue>>(&mut self, property: &SerializedProperty<T>, value: T) {
    self.properties.insert(property.path.to_string(), value.into());
  }
}

/// A single property in a [`SerializedObject`].
#[derive(Default, Clone, Debug)]
pub struct SerializedProperty<T> {
  path: &'static str,
  _marker: std::marker::PhantomData<T>,
}

impl<T> SerializedProperty<T> {
  #[inline]
  pub const fn new(path: &'static str) -> Self {
    Self {
      path,
      _marker: std::marker::PhantomData,
    }
  }
}

/// The internal value for a [`SerializedProperty`].
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SerializedValue {
  Bool(bool),
  Integer(i64),
  Float(f64),
  String(String),
  Array(Vec<SerializedValue>),
}

macro_rules! impl_serialized_value {
  ($type:ty, $variant:ident, $cast:ty) => {
    impl Into<SerializedValue> for $type {
      #[inline(always)]
      fn into(self) -> SerializedValue {
        SerializedValue::$variant(self as $cast)
      }
    }

    impl From<SerializedValue> for $type {
      #[inline(always)]
      fn from(value: SerializedValue) -> Self {
        match value {
          SerializedValue::$variant(value) => value as $type,
          _ => panic!("Invalid type"),
        }
      }
    }
  };
}

impl_serialized_value!(bool, Bool, bool);
impl_serialized_value!(i8, Integer, i64);
impl_serialized_value!(i16, Integer, i64);
impl_serialized_value!(i32, Integer, i64);
impl_serialized_value!(i64, Integer, i64);
impl_serialized_value!(u8, Integer, i64);
impl_serialized_value!(u16, Integer, i64);
impl_serialized_value!(u32, Integer, i64);
impl_serialized_value!(u64, Integer, i64);
impl_serialized_value!(usize, Integer, i64);
impl_serialized_value!(f32, Float, f64);
impl_serialized_value!(f64, Float, f64);
impl_serialized_value!(String, String, String);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serialization_api_should_be_easy_to_use() {
    use surreal::io::Serializable;

    const HEALTH: SerializedProperty<u16> = SerializedProperty::new("player/health");
    const STAMINA: SerializedProperty<u16> = SerializedProperty::new("player/stamina");
    const IS_DEAD: SerializedProperty<bool> = SerializedProperty::new("player/is_dead");

    let mut object = SerializedObject::default();

    object.write_property(&HEALTH, 100);
    object.write_property(&STAMINA, 50);
    object.write_property(&IS_DEAD, true);

    println!("Serialized bytes: {:?}", object.to_binary().unwrap());
    println!("Serialized json: {:?}", object.to_json().unwrap());

    println!("Health is {:?}", object.read_property(&HEALTH).unwrap());
    println!("Stamina is {:?}", object.read_property(&STAMINA).unwrap());
    println!("Is dead is {:?}", object.read_property(&IS_DEAD).unwrap());
  }
}
