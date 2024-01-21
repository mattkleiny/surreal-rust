//! A packed file system.

use std::{fs::File, io::BufReader};

use crate::Deserializable;

/// A packed file.
pub struct PakFile {
  pub headers: Vec<PakFileHeader>,
}

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

impl PakFile {
  /// Loads the Pak file from the given path.
  pub fn load(path: &str) -> crate::Result<Self> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut headers = Vec::new();

    loop {
      let header = match PakFileHeader::from_binary_stream(&mut reader) {
        Ok(header) => header,
        Err(_) => break,
      };

      headers.push(header);
    }

    Ok(Self { headers })
  }
}
