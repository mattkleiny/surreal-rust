/// Represents a single index in a mesh.
pub type Index = u16;

/// Represents type that supports procedural construction of mesh geometry.
pub trait Mesh {
  type Vertex: Copy;
  type Index;

  /// Adds a single vertex to the mesh.
  fn add_vertex(&mut self, vertex: Self::Vertex);

  /// Adds a single index to the mesh.
  fn add_index(&mut self, index: Self::Index);

  /// Adds a triangle of vertices to the mesh.
  fn add_triangle(&mut self, vertices: &[Self::Vertex; 3]) {
    self.add_vertex(vertices[0]);
    self.add_vertex(vertices[1]);
    self.add_vertex(vertices[2]);
  }

  /// Adds a quad of vertices to the mesh.
  fn add_quad(&mut self, vertices: &[Self::Vertex; 4]) {
    self.add_vertex(vertices[0]);
    self.add_vertex(vertices[1]);
    self.add_vertex(vertices[2]);
    self.add_vertex(vertices[3]);
  }

  /// Adds a triangle fan of vertices to the mesh.
  fn add_triangle_fan(&mut self, vertices: &[Self::Vertex]) {
    unimplemented!()
  }
}

/// A simple in-memory mesh backed by a `Vec`.
#[derive(Clone, Debug)]
pub struct BufferMesh<V> {
  vertices: Vec<V>,
  indices: Vec<Index>,
}

impl<V> BufferMesh<V> {
  pub fn new() -> Self {
    Self {
      vertices: Vec::new(),
      indices: Vec::new(),
    }
  }
}

impl<V> Mesh for BufferMesh<V> where V: Copy {
  type Vertex = V;
  type Index = Index;

  fn add_vertex(&mut self, vertex: Self::Vertex) {
    self.vertices.push(vertex);
  }

  fn add_index(&mut self, index: Self::Index) {
    self.indices.push(index);
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::vec2;
  use super::*;

  #[test]
  fn it_should_add_a_triangle_to_the_mesh() {
    let mut mesh = BufferMesh::new();

    mesh.add_triangle(&[
      vec2(0., 0.),
      vec2(1., 0.),
      vec2(1., 1.)
    ]);

    println!("{:?}", mesh);
  }
}