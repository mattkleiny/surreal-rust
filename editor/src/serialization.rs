//! Property and object serialization tools.

use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use surreal::io::VirtualPath;

/// A dynamic object that can be serialized/deserialized.
#[derive(Default)]
pub struct SerializedObject {
  _properties: HashMap<String, SerializedValue>,
}

impl SerializedObject {
  /// Creates a new [`SerializedObject`] from the given [`VirtualPath`].
  pub fn from_path(_path: impl Into<VirtualPath>) -> surreal::Result<Self> {
    todo!()
  }

  /// Finds the given [`SerializedProperty`] in the serialized object.
  pub fn find_property<T: Into<SerializedValue>>(&self, _name: impl AsRef<str>) -> Option<SerializedProperty<T>> {
    todo!()
  }

  /// Finds or create the given [`SerializedProperty`] in the serialized object.
  pub fn find_or_create<T: Into<SerializedValue> + Default>(&mut self, _path: impl AsRef<str>) -> SerializedProperty<T> {
    todo!()
  }

  /// Flush any changes made to the object down to the underlying storage.
  pub fn flush_changes(&mut self) -> surreal::Result<()> {
    todo!()
  }
}

impl Serialize for SerializedObject {
  fn serialize<S: Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
    todo!()
  }
}

impl<'de> Deserialize<'de> for SerializedObject {
  fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
    todo!()
  }
}

/// A single property in a [`SerializedObject`].
#[derive(Default, Clone, Debug)]
pub struct SerializedProperty<T> {
  path: String,
  phantom_data: std::marker::PhantomData<T>,
}

impl<T: Into<SerializedValue>> SerializedProperty<T> {
  /// The path of the property.
  pub fn path(&self) -> &str {
    &self.path
  }

  /// Gets the value of the property.
  pub fn get(&self) -> T {
    todo!()
  }

  /// Sets the value of the property.
  pub fn set(&mut self, _value: T) {
    todo!()
  }
}

/// The internal value for a [`SerializedProperty`].
#[derive(Clone, Debug)]
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
  // use super::*;
  //
  // #[test]
  // fn serialization_api_should_be_easy_to_use() {
  //   use surreal::io::Serializable;
  //
  //   let mut object = SerializedObject::default();
  //
  //   let mut health = object.find_or_create::<u16>("player/health");
  //   let mut stamina = object.find_or_create::<u16>("player/stamina");
  //   let mut is_dead = object.find_or_create::<bool>("player/is_dead");
  //
  //   health.set(100);
  //   stamina.set(100);
  //   is_dead.set(true);
  //
  //   println!("Serialized bytes: {:?}", object.to_binary().unwrap());
  //   println!("Serialized json: {:?}", object.to_json().unwrap());
  // }
}
