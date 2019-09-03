use glam::Vec2;

use super::*;

const VERTEX_SHADER: &'static [u8] = include_bytes!("../../assets/shaders/spritebatch.vert.glsl");
const FRAGMENT_SHADER: &'static [u8] = include_bytes!("../../assets/shaders/spritebatch.frag.glsl");

/// A single vertex used in the optimized mesh representation of our sprite batch.
struct SpriteVertex {
  pos: Vec2,
  uv: Vec2,
}

/// Represents a sprite that can be drawn to a sprite batch.
///
/// A sprite references a texture and denotes an offset/pivot within that texture to use for rendering.
pub struct Sprite<'a, D: GraphicsDevice> {
  pub texture: &'a D::Texture,
  pub offset: Vec2,
  pub size: Vec2i,
  pub pivot: Vec2,
}

/// A simple sprite batch using a single, non-modifiable shader program.
///
/// Each sprite submitted to the batch is expected to be pre-sorted in lowest-to-highest order, such that
/// the last draw call will be the top-most in the scene.
pub struct SpriteBatch<D: GraphicsDevice> {
  shader_program: D::Program,
  mesh: Mesh<D>,
  vertices: Vec<SpriteVertex>,
  indices: Vec<u16>,
  vertex_index: usize,
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
      vertices: Vec::new(),
      indices: Self::prepare_indices(max_sprites * 6), // 6 indices per sprite
      vertex_index: 0,
    }
  }

  /// Prepares the buffer of indices used for sprite rendering.
  fn prepare_indices(index_count: usize) -> Vec<u16> {
    let mut result = Vec::with_capacity(index_count);
    let mut index = 0;

    for i in (0..index_count).step_by(6) {
      result.push(index);
      result.push(index + 1);
      result.push(index + 2);
      result.push(index + 2);
      result.push(index + 3);
      result.push(index);

      index += 4;
    }

    result
  }

  /// Draws the given sprite into the batch.
  pub fn draw_sprite(&mut self, sprite: &Sprite<D>, position: Vec2) {
    let x = position.x();
    let y = position.y();

    let width = sprite.size.x as f32;
    let height = sprite.size.y as f32;

    let extent_x = x + width;
    let extent_y = y + height;

    let u1 = sprite.offset.x() / width;
    let v1 = (sprite.offset.y() + height) / height;
    let u2 = (sprite.offset.x() + width) / width;
    let v2 = sprite.offset.y() / height;

    self.vertices.push(vertex(x, y, u1, v1));
    self.vertices.push(vertex(x, extent_x, u1, v2));
    self.vertices.push(vertex(extent_x, extent_y, u2, v2));
    self.vertices.push(vertex(extent_x, y, u2, v1));

    self.vertex_index += 4;

    /// Builds a new vertex for direct insertion into the sprite batch.
    #[inline(always)]
    fn vertex(x: f32, y: f32, u: f32, v: f32) -> SpriteVertex {
      SpriteVertex { pos: Vec2::new(x, y), uv: Vec2::new(u, v) }
    }
  }

  /// Flushes the batch to the given graphics device.
  pub fn flush(&mut self, command_queue: &CommandQueue<D>) {
    if self.vertex_index > 0 {
      /*// upload the vertices/indices to the GPU
      self.mesh.upload_to_gpu(
        device,
        &self.vertices,
        &self.indices,
      );

      // render the batch as a single mesh
      self.mesh.render(
        device,
        self.indices.len(),
        &self.shader_program,
        textures,
        PrimitiveType::Triangles,
        Mat4::identity(),
      );

      // reset the batch for the next lot of vertices/indices
      self.vertices.clear();
      self.indices.clear();

      self.vertex_index = 0;*/
    }
  }
}
