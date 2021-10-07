//! Input/output abstractions and virtual file system.

pub use vfs::*;

mod vfs;

/// A result from a streaming operation
pub type StreamResult<T> = std::result::Result<T, Error>;

/// Permits binary I/O on some type.
pub trait BinaryStream {
  fn read_bytes(&mut self, buffer: &mut [u8]) -> StreamResult<()>;
  fn write_bytes(&mut self, buffer: &[u8]) -> StreamResult<()>;

  fn read_byte(&mut self, value: &mut u8) -> StreamResult<u8> {
    let mut buffer = [0 as u8; 1];
    self.read_bytes(&mut buffer)?;
    Ok(buffer[0])
  }

  fn write_byte(&mut self, value: u8) -> StreamResult<()> {
    self.write_bytes(&[value])
  }

  fn read_bool(&mut self) -> StreamResult<bool> {
    todo!()
  }

  fn write_bool(&mut self, value: bool) -> StreamResult<()> {
    todo!()
  }

  fn read_u16(&mut self) -> StreamResult<u16> {
    todo!()
  }

  fn write_u16(&mut self, value: u16) -> StreamResult<()> {
    todo!()
  }
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
