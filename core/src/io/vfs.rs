//! A virtual file system with paths and common operations.

pub use local::*;

mod local;

/// A stream for reading from some [`VirtualPath`].
pub trait InputStream: std::io::BufRead + std::io::Seek {}

/// Blanket implementation of [`InputStream`].
impl<T: std::io::BufRead + std::io::Seek> InputStream for T {}

/// A stream for writing to some [`VirtualPath`].
pub trait OutputStream: std::io::Write + std::io::Seek {}

/// Blanket implementation of [`OutputStream`].
impl<T: std::io::Write + std::io::Seek> OutputStream for T {}

/// Represents a type capable of acting as a file system.
///
/// File systems are resolved from the scheme used in [`VirtualPath`]s, and
/// allow operations to be invoked against the underlying operating system and
/// file format.
pub trait FileSystem {
  /// Returns `true` if the given path can be handled by this [`FileSystem`].
  fn can_handle(&self, path: &VirtualPath) -> bool;

  // basic operations
  fn exists(&self, path: &VirtualPath) -> bool;
  fn is_file(&self, path: &VirtualPath) -> bool;
  fn is_directory(&self, path: &VirtualPath) -> bool;

  // read and write
  fn open_read(&self, path: &VirtualPath) -> crate::Result<Box<dyn InputStream>>;
  fn open_write(&self, path: &VirtualPath) -> crate::Result<Box<dyn OutputStream>>;
}

/// Static central manager for [`FileSystem`] implementations.
///
/// This is a singleton that is used to manage [`FileSystem`] implementations.
/// File systems can be registered here, and will be used subsequently for file
/// operations on [`VirtualPath`] instances.
pub struct FileSystemManager {
  file_systems: Vec<Box<dyn FileSystem>>,
}

// The manager is an unsafe singleton type
crate::impl_singleton!(FileSystemManager);

impl Default for FileSystemManager {
  fn default() -> Self {
    Self {
      file_systems: vec![Box::new(LocalFileSystem::new())],
    }
  }
}

impl FileSystemManager {
  /// Registers a new [`FileSystem`] with the manager.
  pub fn register(file_system: impl FileSystem + 'static) {
    let manager = FileSystemManager::instance();

    manager.file_systems.push(Box::new(file_system));
  }

  /// Finds the appropriate [`FileSystem`] for the given [`VirtualPath`].
  pub fn find_for_path(path: &VirtualPath) -> Option<&'static dyn FileSystem> {
    let manager = FileSystemManager::instance();

    manager.file_systems.iter().find(|fs| fs.can_handle(path)).map(|fs| fs.as_ref())
  }
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
#[derive(Clone)]
pub struct VirtualPath<'a> {
  scheme: &'a str,
  location: std::borrow::Cow<'a, str>,
}

impl<'a> VirtualPath<'a> {
  /// Parses the given string-like object into a path with scheme and location.
  pub fn parse(raw: &'a str) -> Self {
    let (scheme, location) = raw.split_once("://").unwrap_or(("local", raw));

    Self {
      scheme,
      location: std::borrow::Cow::Borrowed(location),
    }
  }

  /// Returns the file extension of the path.
  pub fn extension(&'a self) -> &'a str {
    if let Some(extension) = self.location.split('.').last() {
      extension
    } else {
      self.location.as_ref()
    }
  }

  /// Returns a new path with a different file extension.
  pub fn change_extension(&'a self, new_extension: &'a str) -> Self {
    let location = self.location.replace(self.extension(), new_extension);

    Self {
      scheme: self.scheme,
      location: std::borrow::Cow::Owned(location),
    }
  }

  /// Opens a reader for the given path.
  pub fn open_input_stream(&self) -> crate::Result<Box<dyn InputStream>> {
    let file_system = FileSystemManager::find_for_path(self).ok_or(anyhow::anyhow!("No file system found for scheme {}", self.scheme))?;
    let stream = file_system.open_read(self)?;

    Ok(Box::new(stream))
  }

  /// Opens a writer for the given path.
  pub fn open_output_stream(&self) -> crate::Result<Box<dyn OutputStream>> {
    let file_system = FileSystemManager::find_for_path(self).ok_or(anyhow::anyhow!("No file system found for scheme {}", self.scheme))?;
    let stream = file_system.open_write(self)?;

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

impl<'a> From<&VirtualPath<'a>> for VirtualPath<'a> {
  fn from(path: &VirtualPath<'a>) -> Self {
    path.clone()
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
  fn virtual_path_should_parse_simple_schemes() {
    let path = VirtualPath::parse("local://README.md");

    assert_eq!("local", path.scheme);
    assert_eq!("README.md", path.location);
    assert_eq!("local://README.md", format!("{:?}", path));
  }

  #[test]
  fn virtual_path_should_change_extension() {
    let old_path = VirtualPath::parse("local://README.md");
    let new_path = old_path.change_extension("txt");

    assert_eq!("local", new_path.scheme);
    assert_eq!("README.md", old_path.location);
    assert_eq!("README.txt", new_path.location);
  }
}
