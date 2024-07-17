//! Serialization and deserialization of data structures.

use crate::{FromVariant, InputStream, OutputStream, ToVariant, Variant};

mod binary;
mod json;

pub use binary::*;
pub use json::*;

/// A chunk of serialized data
#[derive(Debug, PartialEq)]
pub enum Chunk {
  Variant(Variant),
  Sequence(Vec<Chunk>),
}

/// Represents a type that can be serialized.
pub trait Serialize {
  fn serialize(&self) -> Chunk;
}

/// Represents a type that can be deserialized.
pub trait Deserialize {
  fn deserialize(chunk: Chunk) -> Self;
}

/// A format for reading/writing data.
pub trait FileFormat {
  /// The error type for this format.
  type Error = super::StreamError;

  fn read<V: Deserialize>(&self, stream: &mut dyn InputStream) -> Result<V, Self::Error>;
  fn write<V: Serialize>(&self, stream: &mut dyn OutputStream, value: V) -> Result<(), Self::Error>;
}

impl<V: ToVariant> Serialize for V {
  #[inline]
  fn serialize(&self) -> Chunk {
    Chunk::Variant(self.to_variant())
  }
}

impl<V: FromVariant> Deserialize for V {
  fn deserialize(chunk: Chunk) -> Self {
    match chunk {
      Chunk::Variant(value) => Self::from_variant(value),
      Chunk::Sequence(_) => todo!(),
    }
  }
}

#[cfg(test)]
mod tests {
  use macros::{Deserialize, Serialize};

  use super::*;

  /// A test struct for serialization/deserialization.
  #[derive(Serialize, Deserialize, Debug, PartialEq)]
  struct TestStruct {
    value_1: u32,
    value_2: f64,
    value_3: String,
  }

  #[test]
  fn it_should_serialize_and_deserialize_basic_types() {
    assert_eq!(42u32.serialize(), Chunk::Variant(Variant::U32(42)));
    assert_eq!(42f32, f32::deserialize(Chunk::Variant(Variant::U32(42))));
  }

  #[test]
  fn it_should_serialize_and_deserialize_struct_types() {
    let original = TestStruct {
      value_1: 42,
      value_2: std::f64::consts::PI,
      value_3: "Hello, World!".to_string(),
    };

    let chunk = original.serialize();
    let deserialized = TestStruct::deserialize(chunk);

    assert_eq!(original, deserialized);
  }
}
