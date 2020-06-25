//! Sprite batching and rendering.

use crate::graphics::{GraphicsDevice, Renderable};

/// An efficient batch of sprites for rendering by some provider.
pub struct SpriteBatch {}

impl SpriteBatch {
  pub fn new() -> Self {
    SpriteBatch {}
  }

  /// Pushes sprite geometry into the batch.
  pub fn push(&mut self) {
    unimplemented!()
  }

  /// Flushes the sprite batch to the given batch target.
  pub fn flush(&mut self) {
    unimplemented!()
  }
}

impl Renderable for SpriteBatch {
  fn render(&self, device: &mut impl GraphicsDevice) {
    // TODO: implement me
  }
}