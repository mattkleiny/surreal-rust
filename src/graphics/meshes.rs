/// Represents a mesh of vertices that can be rendered to a graphics device.
#[derive(Clone, Debug)]
pub struct Mesh<V> {
  vertices: Vec<V>,
  indices: Vec<u16>,
}

impl<V> Mesh<V> {
  pub fn new_empty() -> Self {
    Self {
      vertices: Vec::new(),
      indices: Vec::new(),
    }
  }

  pub fn new_triangle() -> Self {
    unimplemented!()
  }

  pub fn new_quad() -> Self {
    unimplemented!()
  }

  pub fn add_vertex(&mut self, vertex: V) {
    self.vertices.push(vertex);
  }

  pub fn add_index(&mut self, index: u16) {
    self.indices.push(index);
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{vec2, Vector2};

  use super::*;

  #[test]
  fn it_should_build_a_simple_mesh() {
    let mut mesh = Mesh::new_empty();

    mesh.add_vertex(vec2(0., 0.));
    mesh.add_vertex(vec2(1., 0.));
    mesh.add_vertex(vec2(0.5, 0.5));

    mesh.add_index(0);
    mesh.add_index(1);
    mesh.add_index(2);
  }

  #[test]
  fn it_should_build_a_simple_triangle() {
    let _mesh = Mesh::<Vector2<f32>>::new_triangle();
  }

  #[test]
  fn it_should_build_a_simple_quad() {
    let _mesh = Mesh::<Vector2<f32>>::new_quad();
  }
}