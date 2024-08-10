use std::{fs::OpenOptions, path::PathBuf};

use super::*;

/// A [`FileSystem`] for the local OS file system.
#[derive(Default)]
pub struct LocalFileSystem {}

impl FileSystem for LocalFileSystem {
  fn can_handle(&self, path: &VirtualPath) -> bool {
    path.scheme == "local" || path.scheme == "file"
  }

  fn exists(&self, path: &VirtualPath) -> bool {
    to_path(path).exists()
  }

  fn is_file(&self, path: &VirtualPath) -> bool {
    to_path(path).is_file()
  }

  fn is_directory(&self, path: &VirtualPath) -> bool {
    to_path(path).is_dir()
  }

  fn files(&self, path: &VirtualPath) -> Vec<VirtualPath> {
    let path = to_path(path);
    let mut results = Vec::new();

    for entry in path.read_dir().unwrap() {
      let entry = entry.unwrap();
      let path = entry.path();

      if path.is_file() {
        results.push(VirtualPath::new(path.to_string_lossy().as_ref()));
      }
    }

    results
  }

  fn directories(&self, path: &VirtualPath) -> Vec<VirtualPath> {
    let path = to_path(path);
    let mut results = Vec::new();

    for entry in path.read_dir().unwrap() {
      let entry = entry.unwrap();
      let path = entry.path();

      if path.is_dir() {
        results.push(VirtualPath::new(path.to_string_lossy().as_ref()));
      }
    }

    results
  }

  fn open_read(&self, path: &VirtualPath) -> Result<Box<dyn InputStream>, FileSystemError> {
    let file = OpenOptions::new()
      .read(true)
      .write(false)
      .create(false)
      .open(to_path(path))?;

    Ok(Box::new(std::io::BufReader::new(file)))
  }

  fn open_write(&self, path: &VirtualPath) -> Result<Box<dyn OutputStream>, FileSystemError> {
    let file = OpenOptions::new()
      .read(false)
      .write(true)
      .create(true)
      .truncate(true)
      .open(to_path(path))?;

    Ok(Box::new(std::io::BufWriter::new(file)))
  }
}

/// Converts a [`VirtualPath`] into a [`Path`].
#[inline(always)]
fn to_path(path: &VirtualPath) -> PathBuf {
  PathBuf::from(&path.location)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_read_file_from_disk() {
    let path = "local://../../assets/fonts/bitboy8_v1.otf".to_virtual_path();
    let bytes = path.read_all_bytes().unwrap();

    assert!(!bytes.is_empty());
  }
}
