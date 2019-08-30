//! IO abstractions for different platforms.

use std::io::{Read, Seek, Write};

// TODO: rethink implicit references/etc.

/// Possible types of I/O error.
#[derive(Debug)]
pub enum Error {
  NotFound(String),
  FailedToRead(String),
  FailedToWrite(String),
  Unknown,
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

  fn open_read<P : AsRef<Path>>(path: P) -> Result<Self::ReadStream, Error>;
  fn open_write<P : AsRef<Path>>(path: P) -> Result<Self::WriteStream, Error>;

  fn read_as_string<P : AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut file = Self::open_read(path)?;

    match file.read_to_string(&mut buffer) {
      Ok(_) => Ok(buffer),
      Err(_error) => Err(Error::NotFound(path.address.clone()))
    }
  }
}

/// A portable file system implementation from Rust itself.
pub struct PortableFileSystem;

impl FileSystem for PortableFileSystem {
  type ReadStream = std::fs::File;
  type WriteStream = std::fs::File;

  fn open_read<P : AsRef<Path>>(path: P) -> Result<Self::ReadStream, Error> {
    let path = path.as_ref();
    match std::fs::File::open(path.address) {
      Ok(file) => Ok(file),
      Err(_error) => Err(Error::FailedToRead(path.address.clone()))
    }
  }

  fn open_write<P : AsRef<Path>>(path: P) -> Result<Self::WriteStream, Error> {
    let path = path.as_ref();
    match std::fs::File::create(path.address) {
      Ok(file) => Ok(file),
      Err(_error) => Err(Error::FailedToWrite(path.address.clone()))
    }
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