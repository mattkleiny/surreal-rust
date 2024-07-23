//! Sprite management and rendering.

pub use aseprite::*;
pub use atlas::*;
pub use batch::*;
pub use pyxel::*;

use super::*;

mod aseprite;
mod atlas;
mod batch;
mod pyxel;

/// Represents something that can be drawn as a sprite.
pub trait Sprite {
  /// Returns the texture region for this sprite.
  fn to_region(&self) -> TextureRegion;
}

impl Sprite for Texture {
  #[inline]
  fn to_region(&self) -> TextureRegion {
    self.to_region()
  }
}

impl Sprite for TextureRegion {
  #[inline]
  fn to_region(&self) -> TextureRegion {
    self.clone()
  }
}
