//! Font loading, management and rendering.
//!
//! We currently support bitmap fonts, with a planned future change to support TrueType fonts.

use crate::assets::{Asset, AssetContext, AssetLoader};
use crate::graphics::{Texture, TextureRegion};
use crate::maths::vec2;

/// A font comprised of bitmap images for each glyph.
pub struct BitmapFont {
  texture: Texture,
  metrics: BitmapFontMetrics,
}

/// Describes the metrics for a bitmap font.
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BitmapFontMetrics {
  pub file_path: String,
  pub glyph_width: u16,
  pub glyph_height: u16,
  pub glyph_padding: u16,
  pub columns: u16,
}

impl BitmapFont {
  /// Creates a new bitmap font.
  pub fn new(texture: &Texture, metrics: BitmapFontMetrics) -> Self {
    Self {
      texture: texture.clone(),
      metrics,
    }
  }

  /// Measures the size of the given text in the font.
  pub fn measure_size(&self, text: &str) -> (usize, usize) {
    let mut line_count = 0;
    let mut longest_line = 0;
    let mut current_line = 0;

    for character in text.chars() {
      current_line += 1;

      if current_line >= longest_line {
        longest_line = current_line;
      }

      if character == '\n' {
        line_count += 1;
        current_line = 0;
      }
    }

    let metrics = &self.metrics;

    let width = longest_line * (metrics.glyph_width + metrics.glyph_padding);
    let height = line_count * (metrics.glyph_height + metrics.glyph_padding);

    return (width as usize, height as usize);
  }

  /// Gets the glyph for the given character.
  pub fn get_glyph(&self, character: char) -> Option<TextureRegion> {
    // we only support ascii glyphs at the moment
    if !character.is_ascii() {
      return None;
    }

    let metrics = &self.metrics;
    let character = character as u8;

    let x = (character as u16 % metrics.columns) * (metrics.glyph_width + metrics.glyph_padding);
    let y = (character as u16 / metrics.columns) * (metrics.glyph_height + metrics.glyph_padding);

    let offset = vec2(x as u32, y as u32);
    let size = vec2(metrics.glyph_width as u32, metrics.glyph_height as u32);

    Some(TextureRegion {
      texture: &self.texture,
      offset,
      size,
    })
  }
}

/// An `AssetLoader` for `BitmapFont`s.
pub struct BitmapFontLoader {}

impl Asset for BitmapFont {
  type Loader = BitmapFontLoader;
}

impl AssetLoader<BitmapFont> for BitmapFontLoader {
  fn load(&self, context: &AssetContext) -> crate::Result<BitmapFont> {
    let metrics: BitmapFontMetrics = context.path.deserialize_json()?;
    let texture = context.load_asset(&metrics.file_path)?;
    let font = BitmapFont::new(&texture, metrics);

    Ok(font)
  }
}
