use crate::graphics::{GraphicsContext, Material, Mesh, PrimitiveTopology, Texture, Vertex2};

const DEFAULT_SPRITE_COUNT: usize = 1024;

/// A fast and lightweight sprite batch renderer.
pub struct SpriteBatch<'a> {
  vertices: Vec<Vertex2>,
  mesh: Mesh<Vertex2>,
  material: Option<&'a mut Material<'a>>,
  last_texture: Option<&'a Texture>,
  vertex_count: usize,
}

impl<'a> SpriteBatch<'a> {
  /// Constructs a new sprite batch.
  pub fn new(context: &GraphicsContext) -> Self {
    Self::with_capacity(context, DEFAULT_SPRITE_COUNT)
  }

  /// Creates a new sprite batch with the given sprite capacity.
  pub fn with_capacity(context: &GraphicsContext, sprite_count: usize) -> Self {
    // upload standard quad indices ahead-of-time
    let mut mesh = Mesh::new(context);

    mesh.indices.write_data(&build_quad_indices(sprite_count * 6));

    Self {
      vertices: Vec::with_capacity(sprite_count * 4),
      mesh,
      material: None,
      last_texture: None,
      vertex_count: 0,
    }
  }

  // TODO: return a batch scope instead?
  /// Starts a new batch with the given [`Material`].
  pub fn begin(&mut self, material: &'a mut Material<'a>) {
    self.material = Some(material);
    self.vertex_count = 0;
  }

  /// Draws a sprite to the batch.
  pub fn draw(&mut self) {
    todo!()
  }

  /// Flushes the batch to the GPU.
  pub fn flush(&mut self) {
    if self.vertex_count == 0 { return; }

    let Some(material) = &mut self.material else { return; };

    let sprite_count = self.vertex_count / 4;
    let index_count = sprite_count * 6;

    if let Some(texture) = self.last_texture {
      material.set_texture("u_texture", texture.handle, 0, None);
    }

    self.mesh.vertices.write_data(&self.vertices);
    self.mesh.draw_sub_mesh(material, PrimitiveTopology::Triangles, self.vertex_count, index_count);

    self.vertex_count = 0;
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
