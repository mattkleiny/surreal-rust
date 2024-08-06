use common::{FastHashMap, FromStream, InputStream};

/// A single glyph in an OpenType font.
struct OpenTypeGlyph {}

/// A font using the OpenType font format.
pub struct OpenTypeFont {
  _glyphs: FastHashMap<char, OpenTypeGlyph>,
}

impl FromStream for OpenTypeFont {
  async fn from_stream_async(stream: &mut dyn InputStream) -> Result<Self, Self::Error> {
    let _a = stream.read_u16()?;
    let _b = stream.read_u16()?;

    let result = OpenTypeFont {
      _glyphs: FastHashMap::default(),
    };

    Ok(result)
  }
}
