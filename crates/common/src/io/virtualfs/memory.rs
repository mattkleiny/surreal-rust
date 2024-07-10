//! An in-memory [`FileSystem`] implementation.

use super::*;

/// A [`FileSystem`] for the in-memory file system.
#[derive(Default)]
pub struct MemoryFileSystem {}

impl FileSystem for MemoryFileSystem {
  fn can_handle(&self, path: &VirtualPath) -> bool {
    path.scheme == "mem" || path.scheme == "memory"
  }

  fn exists(&self, _path: &VirtualPath) -> bool {
    todo!()
  }

  fn is_file(&self, _path: &VirtualPath) -> bool {
    todo!()
  }

  fn is_directory(&self, _path: &VirtualPath) -> bool {
    todo!()
  }

  fn open_read(&self, _path: &VirtualPath) -> Result<Box<dyn InputStream>, FileSystemError> {
    todo!()
  }

  fn open_write(&self, _path: &VirtualPath) -> Result<Box<dyn OutputStream>, FileSystemError> {
    todo!()
  }
}
