//! IO abstractions for different platforms.

use std::io::{Read, Seek, Write};

use super::*;

/// A path to a file in some file system.
#[derive(Copy, Clone, Debug)]
pub struct Path<'a> {
  scheme: &'a str,
  address: &'a str,
}

impl<'a> Path<'a> {
  pub fn new(path: impl Into<&'a str>) -> Self {
    let (scheme, address) = Self::parse(path);
    Self { scheme, address }
  }

  /// Parses a path into scheme and address.
  fn parse(raw: impl Into<&'a str>) -> (&'a str, &'a str) {
    let string = raw.into();
    let parts: Vec<&str> = string.split("://").collect();

    if parts.len() > 1 {
      let scheme = parts[0];
      let address = parts[1];

      (scheme, address)
    } else {
      ("", string)
    }
  }
}

/// A system for simple file I/O.
pub trait FileSystem {
  type ReadStream: Read + Seek;
  type WriteStream: Write + Seek;

  fn open_read(path: Path) -> Result<Self::ReadStream>;
  fn open_write(path: Path) -> Result<Self::WriteStream>;
}

/// A portable file system implementation from Rust itself.
pub struct PortableFileSystem;

impl FileSystem for PortableFileSystem {
  type ReadStream = std::fs::File;
  type WriteStream = std::fs::File;

  fn open_read(_path: Path) -> Result<Self::ReadStream> {
    unimplemented!()
  }

  fn open_write(_path: Path) -> Result<Self::WriteStream> {
    unimplemented!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_parse_a_basic_path() {
    let (scheme, address) = Path::parse("local://resources/test.png");

    assert_eq!(scheme, "local");
    assert_eq!(address, "resources/test.png");
  }

  #[test]
  fn it_should_parse_an_ambiguous_path() {
    let (scheme, address) = Path::parse("resources/test.png");

    assert_eq!(scheme, "");
    assert_eq!(address, "resources/test.png");
  }
}