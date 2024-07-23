//! A utility for loading and working with PyxelEdit files.

use common::{Chunk, Deserialize, FromStream, InputStream, ZipArchive};

/// Represents a PyxelEdit file.
pub struct PyxelFile {
  document: PyxelEditDocument,
}

/// Represents the metadata of a PyxelEdit file.
struct PyxelEditDocument {
  tile_width: u32,
  tile_height: u32,
  tile_count: u32,
}

impl Deserialize for PyxelEditDocument {
  fn deserialize(chunk: &Chunk) -> Self {
    match chunk {
      Chunk::Map(entries) => Self {
        tile_width: u32::deserialize(entries.get("tileWidth").unwrap()),
        tile_height: u32::deserialize(entries.get("tileHeight").unwrap()),
        tile_count: u32::deserialize(entries.get("tileCount").unwrap()),
      },
      _ => panic!("Invalid Pyxel metadata chunk."),
    }
  }
}

impl FromStream for PyxelFile {
  fn from_stream(stream: &mut dyn InputStream) -> Result<Self, Self::Error> {
    let mut archive = ZipArchive::from_stream(stream)?;
    let metadata = archive.get_file_by_name("docData.json")?;
    let raw_bytes = metadata.extra_data().ok_or(Self::Error::InvalidData)?;
    let document = PyxelEditDocument::from_json_bytes(raw_bytes)?;

    Ok(Self { document })
  }
}
