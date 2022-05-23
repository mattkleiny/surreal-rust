use crate::graphics::{GraphicsServer, GraphicsImpl, Material, Mesh, PrimitiveTopology, Texture, Vertex2};

/// The default number of sprites to allocate in a new batch.
const DEFAULT_SPRITE_COUNT: usize = 1024;

/// A fast and lightweight sprite batch renderer.
///
/// This batch pre-allocates an array of vertices and indices and re-uses them for as many
/// sprites as possible.
///
/// Batching is possible over 1 material and texture pair; each texture swap requires a flush
/// and so it's important to pre-sort sprites into batches by material and texture.
pub struct SpriteBatch<G> where G: GraphicsImpl {
  mesh: Mesh<G, Vertex2>,
  vertices: Vec<Vertex2>,
  vertex_count: usize,
}

/// A scope for [`SpriteBatch`] operation.
///
/// Borrows the batch data and holds onto the material and active texture.
pub struct SpriteBatchScope<'a, G> where G: GraphicsImpl {
  batch: &'a mut SpriteBatch<G>,
  material: &'a mut Material<'a, G>,
  texture: Option<&'a Texture<G>>,
}

impl<G> SpriteBatch<G> where G: GraphicsImpl {
  /// Constructs a new [`SpriteBatch`] .
  pub fn new(server: &GraphicsServer<G>) -> Self {
    Self::with_capacity(server, DEFAULT_SPRITE_COUNT)
  }

  /// Creates a new [`SpriteBatch`] with the given sprite capacity.
  pub fn with_capacity(server: &GraphicsServer<G>, sprite_count: usize) -> Self {
    // build standard quad indices ahead-of-time
    let mut mesh = Mesh::new(server);
    let indices = build_quad_indices(sprite_count * 6);

    mesh.indices.write_data(&indices);

    Self { mesh, vertices: Vec::with_capacity(sprite_count * 4), vertex_count: 0 }
  }

  /// Starts a new [`SpriteBatchScope`] with the given [`Material`].
  pub fn begin<'a>(&'a mut self, material: &'a mut Material<'a, G>) -> SpriteBatchScope<'a, G> {
    SpriteBatchScope { batch: self, material, texture: None }
  }
}


impl<'a, G> SpriteBatchScope<'a, G> where G: GraphicsImpl {
  /// Draws a sprite to the batch.
  pub fn draw(&mut self) {
    todo!()
  }

  /// Flushes the batch to the GPU.
  pub fn flush(&mut self) {
    let batch = &mut self.batch;

    if batch.vertex_count == 0 {
      return; // no vertices? no problem
    }

    let sprite_count = batch.vertex_count / 4;
    let index_count = sprite_count * 6;

    if let Some(texture) = self.texture {
      self.material.set_texture("u_texture", texture.handle, 0, None);
    }

    batch.mesh.vertices.write_data(&batch.vertices);
    batch.mesh.draw_sub_mesh(self.material, PrimitiveTopology::Triangles, batch.vertex_count, index_count);

    batch.vertex_count = 0;
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
