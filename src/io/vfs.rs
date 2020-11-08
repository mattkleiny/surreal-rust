use std::io::{Read, Write};

use smallvec::alloc::fmt::Formatter;

pub use local::*;
pub use resource::*;

type Result<T> = std::result::Result<T, Error>;

/// Abstracts over an underlying file system.
pub trait FileSystem {
  fn open_file(&self, path: Path) -> Result<std::fs::File>;
}

/// Represents a path in a virtual file system.
#[derive(Copy, Clone)]
pub struct Path<'a> {
  scheme: &'a str,
  location: &'a str,
  file_system: &'static dyn FileSystem,
}

impl<'a> std::fmt::Debug for Path<'a> {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    Ok(write!(
      formatter,
      "{:?}://{:?}",
      self.scheme, self.location
    )?)
  }
}

impl<'a> Path<'a> {
  /// Parses the given string-like object into a path with scheme and location.
  pub fn parse<S: AsRef<str> + ?Sized>(raw: &'a S) -> Result<Self> {
    let raw = raw.as_ref();
    let split: Vec<&str> = raw.split("://").collect();

    if split.len() != 2 {
      return Err(Error::InvalidScheme);
    }

    Ok(Self {
      scheme: split[0],
      location: split[1],
      file_system: &LocalFileSystem {},
    })
  }
}

/// Represents a path that permits access to it's contents.
pub trait Reader {
  fn read_all_bytes(&self) -> Result<Vec<u8>>;
  fn read_all_text(&self) -> Result<String>;
}

impl<'a> Reader for Path<'a> {
  fn read_all_bytes(&self) -> Result<Vec<u8>> {
    let mut file = self.file_system.open_file(*self)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
  }

  fn read_all_text(&self) -> Result<String> {
    let mut file = self.file_system.open_file(*self)?;
    let mut string = String::new();

    file.read_to_string(&mut string)?;

    Ok(string)
  }
}

/// Represents a path that permits access to it's contents.
pub trait Writer {
  fn write_all_bytes(&self, bytes: &[u8]) -> Result<usize>;
  fn write_all_text(&self, string: &impl AsRef<str>) -> Result<usize>;
}

impl<'a> Writer for Path<'a> {
  fn write_all_bytes(&self, bytes: &[u8]) -> Result<usize> {
    let mut file = self.file_system.open_file(*self)?;

    file.write_all(bytes)?;

    Ok(bytes.len())
  }

  fn write_all_text(&self, string: &impl AsRef<str>) -> Result<usize> {
    let mut file = self.file_system.open_file(*self)?;
    let bytes = string.as_ref().as_bytes();

    file.write_all(bytes)?;

    Ok(bytes.len())
  }
}

/// Represents an error in the VFS.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
  InvalidScheme,
  GeneralIO,
}

impl From<Error> for crate::Error {
  fn from(error: Error) -> Self {
    Self::IO(super::Error::VFS(error))
  }
}

impl From<std::io::Error> for Error {
  fn from(_: std::io::Error) -> Self {
    Self::GeneralIO
  }
}

pub mod local {
  //! A local file system implementation for the VFS.

  use super::*;

  pub struct LocalFileSystem {}

  impl FileSystem for LocalFileSystem {
    fn open_file(&self, path: Path) -> Result<std::fs::File> {
      Ok(std::fs::File::open(path.location)?)
    }
  }
}

pub mod resource {
  //! A resource file system implementation for the VFS.

  use super::*;

  pub struct ResourceFileSystem;

  impl FileSystem for ResourceFileSystem {
    fn open_file(&self, path: Path) -> Result<std::fs::File> {
      unimplemented!()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_parse_simple_schemes() -> Result<()> {
    let path = Path::parse("local://README.md").unwrap();

    assert_eq!("local", path.scheme);
    assert_eq!("README.md", path.location);

    Ok(())
  }

  #[test]
  fn it_should_read_a_simple_file() -> Result<()> {
    let path = Path::parse("local://README.md")?;
    let result = path.read_all_text()?;

    Ok(())
  }
}
