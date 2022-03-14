use crate::assets::Asset;
use crate::graphics::Image;
use crate::maths::Vector2;

/// Flags for texture creation.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextureFlags {
  Clamp = 1 << 0,
}

/// Represents a 2d texture.
pub struct Texture {
  width: usize,
  height: usize,
  flags: TextureFlags,
}

/// Represents a sub-region of a `Texture`.
pub struct TextureRegion {
  pub offset: Vector2<f32>,
  pub size: Vector2<usize>,
  pub texture: Asset<Texture>,
}

impl Texture {
  pub fn new(width: usize, height: usize, flags: TextureFlags) -> Self {
    todo!()
  }

  #[inline(always)]
  pub fn width(&self) -> usize {
    self.width
  }

  #[inline(always)]
  pub fn height(&self) -> usize {
    self.height
  }

  #[inline(always)]
  pub fn flags(&self) -> TextureFlags {
    self.flags
  }

  pub fn upload(&mut self, image: &Image) {
    unimplemented!()
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    todo!()
  }
}
