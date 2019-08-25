//! GPU mesh abstractions.

use std::mem::size_of;

use crate::maths::*;

use super::*;

/// Represents a vertex.
pub trait Vertex: Copy + Sized {
  const VERTEX_FORMAT: VertexFormat;
}

/// Describes the format of a vertex.
pub type VertexFormat = &'static [VertexElement];

/// Describes an individual element of a vertex.
#[derive(Copy, Clone, Debug)]
pub struct VertexElement {
  pub offset: usize,
  pub stride: usize,
  pub count: usize,
}

/// Encapsulates a mesh of vertices that can be submitted for rendering.
#[derive(Debug)]
pub struct Mesh<V> {
  vertices: Vec<V>,
  indices: Vec<u16>,
  pub format: VertexFormat,
}

impl<V: Vertex> Mesh<V> {
  pub fn new() -> Self {
    Self::new_from_format(V::VERTEX_FORMAT)
  }

  pub fn new_from_format(format: VertexFormat) -> Self {
    Self {
      vertices: Vec::new(),
      indices: Vec::new(),
      format,
    }
  }

  /// Adds a vertex to the mesh.
  #[inline]
  pub fn add_vertex(&mut self, vertex: V) {
    self.vertices.push(vertex);
  }

  /// Adds an index to the mesh.
  #[inline]
  pub fn add_index(&mut self, index: u16) {
    self.indices.push(index);
  }

  /// Adds a triangle of vertices to the mesh.
  pub fn add_triangle(&mut self, vertices: &[V; 3]) {
    self.add_vertex(vertices[0]);
    self.add_vertex(vertices[1]);
    self.add_vertex(vertices[2]);

    let start = self.vertices.len() as u16;

    self.add_index(start + 0);
    self.add_index(start + 1);
    self.add_index(start + 2);

    self.add_index(start + 0);
    self.add_index(start + 2);
    self.add_index(start + 3);
  }

  /// Adds a quad of vertices to the mesh.
  pub fn add_quad(&mut self, _vertices: &[V; 4]) {
    unimplemented!()
  }

  /// Clears the contents of the mesh.
  pub fn clear(&mut self) {
    self.vertices.clear();
    self.indices.clear();
  }
}

/// A standard vertex format for mesh rendering.
#[derive(Copy, Clone, Debug)]
pub struct StandardVertex {
  pub position: Vec3,
  pub normal: Vec3,
  pub color: Color,
}

impl Vertex for StandardVertex {
  const VERTEX_FORMAT: VertexFormat = &[
    VertexElement { offset: 0, stride: size_of::<Vec3>(), count: 1 },
    VertexElement { offset: 1, stride: size_of::<Vec3>(), count: 1 },
    VertexElement { offset: 2, stride: size_of::<Color>(), count: 1 },
  ];
}

/// A standard mesh format with position/normal/color.
pub type StandardMesh = Mesh<StandardVertex>;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_build_a_simple_mesh() {
    #[inline]
    fn new_vertex(x: f32, y: f32, z: f32) -> StandardVertex {
      StandardVertex {
        position: Vec3::new(x, y, z),
        normal: Vec3::zero(),
        color: Color::BLACK,
      }
    }

    let mut mesh = StandardMesh::new();

    mesh.add_triangle(&[
      new_vertex(0., 1., 2.),
      new_vertex(0., 1., 2.),
      new_vertex(0., 1., 2.),
    ]);

    mesh.add_triangle(&[
      new_vertex(0., 1., 2.),
      new_vertex(0., 1., 2.),
      new_vertex(0., 1., 2.),
    ]);

    assert_eq!(6, mesh.vertices.len());
  }
}