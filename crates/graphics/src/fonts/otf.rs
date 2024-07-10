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
