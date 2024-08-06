use super::*;
use crate::FastHashMap;

/// A [`FileSystem`] for the in-memory file system.
#[derive(Default)]
pub struct MemoryFileSystem {
  files: RwLock<FastHashMap<String, MemoryFile>>,
}

/// A file in memory.
#[derive(Default)]
struct MemoryFile {
  _data: Vec<u8>,
}

impl FileSystem for MemoryFileSystem {
  fn can_handle(&self, path: &VirtualPath) -> bool {
    path.scheme == "mem" || path.scheme == "memory"
  }

  fn exists(&self, path: &VirtualPath) -> bool {
    self.files.read().unwrap().contains_key(&path.location)
  }

  fn is_file(&self, path: &VirtualPath) -> bool {
    self.exists(path)
  }

  fn is_directory(&self, _path: &VirtualPath) -> bool {
    false // we don't have directories in memory
  }

  fn files(&self, _path: &VirtualPath) -> Vec<VirtualPath> {
    todo!()
  }

  fn directories(&self, _path: &VirtualPath) -> Vec<VirtualPath> {
    todo!()
  }

  fn open_read(&self, _path: &VirtualPath) -> Result<Box<dyn InputStream>, FileSystemError> {
    todo!()
  }

  fn open_write(&self, _path: &VirtualPath) -> Result<Box<dyn OutputStream>, FileSystemError> {
    todo!()
  }
}
