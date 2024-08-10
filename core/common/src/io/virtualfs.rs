use std::sync::RwLock;

pub use local::*;
pub use memory::*;

use super::{InputStream, OutputStream};
use crate::{Singleton, StringName, ToStringName};

mod local;
mod memory;

/// Represents a type capable of acting as a file system.
///
/// File systems are resolved from the scheme used in [`VirtualPath`]s, and
/// allow operations to be invoked against the underlying operating system and
/// file format.
pub trait FileSystem: Send + Sync {
  /// Returns `true` if the given path can be handled by this [`FileSystem`].
  fn can_handle(&self, path: &VirtualPath) -> bool;

  // basic operations
  fn exists(&self, path: &VirtualPath) -> bool;
  fn is_file(&self, path: &VirtualPath) -> bool;
  fn is_directory(&self, path: &VirtualPath) -> bool;
  fn files(&self, path: &VirtualPath) -> Vec<VirtualPath>;
  fn directories(&self, path: &VirtualPath) -> Vec<VirtualPath>;

  // read and write
  fn open_read(&self, path: &VirtualPath) -> Result<Box<dyn InputStream>, FileSystemError>;
  fn open_write(&self, path: &VirtualPath) -> Result<Box<dyn OutputStream>, FileSystemError>;
}

/// Static central manager for [`FileSystem`] implementations.
///
/// This is a singleton that is used to manage [`FileSystem`] implementations.
/// File systems can be registered here, and will be used subsequently for file
/// operations on [`VirtualPath`] instances.
#[derive(Singleton)]
pub struct FileSystemManager {
  file_systems: Vec<Box<dyn FileSystem>>,
}

impl Default for FileSystemManager {
  fn default() -> Self {
    Self {
      file_systems: vec![
        // Add the default file systems here.
        Box::<LocalFileSystem>::default(),
        Box::<MemoryFileSystem>::default(),
      ],
    }
  }
}

impl FileSystemManager {
  /// Registers a new [`FileSystem`] with the manager.
  pub fn register(file_system: impl FileSystem + 'static) {
    Self::instance().file_systems.push(Box::new(file_system));
  }

  /// Finds the appropriate [`FileSystem`] for the given [`VirtualPath`].
  pub fn with_filesystem<R>(path: &VirtualPath, body: impl FnOnce(&dyn FileSystem) -> R) -> R {
    for file_system in &Self::instance().file_systems {
      if file_system.can_handle(path) {
        return body(file_system.as_ref());
      }
    }

    panic!("No file system found for scheme {}", path.scheme());
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
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct VirtualPath {
  scheme: StringName,
  location: String,
}

impl VirtualPath {
  /// Creates a new [`VirtualPath`] from a raw string.
  pub fn new(raw: &str) -> Self {
    let (scheme, location) = raw.split_once("://").unwrap_or(("local", raw));

    VirtualPath {
      scheme: scheme.to_string_name(),
      location: location.to_string(),
    }
  }

  /// The scheme of the path.
  pub fn scheme(&self) -> &StringName {
    &self.scheme
  }

  /// The location of the path.
  pub fn location(&self) -> &str {
    &self.location
  }

  /// Returns the file extension of the path.
  pub fn extension(&self) -> &str {
    self.location.split('.').last().unwrap_or_default()
  }

  /// Determines if the path has the given extension.
  pub fn has_extension(&self, extension: &str) -> bool {
    self.extension() == extension
  }

  /// Returns a new path with a different file extension appended.
  pub fn append_extension(&self, new_extension: &str) -> Self {
    Self {
      scheme: self.scheme,
      location: format!("{:}.{:}", self.location, new_extension),
    }
  }

  /// Returns a new path with a different file extension.
  pub fn change_extension(&self, new_extension: &str) -> Self {
    Self {
      scheme: self.scheme,
      location: self.location.replace(self.extension(), new_extension),
    }
  }

  /// Joins a [`VirtualPath`] relative to the current path.
  pub fn join(&self, relative: &str) -> Self {
    let mut path = self.location.to_string();

    if !path.ends_with('/') {
      path.push('/');
    }

    path.push_str(relative);

    Self {
      scheme: self.scheme,
      location: path,
    }
  }

  /// Determines if the path exists.
  pub fn exists(&self) -> bool {
    FileSystemManager::with_filesystem(self, |file_system| file_system.exists(self))
  }

  /// Opens a reader for the given path.
  pub fn open_input_stream(&self) -> Result<Box<dyn InputStream>, FileSystemError> {
    FileSystemManager::with_filesystem(self, |file_system| file_system.open_read(self))
  }

  /// Opens a writer for the given path.
  pub fn open_output_stream(&self) -> Result<Box<dyn OutputStream>, FileSystemError> {
    FileSystemManager::with_filesystem(self, |file_system| file_system.open_write(self))
  }

  /// Attempts to read all bytes from the given path.
  pub fn read_all_bytes(&self) -> Result<Vec<u8>, FileSystemError> {
    let stream = self.open_input_stream()?;

    Ok(stream.to_buffer()?)
  }

  /// Attempts to read all bytes from the given path asynchronously.
  pub async fn read_all_bytes_async(&self) -> Result<Vec<u8>, FileSystemError> {
    let stream = self.open_input_stream()?;

    Ok(stream.to_buffer_async().await?)
  }

  /// Attempts to read all text from the given path.
  pub fn read_all_text(&self) -> Result<String, FileSystemError> {
    let stream = self.open_input_stream()?;

    Ok(stream.to_string()?)
  }

  /// Attempts to read all text from the given path asynchronously.
  pub async fn read_all_text_async(&self) -> Result<String, FileSystemError> {
    let stream = self.open_input_stream()?;

    Ok(stream.to_string_async().await?)
  }

  /// Finds all files in the given directory.
  pub fn files(&self) -> Vec<VirtualPath> {
    FileSystemManager::with_filesystem(self, |file_system| file_system.files(self))
  }

  pub fn directories(&self) -> Vec<VirtualPath> {
    FileSystemManager::with_filesystem(self, |file_system| file_system.directories(self))
  }
}

impl std::fmt::Debug for VirtualPath {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    Ok(write!(f, "{:}://{:}", self.scheme, self.location.replace('\\', "/"))?)
  }
}

impl std::fmt::Display for VirtualPath {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    Ok(write!(f, "{:}://{:}", self.scheme, self.location.replace('\\', "/"))?)
  }
}

/// Represents a type that can be converted into a [`VirtualPath`].
pub trait ToVirtualPath {
  /// Converts the type into a [`VirtualPath`].
  fn to_virtual_path(self) -> VirtualPath;
}

impl<R: AsRef<str>> ToVirtualPath for R {
  #[inline]
  fn to_virtual_path(self) -> VirtualPath {
    VirtualPath::new(self.as_ref())
  }
}

impl ToVirtualPath for VirtualPath {
  #[inline]
  fn to_virtual_path(self) -> VirtualPath {
    self.clone()
  }
}

impl ToVirtualPath for &VirtualPath {
  #[inline]
  fn to_virtual_path(self) -> VirtualPath {
    self.clone()
  }
}

/// A potential error that can occur when interacting with a [`FileSystem`].
#[derive(Debug)]
pub enum FileSystemError {
  NotFound,
  IoError(std::io::Error),
  StreamError(super::StreamError),
}

impl From<std::io::Error> for FileSystemError {
  #[inline]
  fn from(error: std::io::Error) -> Self {
    Self::IoError(error)
  }
}

impl From<super::StreamError> for FileSystemError {
  #[inline]
  fn from(error: super::StreamError) -> Self {
    Self::StreamError(error)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn virtual_path_should_parse_simple_schemes() {
    let path = "local://README.md".to_virtual_path();

    assert_eq!("local", path.scheme);
    assert_eq!("README.md", path.location);
    assert_eq!("local://README.md", format!("{path:?}"));
  }

  #[test]
  fn virtual_path_should_change_extension() {
    let old_path = "local://README.md".to_virtual_path();
    let new_path = old_path.change_extension("txt");

    assert_eq!("local", new_path.scheme);
    assert_eq!("README.md", old_path.location);
    assert_eq!("README.txt", new_path.location);
  }
}
