//! A virtual file system with paths and common operations.

pub use local::*;

mod local;

// TODO: put a file system registry here
thread_local! {
  static CURRENT_FILE_SYSTEM: LocalFileSystem = LocalFileSystem::new();
}

/// Represents a fallible result in the virtual file system.
pub type FileResult<T> = anyhow::Result<T>;

/// A stream for reading from some `VirtualPath`.
pub trait InputStream: std::io::BufRead + std::io::Seek {}

/// A stream for writing to some `VirtualPath`.
pub trait OutputStream: std::io::Write + std::io::Seek {}

/// Represents a type capable of acting as a file system.
pub trait FileSystem {
  type InputStream: InputStream;
  type OutputStream: OutputStream;

  // basic operations
  fn exists(&self, path: &VirtualPath) -> bool;
  fn is_file(&self, path: &VirtualPath) -> bool;
  fn is_directory(&self, path: &VirtualPath) -> bool;

  // read and write
  fn open_read(&self, path: &VirtualPath) -> FileResult<Self::InputStream>;
  fn open_write(&self, path: &VirtualPath) -> FileResult<Self::OutputStream>;
}

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
    if let Some(extension) = self.location.split('.').last() {
      extension
    } else {
      self.location
    }
  }

  /// Opens a reader for the given path.
  pub fn open_input_stream(&self) -> FileResult<Box<dyn InputStream>> {
    let stream = CURRENT_FILE_SYSTEM.with(|file_system| {
      file_system.open_read(self)
    })?;

    Ok(Box::new(stream))
  }

  /// Opens a writer for the given path.
  pub fn open_output_stream(&self) -> FileResult<Box<dyn OutputStream>> {
    let stream = CURRENT_FILE_SYSTEM.with(|file_system| {
      file_system.open_write(self)
    })?;

    Ok(Box::new(stream))
  }

  /// Attempts to read all bytes from the given path.
  pub fn read_all_bytes(&self) -> FileResult<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut stream = self.open_input_stream()?;

    stream.read_to_end(&mut buffer)?;

    Ok(buffer)
  }

  /// Attempts to read all text from the given path.
  pub fn read_all_text(&self) -> FileResult<String> {
    let mut buffer = String::new();
    let mut stream = self.open_input_stream()?;

    stream.read_to_string(&mut buffer)?;

    Ok(buffer)
  }
}

impl<'a> std::fmt::Debug for VirtualPath<'a> {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Ok(write!(formatter, "{:}://{:}", self.scheme, self.location)?)
  }
}

/// Allows a type to be converted to a `VirtualPath`.
pub trait AsVirtualPath {
  fn as_virtual_path(&self) -> VirtualPath;
}

impl AsVirtualPath for &str {
  fn as_virtual_path(&self) -> VirtualPath {
    VirtualPath::parse(self)
  }
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
