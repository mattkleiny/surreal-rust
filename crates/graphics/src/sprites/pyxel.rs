//! A utility for loading and working with PyxelEdit files.

use common::{FromStream, InputStream, ZipArchive};

/// Represents a PyxelEdit file.
pub struct PyxelFile {}

impl FromStream for PyxelFile {
  fn from_stream(stream: &mut dyn InputStream) -> Result<Self, Self::Error> {
    let mut archive = ZipArchive::from_stream(stream)?;
    // let metadata = archive.get_file_by_name("docData.json")?;

    Ok(Self {})
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_parse_a_simple_pyxel_edit_file() {
    let file = PyxelFile::from_path("assets/test.pyxel").unwrap();
  }
}
