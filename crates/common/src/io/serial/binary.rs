//! Binary serialization format.

use super::*;

/// The binary [`FileFormat`].
pub struct BinaryFileFormat;

impl FileFormat for BinaryFileFormat {
  fn read<V: Deserialize>(&self, _stream: &mut dyn InputStream) -> Result<V, Self::Error> {
    todo!()
  }

  fn write<V: Serialize>(&self, _stream: &mut dyn OutputStream, _value: V) -> Result<(), Self::Error> {
    todo!()
  }
}
