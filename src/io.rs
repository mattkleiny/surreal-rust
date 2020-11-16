//! Input/output abstractions and virtual file system.

pub use vfs::*;

mod vfs;

pub type IOResult<T> = std::result::Result<T, Error>;

/// Permits binary I/O on some type.
pub trait BinaryStream {
  fn read_bytes(&mut self, buffer: &mut [u8]) -> IOResult<()>;
  fn write_bytes(&mut self, buffer: &[u8]) -> IOResult<()>;

  fn read_byte(&mut self, value: &mut u8) -> IOResult<u8> {
    let mut buffer = [0 as u8; 1];
    self.read_bytes(&mut buffer)?;
    Ok(buffer[0])
  }

  fn write_byte(&mut self, value: u8) -> IOResult<()> {
    self.write_bytes(&[value])
  }

  fn read_bool(&mut self) -> IOResult<bool> { unimplemented!() }
  fn write_bool(&mut self, value: bool) -> IOResult<()> { unimplemented!() }
  fn read_u16(&mut self) -> IOResult<u16> { unimplemented!() }
  fn write_u16(&mut self, value: u16) -> IOResult<()> { unimplemented!() }
}

/// A type that can be serialized to/from a binary stream.
pub trait BinarySerializable {
  fn read(&mut self, stream: &mut impl BinaryStream) -> IOResult<()>;
  fn write(&mut self, stream: &mut impl BinaryStream) -> IOResult<()>;
}

/// Represents an error in the VFS.
#[derive(Debug)]
pub enum Error {
  General(std::io::Error),
  InvalidPathScheme,
}

impl From<std::io::Error> for Error {
  fn from(error: std::io::Error) -> Self {
    Self::General(error)
  }
}

impl From<Error> for crate::Error {
  fn from(error: Error) -> Self {
    Self::IO(error)
  }
}
