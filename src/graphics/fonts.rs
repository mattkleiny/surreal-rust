use crate::graphics::{SpriteBatch, Texture, TextureRegion};
use crate::maths::{Rectangle, Vector2};

pub trait Font {
  fn measure_size(&self, text: &str) -> Rectangle<u32>;
}

pub struct BitmapFont {
  texture: Texture,
  descriptor: BitmapFontDescriptor,
}

#[derive(Deserialize)]
struct BitmapFontDescriptor {
  file_path: String,
  glyph_width: u16,
  glyph_height: u16,
  glyph_padding: u16,
  columns: u16,
}

pub struct BitmapGlyph<'a> {
  region: &'a TextureRegion<'a>,
}

impl BitmapFont {
  fn new(texture: &Texture, descriptor: BitmapFontDescriptor) -> Self {
    Self {
      texture: texture.clone(),
      descriptor,
    }
  }

  pub fn draw_text(&self, batch: &mut SpriteBatch, text: &str, position: Vector2<f32>) {
    todo!()
  }

  fn get_glyph(&self, character: char) -> Option<BitmapGlyph> {
    todo!()
  }
}

impl Font for BitmapFont {
  fn measure_size(&self, text: &str) -> Rectangle<u32> {
    todo!()
  }
}
