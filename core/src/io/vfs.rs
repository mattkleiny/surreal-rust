//! A virtual file system with paths and common operations.

pub use local::*;

mod local;

thread_local! {
  static CURRENT_FILE_SYSTEM: LocalFileSystem = LocalFileSystem::new();
}

/// A stream for reading from some [`VirtualPath`].
pub trait InputStream: std::io::BufRead + std::io::Seek {}

/// A stream for writing to some [`VirtualPath`].
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
  fn open_read(&self, path: &VirtualPath) -> crate::Result<Self::InputStream>;
  fn open_write(&self, path: &VirtualPath) -> crate::Result<Self::OutputStream>;
}

/// Represents a path in a virtual file system.
///
/// A path is a scheme and a location within that scheme. The scheme
/// determines which file system component we delegate to for file operations,
/// and so allows for intermixing storage formats and technologies.
///
/// For example, a path might be `file://Assets/Textures/Texture01.png`, or
/// `zip://Assets.zip/Textures/Texture01.png`, or something more exotic like a
/// packed storage scheme `packed://Assets.pak/Textures/Texture01.png`.
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

  /// Returns a new path with a different file extension.
  pub fn change_extension(&self, _new_extension: &'a str) -> Self {
    todo!()
  }

  /// Opens a reader for the given path.
  pub fn open_input_stream(&self) -> crate::Result<Box<dyn InputStream>> {
    let stream = CURRENT_FILE_SYSTEM.with(|file_system| file_system.open_read(self))?;

    Ok(Box::new(stream))
  }

  /// Opens a writer for the given path.
  pub fn open_output_stream(&self) -> crate::Result<Box<dyn OutputStream>> {
    let stream = CURRENT_FILE_SYSTEM.with(|file_system| file_system.open_write(self))?;

    Ok(Box::new(stream))
  }

  /// Attempts to read all bytes from the given path.
  pub fn read_all_bytes(&self) -> crate::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut stream = self.open_input_stream()?;

    stream.read_to_end(&mut buffer)?;

    Ok(buffer)
  }

  /// Attempts to read all text from the given path.
  pub fn read_all_text(&self) -> crate::Result<String> {
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

impl<'a> std::fmt::Display for VirtualPath<'a> {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Ok(write!(formatter, "{:}://{:}", self.scheme, self.location)?)
  }
}

impl<'a> From<&'a str> for VirtualPath<'a> {
  fn from(value: &'a str) -> Self {
    VirtualPath::parse(value)
  }
}

impl<'a> From<&'a String> for VirtualPath<'a> {
  fn from(value: &'a String) -> Self {
    VirtualPath::parse(value.as_str())
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
