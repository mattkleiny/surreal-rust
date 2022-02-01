/// Represents a type that supports procedural construction of mesh geometry.
pub trait Mesh {
  type Vertex;
  type Index;

  /// Adds a single vertex to the mesh.
  fn add_vertex(&mut self, vertex: Self::Vertex);

  /// Adds a single index to the mesh.
  fn add_index(&mut self, index: Self::Index);

  /// Adds a triangle of vertices to the mesh.
  fn add_triangle(&mut self, vertices: [Self::Vertex; 3]) where Self::Vertex: Copy {
    self.add_vertex(vertices[0]);
    self.add_vertex(vertices[1]);
    self.add_vertex(vertices[2]);
  }

  /// Adds a quad of vertices to the mesh.
  fn add_quad(&mut self, vertices: [Self::Vertex; 4]) where Self::Vertex: Copy {
    self.add_vertex(vertices[0]);
    self.add_vertex(vertices[1]);
    self.add_vertex(vertices[2]);
    self.add_vertex(vertices[3]);
  }

  /// Adds a triangle fan of vertices to the mesh.
  fn add_triangle_fan(&mut self, vertices: &[Self::Vertex]) where Self::Vertex: Copy {
    unimplemented!()
  }
}

/// A in-memory mesh of vertices/indices.
#[derive(Clone, Debug)]
pub struct BufferedMesh<V, I> {
  vertices: Vec<V>,
  indices: Vec<I>,
}

impl<V, I> BufferedMesh<V, I> {
  pub fn new() -> Self {
    Self {
      vertices: Vec::new(),
      indices: Vec::new(),
    }
  }
}

impl<V, I> Mesh for BufferedMesh<V, I> {
  type Vertex = V;
  type Index = I;

  fn add_vertex(&mut self, vertex: Self::Vertex) {
    self.vertices.push(vertex);
  }

  fn add_index(&mut self, index: Self::Index) {
    self.indices.push(index);
  }
}