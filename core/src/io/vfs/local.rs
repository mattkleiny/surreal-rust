//! The local [`FileSystem`] implementation.

use std::{
  fs::OpenOptions,
  path::{Path, PathBuf},
};

use super::*;

/// A [`FileSystem`] for the local OS file system.
pub struct LocalFileSystem {
  root: PathBuf,
}

impl Default for LocalFileSystem {
  fn default() -> Self {
    Self::new()
  }
}

impl LocalFileSystem {
  pub fn new() -> Self {
    Self {
      root: std::env::current_dir().expect("Unable to get current directory"),
    }
  }
}

impl FileSystem for LocalFileSystem {
  fn can_handle(&self, path: &VirtualPath) -> bool {
    path.scheme == "local" || path.scheme == "file"
  }

  fn exists(&self, path: &VirtualPath) -> bool {
    to_path(&self.root, path).exists()
  }

  fn is_file(&self, path: &VirtualPath) -> bool {
    to_path(&self.root, path).is_file()
  }

  fn is_directory(&self, path: &VirtualPath) -> bool {
    to_path(&self.root, path).is_dir()
  }

  fn open_read(&self, path: &VirtualPath) -> crate::Result<Box<dyn InputStream>> {
    let file = OpenOptions::new()
      .read(true)
      .write(false)
      .create(false)
      .open(to_path(&self.root, path))?;

    Ok(Box::new(std::io::BufReader::new(file)))
  }

  fn open_write(&self, path: &VirtualPath) -> crate::Result<Box<dyn OutputStream>> {
    let file = OpenOptions::new()
      .read(false)
      .write(true)
      .create(true)
      .truncate(true)
      .open(to_path(&self.root, path))?;

    Ok(Box::new(std::io::BufWriter::new(file)))
  }
}

/// Converts a [`VirtualPath`] into a [`Path`].
fn to_path(root: &Path, path: &VirtualPath) -> PathBuf {
  let mut local_path = PathBuf::new();

  local_path.push(root);
  local_path.push(path.location.as_ref());

  local_path
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn read_from_local_file_system() {
    let path = VirtualPath::from("local://../rustfmt.toml");
    let text = path.read_all_text().expect("Failed to read test file");

    assert!(!text.is_empty());
  }
}
