//! An in-memory [`FileSystem`] implementation.

use std::cell::UnsafeCell;
use std::collections::HashMap;

use super::*;

/// A [`FileSystem`] for the in-memory file system.
pub struct MemoryFileSystem {
  // TODO: Use a safe data structure?
  storage: UnsafeCell<MemoryStorage>,
}

impl MemoryFileSystem {
  pub fn new() -> Self {
    Self {
      storage: UnsafeCell::new(MemoryStorage::default()),
    }
  }
}

/// Internal dynamically-checked storage for the [`MemoryFileSystem`].
#[derive(Default)]
struct MemoryStorage {
  files: HashMap<String, MemoryFile>,
}

/// Represent a file in [`MemoryFileSystem`].
#[derive(Default)]
struct MemoryFile {
  data: Vec<u8>,
}

impl FileSystem for MemoryFileSystem {
  fn can_handle(&self, path: &VirtualPath) -> bool {
    path.scheme == "mem" || path.scheme == "memory"
  }

  fn exists(&self, path: &VirtualPath) -> bool {
    let storage = unsafe { &*self.storage.get() };

    storage.files.contains_key(path.location.as_ref())
  }

  fn is_file(&self, path: &VirtualPath) -> bool {
    let storage = unsafe { &*self.storage.get() };

    storage.files.contains_key(path.location.as_ref())
  }

  fn is_directory(&self, path: &VirtualPath) -> bool {
    !self.is_file(path)
  }

  fn open_read(&self, path: &VirtualPath) -> crate::Result<Box<dyn InputStream>> {
    let storage = unsafe { &*self.storage.get() };
    let file = storage
      .files
      .get(path.location.as_ref())
      .ok_or(anyhow::anyhow!("File not found: {}", path))?;

    Ok(Box::new(std::io::Cursor::new(file.data.clone())))
  }

  fn open_write(&self, path: &VirtualPath) -> crate::Result<Box<dyn OutputStream>> {
    let storage = unsafe { &mut *self.storage.get() };
    let file = storage.files.entry(path.location.to_string()).or_default();

    file.data.clear(); // truncate existing file

    Ok(Box::new(std::io::Cursor::new(&mut file.data)))
  }

  fn watch(&self, _path: &VirtualPath) -> crate::Result<Box<dyn FileWatcher>> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn memory_file_system_should_read_and_write_basic_data() {
    let file_system = MemoryFileSystem::new();

    let path = VirtualPath::parse("memory://test.txt");
    let mut stream = file_system.open_write(&path).unwrap();
    stream.write_string("Hello, world!").unwrap();

    let mut stream = file_system.open_read(&path).unwrap();
    let string = stream.read_string().unwrap();

    assert_eq!(string, "Hello, world!");
  }
}
