//! A lightweight sprite system.

use std::mem::size_of;

use glam::Vec2;

use super::*;

/// Represents a sprite that may be rendered using a batch.
pub struct Sprite {
  texture: TextureRegion,
  offset: Vec2,
  pivot: Vec2,
}

impl Sprite {
  pub fn new(texture: TextureRegion, offset: Vec2, pivot: Vec2) -> Self {
    Self { texture, offset, pivot }
  }
}

/// An optimized vertex format for sprite batches.
#[derive(Copy, Clone, Debug)]
struct SpriteVertex {
  position: Vec2,
}

impl Vertex for SpriteVertex {
  const VERTEX_FORMAT: VertexFormat = &[
    VertexElement { offset: 0, stride: size_of::<Vec2>(), count: 1 },
  ];
}

/// An optimised mesh for sprite rendering.
type SpriteMesh = Mesh<SpriteVertex>;

/// A batch of sprites for efficient rendering.
pub struct SpriteBatch {
  mesh: SpriteMesh,

}

impl SpriteBatch {
  pub fn new() -> Self {
    Self {
      mesh: SpriteMesh::new()
    }
  }

  /// Draws the given sprite at the given position.
  pub fn draw_sprite(&mut self, _position: Vec2, _scale: Vec2, _sprite: &Sprite) {
    unimplemented!()
  }

  /// Ends the existing frame in the sprite batch.
  pub fn flush(&mut self) {
    // TODO: flush to the device.
    self.mesh.clear();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_draw_sprites() {
    let mut _batch = SpriteBatch::new();

    // TODO: finish me
  }
}