use crate::assets::Asset;
use crate::graphics::Image;
use crate::maths::Vector2;

/// Represents a 2d texture.
#[derive(Debug, Eq, PartialEq)]
pub struct Texture {
  width: usize,
  height: usize,
  flags: TextureFlags,
}

impl Texture {
  pub fn new(width: usize, height: usize, flags: TextureFlags) -> Self {
    Self {
      width,
      height,
      flags,
    }
  }

  pub fn width(&self) -> usize { self.width }
  pub fn height(&self) -> usize { self.height }
  pub fn flags(&self) -> TextureFlags { self.flags }

  pub fn upload(&mut self, image: &Image) {
    unimplemented!()
  }
}

/// Represents a sub-region of a `Texture`.
pub struct TextureRegion {
  pub offset: Vector2<f32>,
  pub size: Vector2<usize>,
  pub texture: Asset<Texture>,
}

/// Flags for texture creation.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextureFlags {
  Clamp = 1 << 0,
}
