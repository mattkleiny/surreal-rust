//! A virtual file system.

use std::borrow::Borrow;

use once_cell::sync::Lazy;

mod local;
mod resource;

/// The statically registered file systems for the application.
static FILE_SYSTEMS: Lazy<Vec<Box<dyn FileSystem>>> = Lazy::new(|| {
  vec![
    Box::new(local::LocalFileSystem::new()),
    Box::new(resource::ResourceFileSystem::new()),
  ]
});

/// Represents a result in the VFS system.
pub type VFSResult<T> = std::result::Result<T, Error>;

/// Abstractly represents a file system.
pub trait FileSystem: Send + Sync {
  fn schemes(&self) -> &[&'static str];

  fn copy_to(&self, from: &Path, to: &Path) -> VFSResult<()>;
  fn move_to(&self, from: &Path, to: &Path) -> VFSResult<()>;
  fn delete(&self, from: &Path, to: &Path) -> VFSResult<()>;
}

/// Represents a path in a virtual file system.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Path {
  pub path: String,
  pub scheme: String,
}

impl Path {
  // basic operations
  pub fn copy_to(&self, to: &Path) -> VFSResult<()> {
    self.filesystem()?.copy_to(&self, &to)
  }

  pub fn move_to(&self, to: &Path) -> VFSResult<()> {
    self.filesystem()?.move_to(&self, &to)
  }

  pub fn delete(&self, to: &Path) -> VFSResult<()> {
    self.filesystem()?.delete(&self, &to)
  }

  // reading
  pub fn read_raw(&self) -> VFSResult<Vec<u8>> {
    unimplemented!()
  }

  pub fn read_text(&self) -> VFSResult<String> {
    unimplemented!()
  }

  // writing
  pub fn write_raw(&self, buffer: &[u8]) -> VFSResult<()> {
    unimplemented!()
  }

  pub fn write_text(&self, buffer: impl AsRef<str>) -> VFSResult<()> {
    unimplemented!()
  }

  /// Accesses the file system represented by the path.
  pub fn filesystem(&self) -> VFSResult<&dyn FileSystem> {
    for filesystem in FILE_SYSTEMS.iter() {
      for scheme in filesystem.schemes() {
        if scheme.eq(&self.scheme) {
          return Ok(filesystem.borrow());
        }
      }
    }

    Err(Error::UnknownFileSystem)
  }
}

impl AsRef<Path> for &'static str {
  fn as_ref(&self) -> &Path {
    unimplemented!()
  }
}

/// Represents an error with the VFS.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
  UnknownFileSystem
}

impl From<Error> for crate::Error {
  fn from(_: Error) -> Self {
    crate::Error::VFS
  }
}