use crate::graphics::{GraphicsServer, GraphicsError};

/// An efficient batch of sprites for rendering by some provider.
pub struct SpriteBatch {}

impl SpriteBatch {
  pub fn new(graphics: &mut impl GraphicsServer) -> Result<Self, GraphicsError> {
    Ok(SpriteBatch {})
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
