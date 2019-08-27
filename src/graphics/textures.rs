//! GPU texture abstractions.

use glam::Vec2;

#[derive(Debug)]
struct TextureId(usize);

#[derive(Debug)]
pub struct Texture {
  id: TextureId,
}

#[derive(Debug)]
pub struct TextureRegion {
  texture: Texture,
  offset: Vec2,
  size: Vec2,
}