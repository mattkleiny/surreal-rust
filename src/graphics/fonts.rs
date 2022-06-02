//! Font loading, management and rendering.
//!
//! We currently support bitmap fonts, with a planned future change to support TrueType fonts.

use crate::assets::{Asset, AssetContext, AssetLoader};
use crate::graphics::{Color32, SpriteBatch, SpriteOptions, Texture, TextureRegion};
use crate::maths::Vector2;

/// A font comprised of bitmap images for each glyph.
pub struct BitmapFont {
  texture: Texture,
  metrics: BitmapFontMetrics,
}

/// Describes the metrics for a bitmap font.
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BitmapFontMetrics {
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

  /// Draws the given text on the given sprite batch.
  pub fn draw_text(
    &self,
    batch: &mut SpriteBatch,
    text: &str,
    position: Vector2<f32>,
    color: Color32,
  ) {
    let mut position = position;

    for character in text.chars() {
      if let Some(glyph) = self.get_glyph(character) {
        batch.draw(
          glyph,
          SpriteOptions {
            position,
            color,
            ..Default::default()
          },
        );

        position.x += glyph.size.x as f32;
      }
    }
  }

  /// Gets the glyph for the given character.
  fn get_glyph(&self, _character: char) -> Option<&TextureRegion> {
    todo!()
  }
}

/// An `AssetLoader` for `BitmapFont`s.
pub struct BitmapFontLoader {}

impl Asset for BitmapFont {
  type Loader = BitmapFontLoader;
}

impl AssetLoader<BitmapFont> for BitmapFontLoader {
  fn load(&self, context: &AssetContext) -> crate::Result<BitmapFont> {
    let descriptor: BitmapFontMetrics = context.path.deserialize_json()?;
    let texture = context.load_asset(context.path.change_extension("png"))?;

    let font = BitmapFont::new(&texture, descriptor);

    Ok(font)
  }
}
