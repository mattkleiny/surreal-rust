use crate::{FileSystemError, InputStream, StreamError};

/// Allows reading from a ZIP archive.
pub struct ZipArchive<'a> {
  reader: zip::ZipArchive<&'a mut dyn InputStream>,
}

/// A possible error when interacting with ZIP archives.
#[derive(Debug)]
pub enum ZipError {
  GeneralIoError,
  FileNotFound,
}

impl<'a> ZipArchive<'a> {
  /// Creates a new [`ZipArchive`] from the given [`InputStream`].
  pub fn from_stream(stream: &'a mut dyn InputStream) -> Result<Self, ZipError> {
    Ok(Self {
      reader: zip::read::ZipArchive::new(stream)?,
    })
  }

  /// Returns true if the archive is empty.
  pub fn is_empty(&self) -> bool {
    self.reader.len() == 0
  }

  /// Returns the number of files in the archive.
  pub fn len(&self) -> usize {
    self.reader.len()
  }

  /// Returns the file at the given index.
  pub fn get_file_by_index(&mut self, index: usize) -> Result<zip::read::ZipFile, ZipError> {
    Ok(self.reader.by_index(index)?)
  }

  /// Returns the file with the given name.
  pub fn get_file_by_name(&mut self, name: &str) -> Result<zip::read::ZipFile, ZipError> {
    Ok(self.reader.by_name(name)?)
  }
}

impl From<FileSystemError> for ZipError {
  #[inline]
  fn from(_: FileSystemError) -> Self {
    Self::GeneralIoError
  }
}

impl From<ZipError> for StreamError {
  #[inline]
  fn from(_: ZipError) -> Self {
    Self::InvalidData
  }
}

impl From<zip::result::ZipError> for ZipError {
  #[inline]
  fn from(value: zip::result::ZipError) -> Self {
    match value {
      zip::result::ZipError::FileNotFound => Self::FileNotFound,
      _ => Self::GeneralIoError,
    }
  }
}
