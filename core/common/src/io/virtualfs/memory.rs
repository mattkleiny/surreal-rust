use std::io::{Read, Seek, SeekFrom, Write};

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
  data: Vec<u8>,
}

/// A file stream for memory files.
struct MemoryFileStream {
  pointer: *const u8,
  length: usize,
  position: usize,
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

  fn open_read(&self, path: &VirtualPath) -> Result<Box<dyn InputStream>, FileSystemError> {
    let lock = self.files.read().unwrap();
    let file = lock.get(&path.location).ok_or(FileSystemError::NotFound)?;

    let stream = MemoryFileStream {
      pointer: file.data.as_ptr(),
      length: file.data.len(),
      position: 0,
    };

    Ok(Box::new(std::io::BufReader::new(stream)))
  }

  fn open_write(&self, path: &VirtualPath) -> Result<Box<dyn OutputStream>, FileSystemError> {
    let mut lock = self.files.write().unwrap();
    let file = lock.entry(path.location.to_string()).or_default();

    let stream = MemoryFileStream {
      pointer: file.data.as_ptr(),
      length: file.data.len(),
      position: 0,
    };

    Ok(Box::new(stream))
  }
}

impl Read for MemoryFileStream {
  fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
    let remaining = self.length - self.position;

    if remaining == 0 {
      return Ok(0);
    }

    let to_read = std::cmp::min(remaining, buffer.len());
    let slice = unsafe { std::slice::from_raw_parts(self.pointer.add(self.position), to_read) };

    buffer[..to_read].copy_from_slice(slice);
    self.position += to_read;

    Ok(to_read)
  }
}

impl Write for MemoryFileStream {
  fn write(&mut self, _buffer: &[u8]) -> std::io::Result<usize> {
    todo!()
  }

  fn flush(&mut self) -> std::io::Result<()> {
    Ok(()) // no-op
  }
}

impl Seek for MemoryFileStream {
  fn seek(&mut self, position: SeekFrom) -> std::io::Result<u64> {
    match position {
      SeekFrom::Start(offset) => {
        self.position = offset as usize;
      }
      SeekFrom::End(offset) => {
        self.position = self.length + offset as usize;
      }
      SeekFrom::Current(offset) => {
        self.position += offset as usize;
      }
    }

    Ok(self.position as u64)
  }
}
