use common::{FastHashMap, FileSystemError, FromStream, InputStream};

/// A single glyph in an OpenType font.
struct OpenTypeGlyph {}

/// A font using the OpenType font format.
pub struct OpenTypeFont {
  _glyphs: FastHashMap<char, OpenTypeGlyph>,
}

impl FromStream for OpenTypeFont {
  fn from_stream(stream: &mut dyn InputStream) -> Result<Self, FileSystemError> {
    let _a = stream.read_u16()?;
    let _b = stream.read_u16()?;

    let result = OpenTypeFont {
      _glyphs: FastHashMap::default(),
    };

    Ok(result)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_load_from_otf_file() {
    let _font = OpenTypeFont::from_path("./assets/fonts/bit536_v1.otf").unwrap();
  }
}
