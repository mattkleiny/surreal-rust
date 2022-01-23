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