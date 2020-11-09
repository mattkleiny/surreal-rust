use smallvec::alloc::fmt::Formatter;

pub use local::*;

pub type PathResult<T> = std::result::Result<T, Error>;

/// Abstracts over an underlying file system.
pub trait FileSystem {
  fn open_file(&self, path: Path) -> PathResult<std::fs::File>;
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
  pub fn parse<S: AsRef<str> + ?Sized>(raw: &'a S) -> PathResult<Self> {
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
    fn open_file(&self, path: Path) -> PathResult<std::fs::File> {
      Ok(std::fs::File::open(path.location)?)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_parse_simple_schemes() -> PathResult<()> {
    let path = Path::parse("local://README.md").unwrap();

    assert_eq!("local", path.scheme);
    assert_eq!("README.md", path.location);

    Ok(())
  }
}
