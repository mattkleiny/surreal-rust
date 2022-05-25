use crate::maths::{vec2, Vector2};

use super::*;

/// The default number of sprites to allocate in a new batch.
const DEFAULT_SPRITE_COUNT: usize = 1024;

/// A fast and lightweight sprite batch renderer.
///
/// This batch pre-allocates an array of vertices and indices and re-uses them for as many
/// sprites as possible.
///
/// Batching is possible over 1 material and texture pair; each texture swap requires a flush
/// and so it's important to pre-sort sprites into batches by material and texture.
pub struct SpriteBatch {
  mesh: Mesh<Vertex2>,
  material: Option<Material>,
  texture: Option<Texture>,
  vertices: Vec<Vertex2>,
}

/// Options for drawing a sprite.
pub struct SpriteOptions {
  pub position: Vector2<f32>,
  pub scale: Vector2<f32>,
  pub color: Color,
}

impl Default for SpriteOptions {
  fn default() -> Self {
    Self {
      position: Vector2::ZERO,
      scale: Vector2::ONE,
      color: Color::WHITE,
    }
  }
}

impl SpriteBatch {
  /// Constructs a new `SpriteBatch`.
  pub fn new(server: &GraphicsServer) -> Self {
    Self::with_capacity(server, DEFAULT_SPRITE_COUNT)
  }

  /// Creates a new `SpriteBatch` with the given sprite capacity.
  pub fn with_capacity(server: &GraphicsServer, sprite_count: usize) -> Self {
    // build standard quad indices ahead-of-time
    let vertices = Vec::with_capacity(sprite_count * 4);
    let indices = build_quad_indices(sprite_count * 6);

    // create mesh, upload quad indices immediately
    let mut mesh = Mesh::new(server);

    mesh.with_buffers(|_, buffer| {
      buffer.write_data(&indices);
    });

    Self {
      mesh,
      material: None,
      texture: None,
      vertices,
    }
  }

  /// Starts a new batch run with the given `Material`.
  pub fn begin(&mut self, material: &Material) {
    self.material = Some(material.clone());
    self.texture = None;

    self.vertices.clear();
  }

  /// Draws a single sprite to the batch.
  pub fn draw(&mut self, region: &TextureRegion, options: SpriteOptions) {
    // flush texture has changed
    if let Some(texture) = &self.texture {
      if *texture != region.texture {
        self.flush();
        self.texture = Some(region.texture.clone());
      }
    } else {
      self.texture = Some(region.texture.clone());
    }

    // flush if we've reached capacity
    if self.vertices.len() + 4 >= self.vertices.capacity() {
      self.flush();
    }

    // TODO: apply sprite transform
    let position = options.position;
    let size = options.scale * vec2(region.size.x as f32, region.size.y as f32);
    let uv = region.calculate_uv();

    self.vertices.push(Vertex2 { position: position + vec2(-size.x, -size.y), color: options.color, uv: uv.bottom_left() });
    self.vertices.push(Vertex2 { position: position + vec2(-size.x, size.y), color: options.color, uv: uv.top_left() });
    self.vertices.push(Vertex2 { position: position + vec2(size.x, size.y), color: options.color, uv: uv.top_right() });
    self.vertices.push(Vertex2 { position: position + vec2(size.x, -size.y), color: options.color, uv: uv.bottom_right() });
  }

  /// Flushes the batch to the GPU.
  pub fn flush(&mut self) {
    if self.vertices.len() == 0 {
      return; // no vertices? no problem
    }

    // fetch the material out
    let Some(material) = &mut self.material else { return; };

    let vertex_count = self.vertices.len();
    let sprite_count = vertex_count / 4;
    let index_count = sprite_count * 6;

    if let Some(texture) = &self.texture {
      material.set_uniform("u_texture", texture);
    }

    self.mesh.with_buffers(|vertices, _| {
      vertices.write_data(&self.vertices);
    });

    self.mesh.draw_sub_mesh(&material, PrimitiveTopology::Triangles, vertex_count, index_count);

    self.vertices.clear();
  }
}

/// Fills the given buffer with standard quad indices.
fn build_quad_indices(index_count: usize) -> Vec<u32> {
  let mut indices = Vec::with_capacity(index_count);
  let mut index = 0;

  for _ in 0..indices.capacity() / 6 {
    indices.push(index + 0);
    indices.push(index + 1);
    indices.push(index + 2);
    indices.push(index + 2);
    indices.push(index + 3);
    indices.push(index + 0);

    index += 4;
  }

  indices
}
