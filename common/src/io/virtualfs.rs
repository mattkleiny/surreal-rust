//! A virtual file system with paths and common operations.

use std::{
  borrow::{Borrow, Cow},
  sync::RwLock,
};

pub use local::*;
pub use memory::*;

mod local;
mod memory;

use anyhow::anyhow;
use macros::Singleton;

use super::{InputStream, OutputStream};

/// Represents a type capable of acting as a file system.
///
/// File systems are resolved from the scheme used in [`VirtualPath`]s, and
/// allow operations to be invoked against the underlying operating system and
/// file format.
pub trait FileSystem: Send + Sync + 'static {
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
#[derive(Singleton)]
pub struct FileSystemManager {
  file_systems: RwLock<Vec<Box<dyn FileSystem>>>,
}

impl Default for FileSystemManager {
  fn default() -> Self {
    Self {
      #[rustfmt::skip]
      file_systems: RwLock::new(vec![
        Box::new(LocalFileSystem::default()),
        Box::new(MemoryFileSystem::default()),
      ]),
    }
  }
}

impl FileSystemManager {
  /// Registers a new [`FileSystem`] with the manager.
  pub fn register(file_system: impl FileSystem + 'static) {
    let manager = Self::instance();
    let mut file_systems = manager.file_systems.write().unwrap();

    file_systems.push(Box::new(file_system));
  }

  /// Finds the appropriate [`FileSystem`] for the given [`VirtualPath`].
  pub fn find(path: &VirtualPath) -> Option<&'static dyn FileSystem> {
    let manager = Self::instance();
    let file_systems = manager.file_systems.read().unwrap();

    for file_system in file_systems.iter() {
      if file_system.can_handle(path) {
        return Some(file_system.as_ref());
      }
    }

    None
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
  scheme: Cow<'a, str>,
  location: Cow<'a, str>,
}

impl<'a> VirtualPath<'a> {
  /// The scheme of the path.
  pub fn scheme(&'a self) -> &'a str {
    self.scheme.borrow()
  }

  /// The location of the path.
  pub fn location(&'a self) -> &'a str {
    self.location.borrow()
  }

  /// Returns the file extension of the path.
  pub fn extension(&'a self) -> &'a str {
    if let Some(extension) = self.location.split('.').last() {
      extension
    } else {
      ""
    }
  }

  /// Returns a new path with a different file extension appended.
  pub fn append_extension(&'a self, new_extension: &'a str) -> Self {
    let location = self.location.to_owned();
    let location = format!("{:}.{:}", location, new_extension);

    Self {
      scheme: self.scheme.clone(),
      location: Cow::Owned(location),
    }
  }

  /// Returns a new path with a different file extension.
  pub fn change_extension(&'a self, new_extension: &'a str) -> Self {
    let location = self.location.replace(self.extension(), new_extension);

    Self {
      scheme: self.scheme.clone(),
      location: Cow::Owned(location),
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
      scheme: self.scheme.clone(),
      location: Cow::Owned(path),
    }
  }

  /// Determines if the path exists.
  pub fn exists(&self) -> crate::Result<bool> {
    let file_system =
      FileSystemManager::find(self).ok_or(anyhow!("No file system found for scheme {}", self.scheme))?;

    Ok(file_system.exists(self))
  }

  /// Opens a reader for the given path.
  pub fn open_input_stream(&self) -> crate::Result<Box<dyn InputStream>> {
    let file_system =
      FileSystemManager::find(self).ok_or(anyhow!("No file system found for scheme {}", self.scheme))?;

    let stream = file_system
      .open_read(self)
      .map_err(|error| anyhow!("Unable to open input stream for {}. Error {}", self, error))?;

    Ok(stream)
  }

  /// Opens a writer for the given path.
  pub fn open_output_stream(&self) -> crate::Result<Box<dyn OutputStream>> {
    let file_system =
      FileSystemManager::find(self).ok_or(anyhow!("No file system found for scheme {}", self.scheme))?;

    let stream = file_system
      .open_write(self)
      .map_err(|error| anyhow!("Unable to open output stream for {}. Error {}", self, error))?;

    Ok(stream)
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

  /// Converts the path to a string.
  #[inline]
  pub fn to_string(&self) -> String {
    format!("{:}://{:}", self.scheme, self.location)
  }
}

impl<'a> std::fmt::Debug for VirtualPath<'a> {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Ok(write!(
      formatter,
      "{:}://{:}",
      self.scheme,
      self.location.replace('\\', "/")
    )?)
  }
}

impl<'a> std::fmt::Display for VirtualPath<'a> {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Ok(write!(
      formatter,
      "{:}://{:}",
      self.scheme,
      self.location.replace('\\', "/")
    )?)
  }
}

impl<'a> From<&VirtualPath<'a>> for VirtualPath<'a> {
  fn from(path: &VirtualPath<'a>) -> Self {
    path.clone()
  }
}

impl<'a> From<&'a str> for VirtualPath<'a> {
  fn from(value: &'a str) -> Self {
    let (scheme, location) = value.split_once("://").unwrap_or(("local", value));

    Self {
      scheme: Cow::Borrowed(scheme),
      location: Cow::Borrowed(location),
    }
  }
}

impl<'a> From<&'a String> for VirtualPath<'a> {
  fn from(value: &'a String) -> Self {
    let (scheme, location) = value.split_once("://").unwrap_or(("local", value));

    Self {
      scheme: Cow::Borrowed(scheme),
      location: Cow::Borrowed(location),
    }
  }
}

impl<'a> From<String> for VirtualPath<'a> {
  /// Parses the given string and takes ownership of it's contents.
  fn from(value: String) -> Self {
    let (scheme, location) = value.split_once("://").unwrap_or(("local", &value));

    Self {
      scheme: Cow::Owned(scheme.to_string()),
      location: Cow::Owned(location.to_string()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn virtual_path_should_parse_simple_schemes() {
    let path = VirtualPath::from("local://README.md");

    assert_eq!("local", path.scheme);
    assert_eq!("README.md", path.location);
    assert_eq!("local://README.md", format!("{path:?}"));
  }

  #[test]
  fn virtual_path_should_change_extension() {
    let old_path = VirtualPath::from("local://README.md");
    let new_path = old_path.change_extension("txt");

    assert_eq!("local", new_path.scheme);
    assert_eq!("README.md", old_path.location);
    assert_eq!("README.txt", new_path.location);
  }
}
