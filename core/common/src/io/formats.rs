use crate::{FastHashMap, FromVariant, InputStream, OutputStream, StreamError, ToVariant, ToVirtualPath, Variant};

mod binary;
mod json;

pub use binary::*;
pub use json::*;

/// A chunk of serialized data
#[derive(Debug, PartialEq)]
pub enum Chunk {
  Variant(Variant),
  Sequence(Vec<Chunk>),
  Map(FastHashMap<String, Chunk>),
}

/// Represents a type that can be serialized.
pub trait Serialize: Sized {
  // TODO: make this fallible
  /// Serializes the type into a chunk.
  fn serialize(&self) -> Chunk;

  /// Serializes the type to a binary byte array.
  fn to_binary_bytes(&self) -> Result<Vec<u8>, StreamError> {
    Self::to_format_bytes::<BinaryFormat>(self)
  }

  /// Serializes the type to a binary file.
  fn to_binary_path(&self, path: impl ToVirtualPath) -> Result<(), StreamError> {
    Self::to_format_path::<BinaryFormat>(self, path)
  }

  /// Serializes the type to a JSON byte array.
  fn to_json_bytes(&self) -> Result<Vec<u8>, StreamError> {
    Self::to_format_bytes::<JsonFormat>(self)
  }

  /// Serializes the type to a JSON string.
  fn to_json_string(&self) -> Result<String, StreamError> {
    Self::to_format_string::<JsonFormat>(self)
  }

  /// Serializes the type to a JSON file.
  fn to_json_path(&self, path: impl ToVirtualPath) -> Result<(), StreamError> {
    Self::to_format_path::<JsonFormat>(self, path)
  }

  /// Serializes the type to a byte array with a specific format.
  fn to_format_bytes<F: Format + Default>(&self) -> Result<Vec<u8>, StreamError> {
    let mut format = F::default();
    let mut stream = std::io::Cursor::new(Vec::new());

    format.write_chunk(&mut stream, &self.serialize())?;

    Ok(stream.into_inner())
  }

  /// Serializes the type to a string with a specific format.
  fn to_format_string<F: Format + Default>(&self) -> Result<String, StreamError> {
    let mut format = F::default();
    let mut stream = std::io::Cursor::new(Vec::new());

    format.write_chunk(&mut stream, &self.serialize())?;

    Ok(String::from_utf8(stream.into_inner())?)
  }

  /// Serializes the type to a path with a specific format.
  fn to_format_path<F: Format + Default>(&self, path: impl ToVirtualPath) -> Result<(), StreamError> {
    let path = path.to_virtual_path();
    let mut format = F::default();
    let mut stream = path.open_output_stream()?;

    format.write_chunk(&mut stream, &self.serialize())
  }
}

/// Represents a type that can be deserialized.
pub trait Deserialize: Sized {
  /// Deserializes a chunk into this type.
  fn deserialize(chunk: &Chunk) -> Self;

  /// Deserializes the type from a binary byte array.
  fn from_binary_bytes(data: &[u8]) -> Result<Self, StreamError> {
    Self::from_format_bytes::<BinaryFormat>(data)
  }

  /// Deserializes the type from a binary path.
  fn from_binary_path(path: impl ToVirtualPath) -> Result<Self, StreamError> {
    Self::from_format_path::<BinaryFormat>(path)
  }

  /// Deserializes the type from a JSON byte array.
  fn from_json_bytes(data: &[u8]) -> Result<Self, StreamError> {
    Self::from_format_bytes::<JsonFormat>(data)
  }

  /// Deserializes the type from a JSON string.
  fn from_json_string(data: &str) -> Result<Self, StreamError> {
    Self::from_format_string::<JsonFormat>(data)
  }

  /// Deserializes the type from a JSON path.
  fn from_json_path(path: impl ToVirtualPath) -> Result<Self, StreamError> {
    Self::from_format_path::<JsonFormat>(path)
  }

  /// Deserializes the type from a byte slice with a specific format.
  fn from_format_bytes<F: Format + Default>(data: &[u8]) -> Result<Self, StreamError> {
    let mut format = F::default();
    let mut stream = std::io::Cursor::new(data);

    format.read_chunk(&mut stream).map(|chunk| Self::deserialize(&chunk))
  }

  /// Deserializes the type from a string with a specific format.
  fn from_format_string<F: Format + Default>(data: &str) -> Result<Self, StreamError> {
    let mut format = F::default();
    let mut stream = std::io::Cursor::new(data.as_bytes());

    format.read_chunk(&mut stream).map(|chunk| Self::deserialize(&chunk))
  }

  /// Deserializes the type from a path with a specific format.
  fn from_format_path<F: Format + Default>(path: impl ToVirtualPath) -> Result<Self, StreamError> {
    let path = path.to_virtual_path();
    let mut format = F::default();
    let mut stream = path.open_input_stream()?;

    format.read_chunk(&mut stream).map(|chunk| Self::deserialize(&chunk))
  }
}

/// A format for reading/writing data.
pub trait Format {
  fn read_chunk(&mut self, stream: &mut dyn InputStream) -> Result<Chunk, StreamError>;
  fn write_chunk(&mut self, stream: &mut dyn OutputStream, chunk: &Chunk) -> Result<(), StreamError>;
}

impl<V: ToVariant> Serialize for V {
  #[inline]
  fn serialize(&self) -> Chunk {
    Chunk::Variant(self.to_variant())
  }
}

impl<V: FromVariant> Deserialize for V {
  fn deserialize(chunk: &Chunk) -> Self {
    match chunk {
      Chunk::Variant(value) => Self::from_variant(value.clone()).unwrap(),
      Chunk::Sequence(_) => panic!("Unable to deserialize sequence into a single value"),
      Chunk::Map(_) => panic!("Unable to deserialize map into a single value"),
    }
  }
}

impl<V: Serialize> Serialize for Vec<V> {
  fn serialize(&self) -> Chunk {
    Chunk::Sequence(self.iter().map(Serialize::serialize).collect())
  }
}

impl<V: Deserialize> Deserialize for Vec<V> {
  fn deserialize(chunk: &Chunk) -> Self {
    match chunk {
      Chunk::Variant(_) => panic!("Unable to deserialize variant into a sequence"),
      Chunk::Sequence(values) => values.iter().map(Deserialize::deserialize).collect(),
      Chunk::Map(_) => panic!("Unable to deserialize map into a sequence"),
    }
  }
}

#[cfg(test)]
mod tests {
  use macros::{Deserialize, Serialize};

  use super::*;
  use crate::{Color32, Vec3};

  /// A test struct for serialization/deserialization.
  #[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
  struct TestStruct {
    value_1: u32,
    value_2: f64,
    value_3: String,
    value_4: NestedStruct,
  }

  /// A nested struct for serialization/deserialization.
  #[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
  struct NestedStruct {
    value_1: Vec3,
    value_2: Color32,
  }

  #[test]
  fn it_should_serialize_and_deserialize_basic_types() {
    assert_eq!(42u32.serialize(), Chunk::Variant(Variant::U32(42)));
    assert_eq!(42f32, f32::deserialize(&Chunk::Variant(Variant::U32(42))));
  }

  #[test]
  fn it_should_serialize_struct_types() {
    let input = TestStruct {
      value_1: 42,
      value_2: std::f64::consts::PI,
      value_3: "Hello, World!".to_string(),
      value_4: NestedStruct {
        value_1: Vec3::new(0., 1., 2.),
        value_2: Color32::rgb(255, 0, 255),
      },
    };

    let chunk = input.serialize();
    let output = TestStruct::deserialize(&chunk);

    assert_eq!(input, output);
  }
}
