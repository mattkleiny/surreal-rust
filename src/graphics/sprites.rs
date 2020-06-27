use crate::graphics::{GraphicsError, GraphicsServer};
use crate::maths::Vector2;

/// An efficient batch of sprites for rendering by some provider.
pub struct SpriteBatch {}

impl SpriteBatch {
  pub fn new(graphics: &mut impl GraphicsServer) -> Result<Self, GraphicsError> {
    Ok(SpriteBatch {})
  }

  /// Pushes sprite geometry into the batch.
  pub fn push(&mut self, position: Vector2<f32>, rotation: f32, image: &impl SpriteImage) {}

  /// Flushes the sprite batch to the given batch target.
  pub fn flush(&mut self, graphics: &mut impl GraphicsServer) {}
}

pub trait SpriteImage {}