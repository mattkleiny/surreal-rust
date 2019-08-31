//! IO abstractions for different platforms.

use std::io::{Read, Seek, Write};

/// Possible types of I/O error.
#[derive(Debug)]
pub enum Error {
  NotFound(String),
  IO(String),
}

/// A path to a file in some file system.
#[derive(Clone, Debug)]
pub struct Path {
  scheme: String,
  address: String,
}

impl Path {
  pub fn new<P: AsRef<str>>(path: P) -> Self {
    let (scheme, address) = Self::parse(path.as_ref());
    Self { scheme, address }
  }

  /// Parses a path into scheme and address.
  fn parse(raw: &str) -> (String, String) {
    let parts: Vec<&str> = raw.split("://").collect();

    if parts.len() > 1 {
      let scheme = parts[0];
      let address = parts[1];

      (scheme.to_string(), address.to_string())
    } else {
      ("".to_string(), raw.to_string())
    }
  }
}

/// A system for simple file I/O.
pub trait FileSystem {
  type ReadStream: Read + Seek;
  type WriteStream: Write + Seek;

  fn open_read<P: AsRef<Path>>(path: P) -> Result<Self::ReadStream, Error>;
  fn open_write<P: AsRef<Path>>(path: P) -> Result<Self::WriteStream, Error>;

  fn read_as_string<P: AsRef<Path>>(_path: P) -> Result<String, Error> {
    unimplemented!()
  }
}

/// A portable file system implementation from Rust itself.
pub struct PortableFileSystem;

impl FileSystem for PortableFileSystem {
  type ReadStream = std::fs::File;
  type WriteStream = std::fs::File;

  fn open_read<P: AsRef<Path>>(_path: P) -> Result<Self::ReadStream, Error> {
    unimplemented!()
  }

  fn open_write<P: AsRef<Path>>(_path: P) -> Result<Self::WriteStream, Error> {
    unimplemented!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn path_should_a_basic_path() {
    let (scheme, address) = Path::parse("local://resources/test.png");

    assert_eq!(scheme, "local");
    assert_eq!(address, "resources/test.png");
  }

  #[test]
  fn path_should_an_ambiguous_path() {
    let (scheme, address) = Path::parse("resources/test.png");

    assert_eq!(scheme, "");
    assert_eq!(address, "resources/test.png");
  }
}