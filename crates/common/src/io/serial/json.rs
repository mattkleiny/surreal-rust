//! JSON serialization format.

use super::*;

/// A JSON [`FileFormat`].
pub struct JsonFileFormat;

impl FileFormat for JsonFileFormat {
  fn read<V: Deserialize>(&self, _stream: &mut dyn InputStream) -> Result<V, Self::Error> {
    todo!()
  }

  fn write<V: Serialize>(&self, _stream: &mut dyn OutputStream, _value: V) -> Result<(), Self::Error> {
    todo!()
  }
}
