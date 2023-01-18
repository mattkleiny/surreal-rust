//! A binary packed file format for efficient asset streaming

use crate::{collections::FastHashMap, utilities::Size};

/// Compression formats for packed file entries.
#[repr(u8)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Compression {
  None,
}

/// Header data for a packed file.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PakHeader {
  pub entries: Vec<PakEntry>,
}

impl PakHeader {
  /// Computes the [`Size`] of all entries.
  pub fn size(&self) -> Size {
    self.entries.iter().map(|entry| entry.size).sum()
  }
}

/// A packed file entry.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PakEntry {
  pub name: String,
  pub size: Size,
  pub offset: Size,
  pub metadata: FastHashMap<String, String>,
  pub compression: Compression,
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::io::{Deserializable, Serializable};

  #[test]
  fn packed_file_should_save_and_load_to_binary() {
    let header = PakHeader {
      entries: vec![
        PakEntry {
          name: "Test 1".to_string(),
          size: Size::from_bytes(108),
          offset: Size::from_bytes(0),
          metadata: Default::default(),
          compression: Compression::None,
        },
        PakEntry {
          name: "Test 2".to_string(),
          size: Size::from_kilobytes(0.8),
          offset: Size::from_bytes(108),
          metadata: Default::default(),
          compression: Compression::None,
        },
      ],
    };

    let buffer = header.to_binary().unwrap();
    let header = PakHeader::from_binary(&buffer).unwrap();

    println!("{header:#?}");
  }
}
