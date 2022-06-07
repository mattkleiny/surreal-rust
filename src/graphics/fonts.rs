//! Font loading, management and rendering.
//!
//! We currently support bitmap fonts, with a planned future change to support TrueType fonts.

use std::collections::HashMap;
use std::sync::RwLock;

use ab_glyph::{Font as AbFont, FontVec};

use crate::assets::{Asset, AssetContext, AssetLoader, Handle};
use crate::graphics::{Texture, TextureRegion};
use crate::maths::{vec2, Vector2};

use super::GraphicsServer;

/// Represents different kinds of fonts and permits rendering.
pub trait Font {
  /// Measures the size of the given text in the font.
  fn measure_size(&self, text: &str) -> (usize, usize);

  /// Retrieves a texture region representing the given glyph in the font.
  fn get_glyph(&self, character: char) -> Option<TextureRegion>;
}

/// A font comprised of bitmap images for each glyph.
pub struct BitmapFont {
  texture: Handle<Texture>,
  metrics: BitmapFontMetrics,
}

/// Describes the metrics for a bitmap font.
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BitmapFontMetrics {
  pub file_path: String,
  pub glyph_width: u16,
  pub glyph_height: u16,
  pub glyph_padding: u16,
  pub columns: u16,
}

impl Font for BitmapFont {
  fn measure_size(&self, text: &str) -> (usize, usize) {
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

  fn get_glyph(&self, character: char) -> Option<TextureRegion> {
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
    let texture: Handle<Texture> = context.load_asset(&metrics.file_path)?;

    Ok(BitmapFont {
      texture: texture.clone(),
      metrics,
    })
  }
}

/// A true type font that can be rasterized at different font sizes.
pub struct TrueTypeFont {
  font: FontVec,
  font_size: f32,
  texture: Texture,
  cache: RwLock<HashMap<char, GlyphInfo>>,
}

/// Represents position information for a single glyph in a texture.
struct GlyphInfo {
  pub position: Vector2<u32>,
  pub size: Vector2<u32>,
}

impl Font for TrueTypeFont {
  fn measure_size(&self, _text: &str) -> (usize, usize) {
    todo!()
  }

  fn get_glyph(&self, character: char) -> Option<TextureRegion> {
    // if we already have a glyph position cached for this in our texture
    let cache = self.cache.read().unwrap();

    if let Some(glyph_info) = cache.get(&character) {
      return Some(TextureRegion {
        texture: &self.texture,
        offset: glyph_info.position,
        size: glyph_info.size,
      });
    }

    // otherwise build a new glyph
    let _glyph = self.font.glyph_id(character).with_scale(self.font_size);
    // let mut glyph_cell = self.texture_atlas.create();

    // if let Some(outline) = self.font.outline_glyph(glyph) {
    //   outline.draw(|x, y, coverage| {
    //     let color = Color32::rgba(255, 255, 255, (coverage * 255.0) as u8);

    //     glyph_cell.set_pixel(x, y, color);
    //   });

    //   self.texture_atlas.apply(&mut self.texture);
    // }

    todo!()
  }
}

/// An `AssetLoader` for `TrueTypeFont`s.
pub struct TrueTypeFontLoader {
  pub graphics: GraphicsServer,
  pub font_size: f32,
}

impl Asset for TrueTypeFont {
  type Loader = TrueTypeFontLoader;
}

impl AssetLoader<TrueTypeFont> for TrueTypeFontLoader {
  fn load(&self, context: &AssetContext) -> crate::Result<TrueTypeFont> {
    let bytes = context.path.read_all_bytes()?;

    let font = TrueTypeFont {
      font: FontVec::try_from_vec(bytes)?,
      font_size: self.font_size,
      texture: Texture::new(&self.graphics),
      cache: RwLock::new(HashMap::new()),
    };

    Ok(font)
  }
}
