//! A packed file system.

/// A packed file.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PakFile {
  pub headers: Vec<PakFileHeader>,
}

/// Header for a Pak file.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PakFileHeader {
  pub name: String,
  pub offset: u64,
  pub length: u64,
  pub creation_time: u64,
  pub modification_time: u64,
  pub file_type: PakFileType,
}

/// The type of the Pak file.
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PakFileType {
  #[default]
  File,
  Directory,
  Symlink,
}
