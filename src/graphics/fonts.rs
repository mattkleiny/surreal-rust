use crate::assets::{Asset, AssetContext, AssetLoader};
use crate::graphics::{SpriteBatch, Texture, TextureRegion};
use crate::io::VirtualPath;
use crate::maths::{Rectangle, Vector2};

/// Represents a font.
pub trait Font {
  /// Measures the size of the given string of text in the font.
  fn measure_size(&self, text: &str) -> Rectangle<u32>;
}

/// A font comprised of bitmap images for each glyph.
pub struct BitmapFont {
  texture: Texture,
  descriptor: BitmapFontDescriptor,
}

/// Describes the metrics for a bitmap font.
#[derive(Deserialize)]
struct BitmapFontDescriptor {
  file_path: String,
  glyph_width: u16,
  glyph_height: u16,
  glyph_padding: u16,
  columns: u16,
}

impl BitmapFont {
  /// Creates a new bitmap font from the given descriptor.
  fn new(texture: &Texture, descriptor: BitmapFontDescriptor) -> Self {
    Self {
      texture: texture.clone(),
      descriptor,
    }
  }

  /// Draws the given text on the given sprite batch.
  pub fn draw_text(&self, batch: &mut SpriteBatch, text: &str, position: Vector2<f32>) {
    // TODO: invert this arrangement?
    todo!()
  }

  /// Gets the glyph for the given character.
  fn get_glyph(&self, character: char) -> Option<TextureRegion> {
    todo!()
  }
}

impl Font for BitmapFont {
  fn measure_size(&self, text: &str) -> Rectangle<u32> {
    todo!()
  }
}

/// An asset loader for bitmap fonts.
pub struct BitmapFontLoader {}

impl Asset for BitmapFont {
  type Loader = BitmapFontLoader;
}

impl AssetLoader<BitmapFont> for BitmapFontLoader {
  fn load(&self, context: &AssetContext) -> crate::Result<BitmapFont> {
    let descriptor: BitmapFontDescriptor = context.path.deserialize_json()?;
    let image_path = VirtualPath::parse(&descriptor.file_path);
    let texture = context.load_asset(image_path)?;
    let font = BitmapFont::new(&texture, descriptor);

    Ok(font)
  }
}