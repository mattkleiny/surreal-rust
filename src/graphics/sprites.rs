use crate::assets::Asset;
use crate::graphics::{Color, Texture, TextureRegion};
use crate::maths::Vector2;

/// An efficiently batched array of `Sprite`s.
#[derive(Clone)]
pub struct SpriteBatch {}

impl SpriteBatch {
  /// Creates a new empty sprite batch.
  pub fn new() -> Self {
    Self {}
  }

  /// Pushes sprite geometry into the batch.
  pub fn push(&mut self, position: Vector2<f32>, rotation: f32, size: Vector2<f32>, sprite: &impl Into<TextureRegion>) {
    unimplemented!()
  }
}

/// Vertex definition for our sprite.
#[derive(Clone, Debug)]
struct SpriteVertex {
  pub pos: Vector2<f32>,
  pub uv: Vector2<f32>,
  pub color: Color,
}

/// A sheet of multiple sprites from a single texture.
pub struct SpriteSheet {
  texture: Asset<Texture>,
}

impl SpriteSheet {
  /// Gets a single sprite `TextureRegion` from the sprite sheet.
  pub fn get_sprite(&self, x: u32, y: u32) -> TextureRegion {
    unimplemented!()
  }
}
