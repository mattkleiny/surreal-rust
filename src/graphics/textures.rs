//! GPU texture abstractions.

use glam::Vec2;

#[derive(Debug)]
pub struct TextureId(usize);

#[derive(Debug)]
pub struct Texture {
  id: TextureId,
}

impl Texture {
  pub fn new(id: TextureId) -> Self {
    Self { id }
  }
}

#[derive(Debug)]
pub struct TextureRegion {
  texture: Texture,
  offset: Vec2,
  size: Vec2,
}

impl TextureRegion {
  pub fn new(texture: Texture, offset: Vec2, size: Vec2) -> Self {
    Self { texture, offset, size }
  }
}