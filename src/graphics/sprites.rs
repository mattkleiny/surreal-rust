//! Sprite rendering and management.

use glam::Vec2;

use super::*;

const VERTEX_SHADER: &'static [u8] = include_bytes!("../../assets/shaders/spritebatch.vert.glsl");
const FRAGMENT_SHADER: &'static [u8] = include_bytes!("../../assets/shaders/spritebatch.frag.glsl");

/// A single vertex used in the optimized mesh representation of our sprite batch.
struct SpriteVertex {
  position: Vec2,
  uv: Vec2,
}

/// A simple sprite batch using a single, non-modifiable shader program.
///
/// Each sprite submitted to the batch is expected to be pre-sorted in lowest-to-highest order, such that
/// the last draw call will be the top-most in the scene.
pub struct SpriteBatch<D: GraphicsDevice> {
  shader_program: D::Program,
  mesh: Mesh<D, SpriteVertex>,
}

impl<D: GraphicsDevice> SpriteBatch<D> {
  /// Creates a new sprite batch that can fit up to the given number of sprites.
  pub fn new(device: &D, max_sprites: usize) -> Self {
    unsafe {
      // prepare our shaders and shader program
      let vertex_shader = device.create_shader_from_source(VERTEX_SHADER, ShaderKind::Vertex);
      let fragment_shader = device.create_shader_from_source(FRAGMENT_SHADER, ShaderKind::Fragment);
      let shader_program = device.create_program_from_shaders(vertex_shader, fragment_shader);

      Self::with_shader_program(device, shader_program, max_sprites)
    }
  }

  /// Creates a sprite batch that uses the given shader program for rendering.
  pub fn with_shader_program(device: &D, shader_program: D::Program, max_sprites: usize) -> Self {
    Self {
      shader_program,
      mesh: Mesh::new(device),
    }
  }

  /// Flushes the batch to the given graphics device.
  pub fn flush(&mut self, device: &D) {
    self.mesh.draw(device, &self.shader_program, PrimitiveType::Triangles);
  }
}
