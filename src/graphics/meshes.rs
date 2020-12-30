use super::{Buffer, BufferKind, BufferUsage};

/// Represents a single index in a mesh.
pub type Index = u16;

/// Represents type that supports procedural construction of mesh geometry.
pub trait Mesh {
  type Vertex;
  type Index;

  /// Adds a single vertex to the mesh.
  fn add_vertex(&mut self, vertex: &Self::Vertex);

  /// Adds a single index to the mesh.
  fn add_index(&mut self, index: Self::Index);

  /// Adds a triangle of vertices to the mesh.
  fn add_triangle(&mut self, vertices: &[Self::Vertex; 3]) {
    self.add_vertex(&vertices[0]);
    self.add_vertex(&vertices[1]);
    self.add_vertex(&vertices[2]);
  }

  /// Adds a quad of vertices to the mesh.
  fn add_quad(&mut self, vertices: &[Self::Vertex; 4]) {
    self.add_vertex(&vertices[0]);
    self.add_vertex(&vertices[1]);
    self.add_vertex(&vertices[2]);
    self.add_vertex(&vertices[3]);
  }

  /// Adds a triangle fan of vertices to the mesh.
  fn add_triangle_fan(&mut self, vertices: &[Self::Vertex]) {
    unimplemented!()
  }
}

/// Represents a mesh of vertices and indices.
#[derive(Default, Clone)]
pub struct BufferedMesh<V> {
  vertices: Vec<V>,
  indices: Vec<Index>,
}

impl<V> BufferedMesh<V> {
  /// Creates a new empty mesh.
  pub fn new() -> Self {
    Self::from_raw(
      Vec::new(),
      Vec::new(),
    )
  }

  /// Creates a buffered mesh with the given vertex/index count pre-allocated.
  pub fn with_capacity(vertex_count: usize, index_count: usize) -> Self {
    Self::from_raw(
      Vec::with_capacity(vertex_count),
      Vec::with_capacity(index_count),
    )
  }

  /// Creates a mesh from the given buffers.
  pub fn from_raw(vertices: Vec<V>, indices: Vec<Index>) -> Self {
    Self { vertices, indices }
  }

  // direct vertex access
  pub fn vertex_count(&self) -> usize { self.vertices.len() }
  pub fn vertex_slice(&self) -> &[V] { &self.vertices }
  pub fn vertex_mut_slice(&mut self) -> &mut [V] { &mut self.vertices }

  // direct index access
  pub fn index_count(&self) -> usize { self.indices.len() }
  pub fn index_slice(&self) -> &[u16] { &self.indices }
  pub fn index_mut_slice(&mut self) -> &mut [u16] { &mut self.indices }
}

impl<V> Mesh for BufferedMesh<V> where V: Copy {
  type Vertex = V;
  type Index = Index;

  #[inline]
  fn add_vertex(&mut self, vertex: &Self::Vertex) {
    self.vertices.push(vertex.clone());
  }

  #[inline]
  fn add_index(&mut self, index: Self::Index) {
    self.indices.push(index);
  }
}

/// Represents a mesh of vertices that can be rendered to a graphics device.
///
/// This implementation wraps the underlying mesh and provides de-referencing directly through to it.
pub struct GraphicsMesh<V> {
  mesh: BufferedMesh<V>,
  vertex_buffer: Buffer,
  index_buffer: Buffer,
}

impl<V> GraphicsMesh<V> {
  /// Creates a new empty GPU mesh.
  pub fn new(usage: BufferUsage) -> Self {
    Self::from_mesh(BufferedMesh::new(), usage)
  }

  /// Creates a new GPU mesh from the given raw mesh.
  ///
  /// N.B: The resultant mesh has not yet been `upload`ed to the GPU.
  pub fn from_mesh(mesh: BufferedMesh<V>, usage: BufferUsage) -> Self {
    Self {
      mesh,
      vertex_buffer: Buffer::new(BufferKind::Element, usage),
      index_buffer: Buffer::new(BufferKind::Index, usage),
    }
  }

  /// Uploads the mesh contents to the GPU.
  pub fn upload(&mut self) {
    self.vertex_buffer.upload(&self.mesh.vertices);
    self.index_buffer.upload(&self.mesh.indices);
  }
}

impl<V> std::ops::Deref for GraphicsMesh<V> {
  type Target = BufferedMesh<V>;

  fn deref(&self) -> &Self::Target {
    &self.mesh
  }
}

impl<V> std::ops::DerefMut for GraphicsMesh<V> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.mesh
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::vec2;

  use super::*;

  #[test]
  fn it_should_work() {
    let mut mesh = GraphicsMesh::new(BufferUsage::Static);

    mesh.add_triangle(&[vec2(0., 0.), vec2(1., 0.), vec2(1., 1.)]);
    mesh.add_triangle(&[vec2(0., 0.), vec2(1., 0.), vec2(1., 1.)]);
    mesh.add_triangle(&[vec2(0., 0.), vec2(1., 0.), vec2(1., 1.)]);
    mesh.add_triangle(&[vec2(0., 0.), vec2(1., 0.), vec2(1., 1.)]);

    mesh.upload();
  }
}