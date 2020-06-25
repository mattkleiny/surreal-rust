use crate::graphics::{GraphicsServer, TextureError};
use crate::RID;

/// An efficient batch of sprites for rendering by some provider.
pub struct SpriteBatch {
  texture: RID
}

impl SpriteBatch {
  pub fn new(graphics: &mut impl GraphicsServer) -> Result<Self, TextureError> {
    Ok(SpriteBatch {
      texture: graphics.create_texture()?
    })
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
