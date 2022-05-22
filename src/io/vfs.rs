//! A virtual file system with paths and common operations.

/// Represents a fallible result in the virtual file system.
pub type FileResult<T> = anyhow::Result<T>;

/// Represents a path in a virtual file system.
#[derive(Copy, Clone)]
pub struct VirtualPath<'a> {
  scheme: &'a str,
  location: &'a str,
}

impl<'a> VirtualPath<'a> {
  /// Parses the given string-like object into a path with scheme and location.
  pub fn parse(raw: &'a str) -> Self {
    let split: Vec<&str> = raw.split("://").collect();

    if split.len() != 2 {
      return Self {
        scheme: "local",
        location: split[0],
      };
    }

    Self {
      scheme: split[0],
      location: split[1],
    }
  }

  /// Returns the file extension of the path.
  pub fn extension(&self) -> &'a str {
    todo!()
  }

  /// Attempts to read all text from the given path.
  pub fn read_all_text(&self) -> FileResult<String> {
    todo!()
  }
}

impl<'a> std::fmt::Debug for VirtualPath<'a> {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Ok(write!(formatter, "{:}://{:}", self.scheme, self.location)?)
  }
}

/// A stream allows buffered I/O on some source.
pub trait Stream {}

/// Represents a type capable of acting as a file system.
pub trait FileSystem {
  type Stream: Stream;

  // basic operations
  fn exists(&self, path: VirtualPath) -> bool;
  fn is_file(&self, path: VirtualPath) -> bool;
  fn is_directory(&self, path: VirtualPath) -> bool;

  // read and write
  fn open_read(&self, path: VirtualPath) -> FileResult<Self::Stream>;
  fn open_write(&self, path: VirtualPath) -> FileResult<Self::Stream>;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn path_should_parse_simple_schemes() {
    let path = VirtualPath::parse("local://README.md");

    assert_eq!("local", path.scheme);
    assert_eq!("README.md", path.location);
    assert_eq!("local://README.md", format!("{:?}", path));
  }
}
