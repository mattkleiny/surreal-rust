use crate::graphics::{Color, GraphicsDevice, GraphicsError, TextureRegion};
use crate::maths::Vector2;

/// Represents a sprite that may be batched via a `SpriteBatch`.
pub trait Sprite {
  /// Gets the `TextureRegion` for this sprite.
  fn get_sprite_texture(&self) -> &TextureRegion;
}

/// An efficiently batched array of `Sprite`s.
#[derive(Clone, Debug)]
pub struct SpriteBatch {
  vertices: Vec<SpriteVertex>,
  indices: Vec<u16>,
}

impl SpriteBatch {
  pub fn new(graphics: &mut impl GraphicsDevice) -> Result<Self, GraphicsError> {
    Ok(SpriteBatch {
      vertices: Vec::new(),
      indices: Vec::new(),
    })
  }

  /// Pushes sprite geometry into the batch.
  pub fn push(&mut self, position: Vector2<f32>, rotation: f32, sprite: &impl Sprite) {}

  /// Flushes the sprite batch to the given batch target.
  pub fn flush(&mut self, graphics: &mut impl GraphicsDevice) {}
}

/// Vertex definition for our sprite.
#[derive(Clone, Debug)]
struct SpriteVertex {
  pub pos: Vector2<f32>,
  pub uv: Vector2<f32>,
  pub color: Color,
}