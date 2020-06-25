//! Sprite batching and rendering.

/// An efficient batch of sprites for rendering by some provider.
pub struct SpriteBatch {}

impl SpriteBatch {
  pub fn new() -> Self {
    Self {}
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
