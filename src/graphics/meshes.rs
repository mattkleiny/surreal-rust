pub use surreal_macros::Vertex;

use crate::graphics::{BufferKind, BufferUsage, Color, GraphicsBuffer, GraphicsServer, GraphicsHandle, Material};
use crate::maths::{Tessellation, vec2, Vector2, Vector3};

/// Represents the different topologies supported for a mesh.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PrimitiveTopology {
  Points,
  Lines,
  Triangles,
}

/// Describes a kind of vertex.
///
/// Vertices provide a set of [`VertexDescriptor`]s which are used for binding vertex data to a mesh.
pub trait Vertex: Copy {
  const DESCRIPTORS: &'static [VertexDescriptor];
}

/// Describes a single vertex field in a `Vertex` type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct VertexDescriptor {
  pub count: usize,
  pub kind: VertexKind,
  pub should_normalize: bool,
}

/// Different kinds of vertex primitives.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VertexKind { U8, U16, U32, I16, I32, F32, F64 }

impl VertexKind {
  /// Returns the size of this element type, in bytes.
  pub const fn size(&self) -> usize {
    match self {
      VertexKind::U8 => std::mem::size_of::<u8>(),
      VertexKind::U16 => std::mem::size_of::<u16>(),
      VertexKind::U32 => std::mem::size_of::<u32>(),
      VertexKind::I16 => std::mem::size_of::<i16>(),
      VertexKind::I32 => std::mem::size_of::<i32>(),
      VertexKind::F32 => std::mem::size_of::<f32>(),
      VertexKind::F64 => std::mem::size_of::<f64>(),
    }
  }
}

/// A simple vertex in 2-space.
#[derive(Copy, Clone, Debug)]
pub struct Vertex2 {
  pub position: Vector2<f32>,
  pub uv: Vector2<f32>,
  pub color: Color,
}

// TODO: swap with derive macro
impl Vertex for Vertex2 {
  const DESCRIPTORS: &'static [VertexDescriptor] = &[
    VertexDescriptor { count: 2, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 2, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 4, kind: VertexKind::F32, should_normalize: false },
  ];
}

/// A simple vertex in 3-space.
#[derive(Vertex, Copy, Clone, Debug)]
pub struct Vertex3 {
  #[vertex(3, F32)] pub position: Vector3<f32>,
  #[vertex(2, F32)] pub uv: Vector2<f32>,
  #[vertex(4, F32)] pub color: Color,
}

/// A mesh of vertices of [`V`] that has been uploaded to the GPU.
///
/// Meshes are stored on the GPU as vertex/index buffers and can be submitted for rendering at any
/// time, provided a valid [`Material`] is available.
pub struct Mesh<V> {
  server: GraphicsServer,
  pub handle: GraphicsHandle,
  pub vertices: GraphicsBuffer<V>,
  pub indices: GraphicsBuffer<u32>,
}

impl<V> Mesh<V> where V: Vertex {
  /// Constructs a new blank mesh on the GPU.
  pub fn new(server: &GraphicsServer) -> Self {
    let vertices = GraphicsBuffer::new(server, BufferKind::Element, BufferUsage::Static);
    let indices = GraphicsBuffer::new(server, BufferKind::Index, BufferUsage::Static);

    Self {
      server: server.clone(),
      handle: server.create_mesh(
        vertices.handle,
        indices.handle,
        V::DESCRIPTORS,
      ),
      vertices,
      indices,
    }
  }

  /// Constructs a mesh with the given factory method.
  pub fn create(server: &GraphicsServer, factory: impl Fn(&mut Tessellator<V>)) -> Self {
    let mut mesh = Self::new(server);
    let mut tessellator = Tessellator::new();

    factory(&mut tessellator);

    tessellator.upload_to(&mut mesh);

    mesh
  }

  /// Draws this mesh with the given material and topology.
  pub fn draw(&self, material: &Material, topology: PrimitiveTopology) {
    self.draw_sub_mesh(material, topology, self.vertices.len(), self.indices.len());
  }

  /// Draws a sub mesh of this mesh with the given material and topology.
  pub fn draw_sub_mesh(&self, material: &Material, topology: PrimitiveTopology, vertex_count: usize, index_count: usize) {
    material.bind();

    self.server.draw_mesh(self.handle, topology, vertex_count, index_count);
  }
}

/// Specialization for standard 2d meshes.
impl Mesh<Vertex2> {
  /// Constructs a simple triangle mesh of the given size.
  pub fn create_triangle(server: &GraphicsServer, size: f32) -> Self {
    Self::create(server, |mesh| {
      mesh.add_triangle(&[
        Vertex2 { position: vec2(-size, -size), color: Color::WHITE, uv: vec2(0., 0.) },
        Vertex2 { position: vec2(0., size), color: Color::WHITE, uv: vec2(0.5, 1.) },
        Vertex2 { position: vec2(size, -size), color: Color::WHITE, uv: vec2(1., 0.) },
      ]);
    })
  }

  /// Constructs a simple quad mesh of the given size.
  pub fn create_quad(server: &GraphicsServer, size: f32) -> Self {
    Self::create(server, |mesh| {
      mesh.add_quad(&[
        Vertex2 { position: vec2(-size, -size), color: Color::WHITE, uv: vec2(0., 1.) },
        Vertex2 { position: vec2(-size, size), color: Color::WHITE, uv: vec2(0., 0.) },
        Vertex2 { position: vec2(size, size), color: Color::WHITE, uv: vec2(1., 0.) },
        Vertex2 { position: vec2(size, -size), color: Color::WHITE, uv: vec2(1., 1.) },
      ]);
    })
  }

  /// Constructs a simple circle mesh of the given size.
  pub fn create_circle(server: &GraphicsServer, radius: f32, segments: usize) -> Self {
    Self::create(server, |mesh| {
      use std::f32::consts::PI;

      let mut vertices = Vec::with_capacity(segments);
      let mut theta = 0.;

      for _ in 0..segments {
        theta += 2. * PI / segments as f32;

        let cos = theta.cos();
        let sin = theta.sin();

        let x = radius * cos;
        let y = radius * sin;

        let u = (cos + 1.) / 2.;
        let v = (sin + 1.) / 2.;

        vertices.push(Vertex2 {
          position: vec2(x, y),
          color: Color::WHITE,
          uv: vec2(u, v),
        })
      }

      mesh.add_triangle_fan(&vertices);
    })
  }
}

impl<V> Drop for Mesh<V> {
  /// Deletes the mesh from the GPU.
  fn drop(&mut self) {
    self.server.delete_mesh(self.handle);
  }
}

/// A simple tessellator for mesh shapes.
pub struct Tessellator<V> {
  vertices: Vec<V>,
  indices: Vec<u32>,
}

impl<V> Tessellator<V> {
  /// Creates a new empty tessellator.
  pub fn new() -> Self {
    Self {
      vertices: Vec::new(),
      indices: Vec::new(),
    }
  }

  /// Uploads the contents of the tessellator to the given [`Mesh`].
  pub fn upload_to(&self, mesh: &mut Mesh<V>) {
    mesh.vertices.write_data(self.vertices.as_slice());
    mesh.indices.write_data(self.indices.as_slice());
  }
}

impl<V> Tessellation for Tessellator<V> where V: Vertex {
  type Vertex = V;

  fn vertex_count(&self) -> u32 { self.vertices.len() as u32 }
  fn index_count(&self) -> usize { self.indices.len() }

  fn add_vertex(&mut self, vertex: Self::Vertex) {
    self.vertices.push(vertex);
  }

  fn add_index(&mut self, index: u32) {
    self.indices.push(index);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vertex_2_should_derive_valid_descriptors() {
    let descriptors = Vertex2::DESCRIPTORS;

    assert_eq!(descriptors.len(), 3);
  }

  #[test]
  fn vertex_3_should_derive_valid_descriptors() {
    let descriptors = Vertex3::DESCRIPTORS;

    assert_eq!(descriptors.len(), 3);
  }
}
