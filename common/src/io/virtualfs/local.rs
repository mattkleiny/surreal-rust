//! The local [`FileSystem`] implementation.

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

  fn open_read(&self, path: &VirtualPath) -> crate::Result<Box<dyn InputStream>> {
    let file = OpenOptions::new()
      .read(true)
      .write(false)
      .create(false)
      .open(to_path(path))?;

    Ok(Box::new(std::io::BufReader::new(file)))
  }

  fn open_write(&self, path: &VirtualPath) -> crate::Result<Box<dyn OutputStream>> {
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
  PathBuf::from(path.location.as_ref())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_read_file_from_disk() {
    let path = VirtualPath::from("local://../assets/fonts/bitboy8_v1.otf");
    let bytes = path.read_all_bytes().unwrap();

    assert!(!bytes.is_empty());
  }
}
