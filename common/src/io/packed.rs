//! A packed file system.

/// A packed file.
pub struct PakFile {}

/// Header for a Pak file.
#[repr(C)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct PakFileHeader {
  pub name: String,
  pub offset: u64,
  pub length: u64,
  pub creation_time: u64,
  pub modification_time: u64,
  pub file_type: PakFileType,
}

/// The type of the Pak file.
#[repr(C)]
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PakFileType {
  #[default]
  File,
  Directory,
  Symlink,
}
