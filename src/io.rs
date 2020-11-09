//! Input/output abstractions and virtual file system.

use std::io::{Read, Write};

pub use vfs::*;

mod vfs;

pub type StreamResult<T> = std::result::Result<T, Error>;

/// Abstracts over a source of bytes for reading/writing.
pub trait Stream {
  /// Attempts to read a sequence of bytes from the stream.
  fn read_bytes(&mut self, buffer: &mut [u8]) -> StreamResult<usize>;

  fn read_byte(&mut self) -> StreamResult<u8> {
    let mut buffer = [0; 1];
    self.read_bytes(&mut buffer)?;
    Ok(buffer[0])
  }

  fn read_all_bytes(&mut self) -> StreamResult<Vec<u8>> {
    let mut buffer = Vec::new();

    while self.read_bytes(&mut buffer)? > 0 {
      // loop
    }

    Ok(buffer)
  }

  fn read_all_text(&mut self) -> StreamResult<String> {
    let mut string = String::new();

    unsafe {
      while self.read_bytes(&mut string.as_bytes_mut())? > 0 {
        // loop
      }
    }

    Ok(string)
  }

  /// Attempts to write a sequence of bytes to the stream.
  fn write_bytes(&mut self, buffer: &[u8]) -> StreamResult<usize>;

  fn write_byte(&mut self, value: u8) -> StreamResult<()> {
    let buffer = [value; 1];
    self.write_bytes(&buffer)?;
    Ok(())
  }

  fn write_all_bytes(&self, bytes: &[u8]) -> StreamResult<usize> {
    unimplemented!()
  }

  fn write_all_text(&self, string: &impl AsRef<str>) -> StreamResult<usize> {
    unimplemented!()
  }
}

/// A stream implementation for standard files.
pub struct FileStream {
  file: std::fs::File,
}

impl FileStream {
  pub fn open(path: impl AsRef<str>) -> StreamResult<Self> {
    let file = std::fs::File::open(path.as_ref())?;

    Ok(Self { file })
  }
}

impl Stream for FileStream {
  fn read_bytes(&mut self, buffer: &mut [u8]) -> StreamResult<usize> {
    Ok(self.file.read(buffer)?)
  }

  fn write_bytes(&mut self, buffer: &[u8]) -> StreamResult<usize> {
    Ok(self.file.write(buffer)?)
  }
}

/// Represents an error in the IO subsystem.
#[derive(Debug)]
pub enum Error {
  VFS(vfs::Error),
  IO(std::io::Error),
}

impl From<Error> for crate::Error {
  fn from(error: Error) -> Self {
    Self::IO(error)
  }
}

impl From<std::io::Error> for Error {
  fn from(error: std::io::Error) -> Self {
    Self::IO(error)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_read_a_simple_file() {
    let mut stream = FileStream::open("./README.md").unwrap();
    let mut buffer = [0; 1024];

    stream.read_bytes(&mut buffer).unwrap();
  }
}