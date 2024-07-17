//! Serialization and deserialization of data structures.

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
  /// Serializes the type into a chunk.
  fn serialize(&self) -> Chunk;

  /// Serializes the type to a binary byte array.
  fn to_binary_bytes(&self) -> Result<Vec<u8>, StreamError> {
    Self::to_format_bytes::<BinaryFileFormat>(self)
  }

  /// Serializes the type to a binary file.
  fn to_binary_path(&self, path: impl ToVirtualPath) -> Result<(), StreamError> {
    Self::to_format_path::<BinaryFileFormat>(self, path)
  }

  /// Serializes the type to a JSON byte array.
  fn to_json_bytes(&self) -> Result<Vec<u8>, StreamError> {
    Self::to_format_bytes::<JsonFileFormat>(self)
  }

  /// Serializes the type to a JSON string.
  fn to_json_string(&self) -> Result<String, StreamError> {
    Self::to_format_string::<JsonFileFormat>(self)
  }

  /// Serializes the type to a JSON file.
  fn to_json_path(&self, path: impl ToVirtualPath) -> Result<(), StreamError> {
    Self::to_format_path::<JsonFileFormat>(self, path)
  }

  /// Serializes the type to a byte array with a specific format.
  fn to_format_bytes<F: FileFormat + Default>(&self) -> Result<Vec<u8>, StreamError> {
    let mut format = F::default();
    let mut stream = std::io::Cursor::new(Vec::new());

    format.write(&mut stream, self)?;

    Ok(stream.into_inner())
  }

  /// Serializes the type to a string with a specific format.
  fn to_format_string<F: FileFormat + Default>(&self) -> Result<String, StreamError> {
    let mut format = F::default();
    let mut stream = std::io::Cursor::new(Vec::new());

    format.write(&mut stream, self)?;

    Ok(String::from_utf8(stream.into_inner())?)
  }

  /// Serializes the type to a path with a specific format.
  fn to_format_path<F: FileFormat + Default>(&self, path: impl ToVirtualPath) -> Result<(), StreamError> {
    let path = path.to_virtual_path();
    let mut format = F::default();
    let mut stream = path.open_output_stream()?;

    format.write(&mut stream, self)
  }
}

/// Represents a type that can be deserialized.
pub trait Deserialize: Sized {
  /// Deserializes a chunk into this type.
  fn deserialize(chunk: &Chunk) -> Self;

  /// Deserializes the type from a binary byte array.
  fn from_binary_bytes(data: &[u8]) -> Result<Self, StreamError> {
    Self::from_format_bytes::<BinaryFileFormat>(data)
  }

  /// Deserializes the type from a binary path.
  fn from_binary_path(path: impl ToVirtualPath) -> Result<Self, StreamError> {
    Self::from_format_path::<BinaryFileFormat>(path)
  }

  /// Deserializes the type from a JSON byte array.
  fn from_json_bytes(data: &[u8]) -> Result<Self, StreamError> {
    Self::from_format_bytes::<JsonFileFormat>(data)
  }

  /// Deserializes the type from a JSON string.
  fn from_json_string(data: &str) -> Result<Self, StreamError> {
    Self::from_format_string::<JsonFileFormat>(data)
  }

  /// Deserializes the type from a JSON path.
  fn from_json_path(path: impl ToVirtualPath) -> Result<Self, StreamError> {
    Self::from_format_path::<JsonFileFormat>(path)
  }

  /// Deserializes the type from a byte slice with a specific format.
  fn from_format_bytes<F: FileFormat + Default>(data: &[u8]) -> Result<Self, StreamError> {
    let mut format = F::default();
    let mut stream = std::io::Cursor::new(data);

    Ok(format.read(&mut stream)?)
  }

  /// Deserializes the type from a string with a specific format.
  fn from_format_string<F: FileFormat + Default>(data: &str) -> Result<Self, StreamError> {
    let mut format = F::default();
    let mut stream = std::io::Cursor::new(data.as_bytes());

    Ok(format.read(&mut stream)?)
  }

  /// Deserializes the type from a path with a specific format.
  fn from_format_path<F: FileFormat + Default>(path: impl ToVirtualPath) -> Result<Self, StreamError> {
    let path = path.to_virtual_path();
    let mut format = F::default();
    let mut stream = path.open_input_stream()?;

    Ok(format.read(&mut stream)?)
  }
}

/// A format for reading/writing data.
pub trait FileFormat {
  /// Reads a value from the stream.
  #[inline]
  fn read<V: Deserialize>(&mut self, stream: &mut dyn InputStream) -> Result<V, StreamError> {
    self.read_chunk(stream).map(|chunk| V::deserialize(&chunk))
  }

  /// Writes a value to the stream.
  #[inline]
  fn write<V: Serialize>(&mut self, stream: &mut dyn OutputStream, value: &V) -> Result<(), StreamError> {
    self.write_chunk(stream, &value.serialize())
  }

  /// Reads a [`Chunk`] from a stream.
  fn read_chunk(&mut self, stream: &mut dyn InputStream) -> Result<Chunk, StreamError>;

  /// Writes a [`Chunk`] to a stream.
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
      Chunk::Variant(value) => Self::from_variant(value.clone()),
      Chunk::Sequence(_) => panic!("Unable to deserialize sequence into a single value"),
      Chunk::Map(_) => panic!("Unable to deserialize map into a single value"),
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
