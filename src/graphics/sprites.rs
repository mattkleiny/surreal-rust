use crate::graphics::{BufferedMesh, Color, Mesh};
use crate::maths::{vec2, Vector2};
use crate::prelude::GraphicsServer;

/// Represents a single vertex in the sprite batch.
#[derive(Copy, Clone, Debug)]
struct SpriteVertex {
  pub pos: Vector2<f32>,
  pub uv: Vector2<f32>,
  pub color: Color,
}

/// An efficiently batched array of sprite geometry.
#[derive(Clone)]
pub struct SpriteBatch {
  mesh: BufferedMesh<SpriteVertex, u16>,
}

impl SpriteBatch {
  pub fn new() -> Self {
    Self {
      mesh: BufferedMesh::new()
    }
  }

  /// Adds sprite geometry into the batch.
  pub fn add_sprite(&mut self, position: Vector2<f32>, rotation: f32, size: Vector2<f32>, color: Color) {
    self.mesh.add_quad([
      SpriteVertex { pos: position + vec2(-0.5, -0.5), uv: vec2(0., 0.), color },
      SpriteVertex { pos: position + vec2(-0.5, 0.5), uv: vec2(0., 1.), color },
      SpriteVertex { pos: position + vec2(0.5, 0.5), uv: vec2(1., 1.), color },
      SpriteVertex { pos: position + vec2(0.5, -0.5), uv: vec2(1., 0.), color },
    ]);
  }

  /// Flushes the batch to the given graphics server, rendering the results.
  pub fn draw(&mut self, graphics_server: &dyn GraphicsServer) {
    todo!()
  }
}
