//! A virtual file system with paths and common operations.

pub use local::*;
use macros::Singleton;
pub use memory::*;
use std::borrow::Borrow;

use crate as surreal;

mod local;
mod memory;

/// A stream for reading from some [`VirtualPath`].
pub trait InputStream: std::io::BufRead + std::io::Seek {
  fn read_u8(&mut self) -> crate::Result<u8>;
  fn read_u16(&mut self) -> crate::Result<u16>;
  fn read_u32(&mut self) -> crate::Result<u32>;
  fn read_u64(&mut self) -> crate::Result<u64>;
  fn read_u128(&mut self) -> crate::Result<u128>;
  fn read_usize(&mut self) -> crate::Result<usize>;
  fn read_i8(&mut self) -> crate::Result<i8>;
  fn read_i16(&mut self) -> crate::Result<i16>;
  fn read_i32(&mut self) -> crate::Result<i32>;
  fn read_i64(&mut self) -> crate::Result<i64>;
  fn read_i128(&mut self) -> crate::Result<i128>;
  fn read_isize(&mut self) -> crate::Result<isize>;
  fn read_f32(&mut self) -> crate::Result<f32>;
  fn read_f64(&mut self) -> crate::Result<f64>;
  fn read_string(&mut self) -> crate::Result<String>;
  fn read_bytes(&mut self, amount: usize) -> crate::Result<Vec<u8>>;
}

macro_rules! impl_read {
  ($self:expr, $buffer_size:expr, $result:ty) => {{
    let mut buffer = [0; $buffer_size];
    $self.read_exact(&mut buffer)?;
    Ok(<$result>::from_le_bytes(buffer))
  }};
}

/// Blanket implementation of [`InputStream`].
impl<T: std::io::BufRead + std::io::Seek> InputStream for T {
  fn read_u8(&mut self) -> crate::Result<u8> {
    impl_read!(self, 1, u8)
  }

  fn read_u16(&mut self) -> crate::Result<u16> {
    impl_read!(self, 2, u16)
  }

  fn read_u32(&mut self) -> crate::Result<u32> {
    impl_read!(self, 4, u32)
  }

  fn read_u64(&mut self) -> crate::Result<u64> {
    impl_read!(self, 8, u64)
  }

  fn read_u128(&mut self) -> crate::Result<u128> {
    impl_read!(self, 16, u128)
  }

  fn read_usize(&mut self) -> crate::Result<usize> {
    impl_read!(self, std::mem::size_of::<usize>(), usize)
  }

  fn read_i8(&mut self) -> crate::Result<i8> {
    impl_read!(self, 1, i8)
  }

  fn read_i16(&mut self) -> crate::Result<i16> {
    impl_read!(self, 2, i16)
  }

  fn read_i32(&mut self) -> crate::Result<i32> {
    impl_read!(self, 4, i32)
  }

  fn read_i64(&mut self) -> crate::Result<i64> {
    impl_read!(self, 8, i64)
  }

  fn read_i128(&mut self) -> crate::Result<i128> {
    impl_read!(self, 16, i128)
  }

  fn read_isize(&mut self) -> crate::Result<isize> {
    impl_read!(self, std::mem::size_of::<isize>(), isize)
  }

  fn read_f32(&mut self) -> crate::Result<f32> {
    impl_read!(self, 4, f32)
  }

  fn read_f64(&mut self) -> crate::Result<f64> {
    impl_read!(self, 8, f64)
  }

  fn read_string(&mut self) -> crate::Result<String> {
    let length = self.read_usize()?;
    let mut buffer = vec![0; length];

    self.read_exact(&mut buffer)?;

    Ok(String::from_utf8(buffer)?)
  }

  fn read_bytes(&mut self, amount: usize) -> crate::Result<Vec<u8>> {
    let mut buffer = vec![0; amount];
    self.read_exact(&mut buffer)?;

    Ok(buffer)
  }
}

/// A stream for writing to some [`VirtualPath`].
pub trait OutputStream: std::io::Write + std::io::Seek {
  fn write_u8(&mut self, value: u8) -> crate::Result<()>;
  fn write_u16(&mut self, value: u16) -> crate::Result<()>;
  fn write_u32(&mut self, value: u32) -> crate::Result<()>;
  fn write_u64(&mut self, value: u64) -> crate::Result<()>;
  fn write_u128(&mut self, value: u128) -> crate::Result<()>;
  fn write_usize(&mut self, value: usize) -> crate::Result<()>;
  fn write_i8(&mut self, value: i8) -> crate::Result<()>;
  fn write_i16(&mut self, value: i16) -> crate::Result<()>;
  fn write_i32(&mut self, value: i32) -> crate::Result<()>;
  fn write_i64(&mut self, value: i64) -> crate::Result<()>;
  fn write_i128(&mut self, value: i128) -> crate::Result<()>;
  fn write_isize(&mut self, value: isize) -> crate::Result<()>;
  fn write_f32(&mut self, value: f32) -> crate::Result<()>;
  fn write_f64(&mut self, value: f64) -> crate::Result<()>;
  fn write_string(&mut self, value: &str) -> crate::Result<()>;
  fn write_bytes(&mut self, value: &[u8]) -> crate::Result<()>;
}

macro_rules! impl_write {
  ($self:expr, $type:ty, $value:expr) => {{
    let buffer = <$type>::to_le_bytes($value);
    $self.write_all(&buffer)?;
    Ok(())
  }};
}

/// Blanket implementation of [`OutputStream`].
impl<T: std::io::Write + std::io::Seek> OutputStream for T {
  fn write_u8(&mut self, value: u8) -> crate::Result<()> {
    impl_write!(self, u8, value)
  }

  fn write_u16(&mut self, value: u16) -> crate::Result<()> {
    impl_write!(self, u16, value)
  }

  fn write_u32(&mut self, value: u32) -> crate::Result<()> {
    impl_write!(self, u32, value)
  }

  fn write_u64(&mut self, value: u64) -> crate::Result<()> {
    impl_write!(self, u64, value)
  }

  fn write_u128(&mut self, value: u128) -> crate::Result<()> {
    impl_write!(self, u128, value)
  }

  fn write_usize(&mut self, value: usize) -> crate::Result<()> {
    impl_write!(self, usize, value)
  }

  fn write_i8(&mut self, value: i8) -> crate::Result<()> {
    impl_write!(self, i8, value)
  }

  fn write_i16(&mut self, value: i16) -> crate::Result<()> {
    impl_write!(self, i16, value)
  }

  fn write_i32(&mut self, value: i32) -> crate::Result<()> {
    impl_write!(self, i32, value)
  }

  fn write_i64(&mut self, value: i64) -> crate::Result<()> {
    impl_write!(self, i64, value)
  }

  fn write_i128(&mut self, value: i128) -> crate::Result<()> {
    impl_write!(self, i128, value)
  }

  fn write_isize(&mut self, value: isize) -> crate::Result<()> {
    impl_write!(self, isize, value)
  }

  fn write_f32(&mut self, value: f32) -> crate::Result<()> {
    impl_write!(self, f32, value)
  }

  fn write_f64(&mut self, value: f64) -> crate::Result<()> {
    impl_write!(self, f64, value)
  }

  fn write_string(&mut self, value: &str) -> crate::Result<()> {
    self.write_usize(value.len())?;
    self.write_bytes(value.as_bytes())?;

    Ok(())
  }

  fn write_bytes(&mut self, value: &[u8]) -> crate::Result<()> {
    self.write_all(value)?;

    Ok(())
  }
}

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

  /// Watches for changes to the given [`VirtualPath`].
  ///
  /// Not all file systems implement this, and will return an error if they do not.
  fn watch(&self, path: &VirtualPath) -> crate::Result<Box<dyn FileWatcher>>;
}

/// A watcher for actions in a [`FileSystem`].
///
/// This is used to watch for changes in a [`FileSystem`], and is used to
/// implement [`FileSystem::watch`].
pub trait FileWatcher {}

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
      #[rustfmt::skip]
      file_systems: vec![
        Box::new(LocalFileSystem::new()),
        Box::new(MemoryFileSystem::new()),
      ],
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
  pub fn find(path: &VirtualPath) -> Option<&'static dyn FileSystem> {
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

  /// The scheme of the path.
  pub fn scheme(&'a self) -> &'a str {
    self.scheme
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

  /// Returns a new path with a different file extension.
  pub fn change_extension(&'a self, new_extension: &'a str) -> Self {
    let location = self.location.replace(self.extension(), new_extension);

    Self {
      scheme: self.scheme,
      location: std::borrow::Cow::Owned(location),
    }
  }

  /// Joins a [`VirtualPath`] relative to the current path.
  pub fn join(&self, relative: &str) -> Self {
    let mut path = self.location.to_string();

    if !path.ends_with("/") {
      path.push_str("/");
    }

    path.push_str(relative);

    Self {
      scheme: self.scheme,
      location: std::borrow::Cow::Owned(path),
    }
  }

  /// Determines if the path exists.
  pub fn exists(&self) -> crate::Result<bool> {
    let file_system = FileSystemManager::find(self).ok_or(anyhow::anyhow!("No file system found for scheme {}", self.scheme))?;

    Ok(file_system.exists(self))
  }

  /// Opens a reader for the given path.
  pub fn open_input_stream(&self) -> crate::Result<Box<dyn InputStream>> {
    let file_system = FileSystemManager::find(self).ok_or(anyhow::anyhow!("No file system found for scheme {}", self.scheme))?;
    let stream = file_system
      .open_read(self)
      .map_err(|error| surreal::anyhow!("Unable to open input stream for {}. Error {}", self, error))?;

    Ok(stream)
  }

  /// Opens a writer for the given path.
  pub fn open_output_stream(&self) -> crate::Result<Box<dyn OutputStream>> {
    let file_system = FileSystemManager::find(self).ok_or(anyhow::anyhow!("No file system found for scheme {}", self.scheme))?;
    let stream = file_system
      .open_write(self)
      .map_err(|error| surreal::anyhow!("Unable to open output stream for {}. Error {}", self, error))?;

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
}

impl<'a> std::fmt::Debug for VirtualPath<'a> {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Ok(write!(formatter, "{:}://{:}", self.scheme, self.location.replace("\\", "/"))?)
  }
}

impl<'a> std::fmt::Display for VirtualPath<'a> {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Ok(write!(formatter, "{:}://{:}", self.scheme, self.location.replace("\\", "/"))?)
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
