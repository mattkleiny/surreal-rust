//! A virtual file system with paths and common operations.

use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::io::{BufRead, Read, Seek, Write};

pub use local::*;

mod local;

thread_local! {
  /// Top-level registry of all file systems.
  static REGISTRY: std::sync::Mutex<UnsafeCell<FileSystemRegistry>> = std::sync::Mutex::new(UnsafeCell::new(FileSystemRegistry::new()));
}

// TODO: remove this unsafe hackery?

/// Registers a new file system for the given virtual path scheme.
pub fn register_file_system(scheme: &str, file_system: impl FileSystem + 'static) {
  REGISTRY.with(|registry| {
    let registry = unsafe { &mut *registry.lock().unwrap().get() };

    registry.file_systems.insert(scheme.to_string(), Box::new(file_system));
  });
}

/// Attempts to get the file system for the given virtual path scheme.
pub fn get_file_system(scheme: &str) -> crate::Result<&'static dyn FileSystem> {
  REGISTRY.with(|registry| {
    let registry = unsafe { &*registry.lock().unwrap().get() };

    match registry.file_systems.get(scheme) {
      Some(file_system) => Ok(file_system.as_ref()),
      None => Err(anyhow::anyhow!("No file system exists for {:}", scheme)),
    }
  })
}

/// Allows registering file systems for different virtual path schemes.
pub struct FileSystemRegistry {
  file_systems: HashMap<String, Box<dyn FileSystem>>,
}

impl FileSystemRegistry {
  /// Builds a new file system registry.
  pub fn new() -> Self {
    Self {
      file_systems: HashMap::new(),
    }
  }
}

/// Represents a type capable of acting as a file system.
pub trait FileSystem {
  // basic operations
  fn exists(&self, path: &VirtualPath) -> bool;
  fn is_file(&self, path: &VirtualPath) -> bool;
  fn is_directory(&self, path: &VirtualPath) -> bool;

  // read and write
  fn open_read(&self, path: &VirtualPath) -> crate::Result<InputStream>;
  fn open_write(&self, path: &VirtualPath) -> crate::Result<OutputStream>;
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
  pub fn open_input_stream(&self) -> crate::Result<InputStream> {
    let file_system = get_file_system(self.scheme)?;

    Ok(file_system.open_read(self)?)
  }

  /// Opens a writer for the given path.
  pub fn open_output_stream(&self) -> crate::Result<OutputStream> {
    let file_system = get_file_system(self.scheme)?;

    Ok(file_system.open_write(self)?)
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

/// Allows a type to be converted to a [`VirtualPath`].
pub trait AsVirtualPath {
  fn as_virtual_path(&self) -> VirtualPath;
}

impl<'a> AsVirtualPath for VirtualPath<'a> {
  fn as_virtual_path(&self) -> VirtualPath<'a> {
    *self
  }
}

impl AsVirtualPath for &str {
  fn as_virtual_path(&self) -> VirtualPath {
    VirtualPath::parse(self)
  }
}

/// A stream for reading from some [`VirtualPath`].
pub struct InputStream {}

impl Seek for InputStream {
  fn seek(&mut self, position: std::io::SeekFrom) -> std::io::Result<u64> {
    todo!()
  }
}

impl Read for InputStream {
  fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
    todo!()
  }
}

impl BufRead for InputStream {
  fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
    todo!()
  }

  fn consume(&mut self, amount: usize) {
    todo!()
  }
}

/// A stream for writing to some [`VirtualPath`].
pub struct OutputStream {}

impl Seek for OutputStream {
  fn seek(&mut self, position: std::io::SeekFrom) -> std::io::Result<u64> {
    todo!()
  }
}

impl Write for OutputStream {
  fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
    todo!()
  }

  fn flush(&mut self) -> std::io::Result<()> {
    todo!()
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
