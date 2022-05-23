//! The local file system implementation.

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::io::{FileResult, FileSystem, InputStream, OutputStream, VirtualPath};

/// Converts a [`VirtualPath`] into a [`Path`].
fn to_path(root: &Path, path: &VirtualPath) -> PathBuf {
  let mut local_path = PathBuf::new();

  local_path.push(root);
  local_path.push(path.location);

  local_path
}

/// A virtual file system implementation that reads/writes the real local file system.
pub struct LocalFileSystem {
  root: PathBuf,
}

impl LocalFileSystem {
  pub fn new() -> Self {
    Self {
      root: std::env::current_dir().expect("Unable to get current directory")
    }
  }
}

impl FileSystem for LocalFileSystem {
  type InputStream = LocalInputStream;
  type OutputStream = LocalOutputStream;

  fn exists(&self, path: &VirtualPath) -> bool {
    to_path(&self.root, path).exists()
  }

  fn is_file(&self, path: &VirtualPath) -> bool {
    to_path(&self.root, path).is_file()
  }

  fn is_directory(&self, path: &VirtualPath) -> bool {
    to_path(&self.root, path).is_dir()
  }

  fn open_read(&self, path: &VirtualPath) -> FileResult<Self::InputStream> {
    let file = OpenOptions::new()
      .read(true)
      .write(false)
      .open(to_path(&self.root, path))?;

    Ok(Self::InputStream {
      reader: BufReader::new(file)
    })
  }

  fn open_write(&self, path: &VirtualPath) -> FileResult<Self::OutputStream> {
    let file = OpenOptions::new()
      .read(false)
      .write(true)
      .open(to_path(&self.root, path))?;

    Ok(Self::OutputStream {
      writer: BufWriter::new(file)
    })
  }
}

/// A stream for reading from the local file system.
pub struct LocalInputStream {
  reader: BufReader<File>,
}

impl Seek for LocalInputStream {
  fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
    self.reader.seek(pos)
  }
}

impl Read for LocalInputStream {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    self.reader.read(buf)
  }
}

impl BufRead for LocalInputStream {
  fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
    self.reader.fill_buf()
  }

  fn consume(&mut self, amount: usize) {
    self.reader.consume(amount)
  }
}

impl InputStream for LocalInputStream {}

/// A stream for writing to the local file system.
pub struct LocalOutputStream {
  writer: BufWriter<File>,
}

impl Seek for LocalOutputStream {
  fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
    self.writer.seek(pos)
  }
}

impl Write for LocalOutputStream {
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    self.writer.write(buf)
  }

  fn flush(&mut self) -> std::io::Result<()> {
    self.writer.flush()
  }
}

impl OutputStream for LocalOutputStream {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn read_from_local_file_system() {
    let path = VirtualPath::parse("local://rustfmt.toml");
    let text = path.read_all_text().expect("Failed to read test file");

    assert!(text.len() > 0);
  }
}