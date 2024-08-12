use crate::FastHashMap;

#[derive(Default)]
pub struct SerializedObject {
  pub fields: FastHashMap<String, SerializedValue>,
}

impl SerializedObject {
  pub fn is_empty(&self) -> bool {
    self.fields.is_empty()
  }

  pub fn len(&self) -> usize {
    self.fields.len()
  }

  pub fn push(&mut self, key: &str, value: impl IntoSerializedValue) {
    self.fields.insert(key.to_owned(), value.into_serialized_value());
  }

  pub fn get(&self, key: &str) -> Option<&SerializedValue> {
    self.fields.get(key)
  }

  pub fn get_mut(&mut self, key: &str) -> Option<&mut SerializedValue> {
    self.fields.get_mut(key)
  }

  pub fn clear(&mut self) {
    self.fields.clear();
  }

  pub fn iter(&self) -> impl Iterator<Item = (&String, &SerializedValue)> {
    self.fields.iter()
  }

  pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut SerializedValue)> {
    self.fields.iter_mut()
  }
}

pub enum SerializedValue {
  Null,
  Bool(bool),
  Int(i64),
  Float(f64),
  String(String),
  Array(Vec<SerializedValue>),
  Object(Box<SerializedObject>),
}

pub trait ToSerializedValue {
  fn to_serialized_value(&self) -> SerializedValue;
}

pub trait IntoSerializedValue {
  fn into_serialized_value(self) -> SerializedValue;
}

pub trait FromSerializedValue: Sized {
  fn from_serialized_value(value: &SerializedValue) -> Option<Self>;
}

pub trait ToSerializedObject {
  fn to_serialized_object(&self) -> SerializedObject;
}

pub trait IntoSerializedObject {
  fn into_serialized_object(self) -> SerializedObject;
}

pub trait FromSerializedObject: Sized {
  fn from_serialized_object(object: &SerializedObject) -> Option<Self>;
}

macro_rules! impl_serialized_value {
  ((), $name:tt) => {
    impl From<()> for SerializedValue {
      #[inline(always)]
      fn from(_: ()) -> Self {
        SerializedValue::$name
      }
    }

    impl ToSerializedValue for () {
      #[inline(always)]
      fn to_serialized_value(&self) -> SerializedValue {
        SerializedValue::$name
      }
    }

    impl IntoSerializedValue for () {
      #[inline(always)]
      fn into_serialized_value(self) -> SerializedValue {
        SerializedValue::$name
      }
    }

    impl FromSerializedValue for () {
      fn from_serialized_value(value: &SerializedValue) -> Option<Self> {
        if let SerializedValue::$name = value {
          Some(())
        } else {
          None
        }
      }
    }
  };
  ($type:ty, $name:tt) => {
    impl From<$type> for SerializedValue {
      #[inline(always)]
      fn from(value: $type) -> Self {
        SerializedValue::$name(value)
      }
    }

    impl ToSerializedValue for $type {
      #[inline(always)]
      fn to_serialized_value(&self) -> SerializedValue {
        SerializedValue::$name(self.clone())
      }
    }

    impl IntoSerializedValue for $type {
      #[inline(always)]
      fn into_serialized_value(self) -> SerializedValue {
        SerializedValue::$name(self)
      }
    }

    impl FromSerializedValue for $type {
      fn from_serialized_value(value: &SerializedValue) -> Option<Self> {
        if let SerializedValue::$name(value) = value {
          Some(value.clone())
        } else {
          None
        }
      }
    }
  };
}

impl_serialized_value!((), Null);
impl_serialized_value!(bool, Bool);
impl_serialized_value!(i64, Int);
impl_serialized_value!(f64, Float);
impl_serialized_value!(String, String);

impl IntoSerializedValue for SerializedValue {
  #[inline(always)]
  fn into_serialized_value(self) -> SerializedValue {
    self
  }
}

impl<'a, Value: ToSerializedValue> IntoSerializedValue for &'a Value {
  #[inline(always)]
  fn into_serialized_value(self) -> SerializedValue {
    self.to_serialized_value()
  }
}

impl<'a, Value: ToSerializedValue> ToSerializedValue for &'a [Value] {
  fn to_serialized_value(&self) -> SerializedValue {
    SerializedValue::Array(self.iter().map(ToSerializedValue::to_serialized_value).collect())
  }
}

impl<Object: ToSerializedObject> ToSerializedValue for Object {
  fn to_serialized_value(&self) -> SerializedValue {
    SerializedValue::Object(Box::new(self.to_serialized_object()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  pub struct TestStruct {
    pub field1: i64,
    pub field2: String,
  }

  impl ToSerializedObject for TestStruct {
    fn to_serialized_object(&self) -> SerializedObject {
      let mut object = SerializedObject::default();

      object.push("field1", self.field1.to_serialized_value());
      object.push("field2", self.field2.to_serialized_value());

      object
    }
  }

  impl FromSerializedObject for TestStruct {
    fn from_serialized_object(object: &SerializedObject) -> Option<Self> {
      let field1 = i64::from_serialized_value(object.get("field1")?)?;
      let field2 = String::from_serialized_value(object.get("field2")?)?;

      Some(Self { field1, field2 })
    }
  }

  #[test]
  fn test_basic_serialized_object() {
    let mut object = SerializedObject::default();

    assert!(object.is_empty());

    object.push("key1", 42);
    object.push("key2", "hello".to_string());
    object.push("key3", &TestStruct {
      field1: 42,
      field2: "hello".to_owned(),
    });

    assert_eq!(object.len(), 3);

    object.clear();
  }

  #[test]
  fn test_structured_serialized_object() {
    let test_struct = TestStruct {
      field1: 42,
      field2: "hello".to_owned(),
    };

    let object = test_struct.to_serialized_object();

    assert_eq!(object.len(), 2);
  }
}
