/// Represents type that supports procedural construction of geometry.
pub trait Tessellation {
  /// The type of vertex that we're emitting.
  type Vertex: Clone;

  /// Returns the number of vertices in the tessellator.
  fn vertex_count(&self) -> u32;

  /// Returns the number of indices in the tessellator.
  fn index_count(&self) -> usize;

  /// Adds a single vertex to the tessellator.
  fn add_vertex(&mut self, vertex: Self::Vertex);

  /// Adds a single index to the tessellator.
  fn add_index(&mut self, index: u32);

  /// Adds a line of vertices to the tessellator.
  fn add_line(&mut self, vertices: &[Self::Vertex; 2]) {
    let offset = self.vertex_count();

    self.add_vertex(vertices[0].clone());
    self.add_vertex(vertices[1].clone());

    self.add_index(offset + 0);
    self.add_index(offset + 1);
    self.add_index(offset + 1);
  }

  /// Adds a triangle of vertices to the tessellator.
  fn add_triangle(&mut self, vertices: &[Self::Vertex; 3]) {
    let offset = self.vertex_count();

    self.add_vertex(vertices[0].clone());
    self.add_vertex(vertices[1].clone());
    self.add_vertex(vertices[2].clone());

    self.add_index(offset + 0);
    self.add_index(offset + 1);
    self.add_index(offset + 2);
  }

  /// Adds a triangle fan of vertices to the tessellator.
  fn add_triangle_fan(&mut self, vertices: &[Self::Vertex]) {
    let first = self.vertex_count();

    self.add_vertex(vertices[0].clone());

    for i in 1..vertices.len() - 1 {
      let offset = self.vertex_count();

      self.add_vertex(vertices[i + 0].clone());
      self.add_vertex(vertices[i + 1].clone());

      self.add_index(first);
      self.add_index(offset + 0);
      self.add_index(offset + 1);
    }
  }

  /// Adds a quad of vertices to the tessellator.
  fn add_quad(&mut self, vertices: &[Self::Vertex; 4]) {
    let offset = self.vertex_count();

    self.add_vertex(vertices[0].clone());
    self.add_vertex(vertices[1].clone());
    self.add_vertex(vertices[2].clone());
    self.add_vertex(vertices[3].clone());

    self.add_index(offset + 0);
    self.add_index(offset + 1);
    self.add_index(offset + 2);

    self.add_index(offset + 0);
    self.add_index(offset + 2);
    self.add_index(offset + 3);
  }
}
