//! A virtual file system.

mod local;
mod resource;

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
    unimplemented!()
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