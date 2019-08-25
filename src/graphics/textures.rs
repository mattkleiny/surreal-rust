//! GPU texture abstractions.

use glam::Vec2;

struct TextureId(usize);

pub struct Texture {
  id: TextureId,
}

pub struct TextureRegion {
  texture: Texture,
  offset: Vec2,
  size: Vec2,
}