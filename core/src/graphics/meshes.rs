//! Mesh creation and management.
//!
//! Meshes abstract over vertex and index data on the GPU as well, and
//! provide utilities for constructing data from pieces.

use std::{cell::RefCell, rc::Rc};

use super::*;
use crate::{
  maths::{vec2, Vec2, Vec3},
  utilities::Size,
};

/// Represents the different topologies supported for a mesh.
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PrimitiveTopology {
  Points,
  Lines,
  #[default]
  Triangles,
}

/// Describes a kind of vertex.
///
/// Vertices provide a set of [`VertexDescriptor`]s which are used for binding
/// vertex data to a mesh.
pub trait Vertex: Clone {
  const DESCRIPTORS: &'static [VertexDescriptor];
}

/// Describes a kind of index.
pub type Index = u32;

/// Describes a single vertex field in a [`Vertex`] type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct VertexDescriptor {
  pub count: usize,
  pub kind: VertexKind,
  pub should_normalize: bool,
}

impl VertexDescriptor {
  /// Calculates the size of this vertex element.
  pub fn size(&self) -> Size {
    self.kind.size() * self.count
  }
}

/// Different kinds of vertex primitives.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VertexKind {
  U8,
  U16,
  U32,
  I16,
  I32,
  F32,
  F64,
}

impl VertexKind {
  /// Returns the size of this element type, in bytes.
  pub const fn size(&self) -> Size {
    Size::from_bytes(match self {
      VertexKind::U8 => std::mem::size_of::<u8>(),
      VertexKind::U16 => std::mem::size_of::<u16>(),
      VertexKind::U32 => std::mem::size_of::<u32>(),
      VertexKind::I16 => std::mem::size_of::<i16>(),
      VertexKind::I32 => std::mem::size_of::<i32>(),
      VertexKind::F32 => std::mem::size_of::<f32>(),
      VertexKind::F64 => std::mem::size_of::<f64>(),
    })
  }
}

/// A simple vertex in 2-space with UV and color.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Vertex2 {
  pub position: Vec2,
  pub uv: Vec2,
  pub color: Color32,
}

impl Vertex2 {
  /// Creates a vertex from the given raw parts.
  pub fn new(position: impl Into<Vec2>, uv: impl Into<Vec2>, color: impl Into<Color32>) -> Self {
    Self {
      position: position.into(),
      uv: uv.into(),
      color: color.into(),
    }
  }
}

impl Vertex for Vertex2 {
  #[rustfmt::skip]
  const DESCRIPTORS: &'static [VertexDescriptor] = &[
    VertexDescriptor { count: 2, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 2, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 4, kind: VertexKind::U8, should_normalize: true },
  ];
}

/// A simple vertex in 3-space with UV and color.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Vertex3 {
  pub position: Vec3,
  pub uv: Vec2,
  pub color: Color32,
}

impl Vertex3 {
  /// Creates a vertex from the given raw parts.
  pub fn new(position: impl Into<Vec3>, uv: impl Into<Vec2>, color: impl Into<Color32>) -> Self {
    Self {
      position: position.into(),
      uv: uv.into(),
      color: color.into(),
    }
  }
}

impl Vertex for Vertex3 {
  #[rustfmt::skip]
  const DESCRIPTORS: &'static [VertexDescriptor] = &[
    VertexDescriptor { count: 3, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 2, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 4, kind: VertexKind::U8, should_normalize: true },
  ];
}

/// A mesh of vertices of [`V`] that has been uploaded to the GPU.
///
/// Meshes are stored on the GPU as vertex/index buffers and can be submitted
/// for rendering at any time, provided a valid [`Material`] is available.
#[derive(Clone)]
pub struct Mesh<V> {
  state: Rc<RefCell<MeshState<V>>>,
}

/// The internal state for a mesh.
struct MeshState<V> {
  id: MeshId,
  graphics: GraphicsServer,
  vertices: Buffer<V>,
  indices: Buffer<Index>,
}

impl<V> MeshState<V> {
  /// Borrows the underlying graphics buffers from the state at the same time.
  pub fn borrow_buffers_mut(&mut self) -> (&mut Buffer<V>, &mut Buffer<Index>) {
    (&mut self.vertices, &mut self.indices)
  }
}

impl<V: Vertex> Mesh<V> {
  /// Constructs a new blank mesh on the GPU.
  pub fn new(graphics: &GraphicsServer, usage: BufferUsage) -> crate::Result<Self> {
    let vertices = Buffer::new(graphics, BufferKind::Element, usage)?;
    let indices = Buffer::new(graphics, BufferKind::Index, usage)?;

    Ok(Self {
      state: Rc::new(RefCell::new(MeshState {
        id: graphics.mesh_create(vertices.id(), indices.id(), V::DESCRIPTORS)?,
        graphics: graphics.clone(),
        vertices,
        indices,
      })),
    })
  }

  /// Constructs a mesh with the given [`MeshBuilder`] factory method.
  pub fn from_factory(graphics: &GraphicsServer, factory: impl Fn(&mut MeshBuilder<V>)) -> Self {
    let mut builder = MeshBuilder::new();

    factory(&mut builder);

    Self::from_builder(graphics, &builder)
  }

  /// Constructs a new mesh from the [`MeshBrush`].
  pub fn from_brush(graphics: &GraphicsServer, brush: &impl MeshBrush<V>) -> Self {
    let mut builder = MeshBuilder::new();

    brush.build(&mut builder);

    Self::from_builder(graphics, &builder)
  }

  /// Constructs a mesh with the given [`MeshBuilder`] factory method.
  pub fn from_builder(graphics: &GraphicsServer, builder: &MeshBuilder<V>) -> Self {
    builder.to_mesh(graphics)
  }

  /// Acquires mutable write access the mesh buffers.
  pub fn with_buffers(&mut self, body: impl FnOnce(&mut Buffer<V>, &mut Buffer<Index>)) {
    let state = &mut self.state.borrow_mut();
    let (vertices, indices) = state.borrow_buffers_mut();

    body(vertices, indices);
  }

  /// Draws this mesh with the given material and topology.
  pub fn draw(&self, material: &Material, topology: PrimitiveTopology) {
    let state = self.state.borrow();

    self.draw_sub(material, topology, state.vertices.len(), state.indices.len());
  }

  /// Draws a sub mesh of this mesh with the given material and topology.
  pub fn draw_sub(&self, material: &Material, topology: PrimitiveTopology, vertex_count: usize, index_count: usize) {
    material.bind();

    let state = self.state.borrow();
    let graphics = &state.graphics;

    graphics
      .mesh_draw(state.id, topology, vertex_count, index_count)
      .expect("Failed to draw mesh");

    material.unbind();
  }
}

impl<V> Drop for MeshState<V> {
  fn drop(&mut self) {
    self.graphics.mesh_delete(self.id).expect("Failed to delete mesh");
  }
}

/// A builder pattern for [`Mesh`]es.
#[derive(Default)]
pub struct MeshBuilder<V> {
  vertices: Vec<V>,
  indices: Vec<Index>,
}

impl<V: Vertex> MeshBuilder<V> {
  /// Creates a new empty [`MeshBuilder`].
  pub fn new() -> Self {
    Self {
      vertices: Vec::new(),
      indices: Vec::new(),
    }
  }

  /// Returns the number of vertices in the mesh.
  pub fn vertex_count(&self) -> Index {
    self.vertices.len() as Index
  }

  /// Returns the number of indices in the mesh.
  pub fn index_count(&self) -> usize {
    self.indices.len()
  }

  /// Adds a single vertex to the mesh.
  pub fn add_vertex(&mut self, vertex: V) {
    self.vertices.push(vertex);
  }

  /// Adds a single index to the mesh.
  pub fn add_index(&mut self, index: u32) {
    self.indices.push(index);
  }

  /// Adds a line of vertices to the mesh.
  pub fn add_line(&mut self, vertices: &[V; 2]) {
    let offset = self.vertex_count();

    self.add_vertex(vertices[0].clone());
    self.add_vertex(vertices[1].clone());

    self.add_index(offset);
    self.add_index(offset + 1);
    self.add_index(offset + 1);
  }

  /// Adds a triangle of vertices to the mesh.
  pub fn add_triangle(&mut self, vertices: &[V; 3]) {
    let offset = self.vertex_count();

    self.add_vertex(vertices[0].clone());
    self.add_vertex(vertices[1].clone());
    self.add_vertex(vertices[2].clone());

    self.add_index(offset);
    self.add_index(offset + 1);
    self.add_index(offset + 2);
  }

  /// Adds a triangle fan of vertices to the mesh.
  pub fn add_triangle_fan(&mut self, vertices: &[V]) {
    let first = self.vertex_count();

    self.add_vertex(vertices[0].clone());

    for i in 1..vertices.len() - 1 {
      let offset = self.vertex_count();

      self.add_vertex(vertices[i].clone());
      self.add_vertex(vertices[i + 1].clone());

      self.add_index(first);
      self.add_index(offset);
      self.add_index(offset + 1);
    }
  }

  /// Adds a quad of vertices to the mesh.
  pub fn add_quad(&mut self, vertices: &[V; 4]) {
    let offset = self.vertex_count();

    self.add_vertex(vertices[0].clone());
    self.add_vertex(vertices[1].clone());
    self.add_vertex(vertices[2].clone());
    self.add_vertex(vertices[3].clone());

    self.add_index(offset);
    self.add_index(offset + 1);
    self.add_index(offset + 2);

    self.add_index(offset);
    self.add_index(offset + 2);
    self.add_index(offset + 3);
  }

  /// Uploads the contents of the [`MeshBuilder`] to the given [`Mesh`].
  pub fn upload_to(&self, mesh: &mut Mesh<V>) {
    mesh.with_buffers(|vertices, indices| {
      vertices.write_data(&self.vertices);
      indices.write_data(&self.indices);
    });
  }

  /// Builds a new [`Mesh`] and returns it.
  pub fn to_mesh(&self, graphics: &GraphicsServer) -> Mesh<V> {
    let mut mesh = Mesh::new(graphics, BufferUsage::Static).expect("Failed to create mesh");

    self.upload_to(&mut mesh);

    mesh
  }
}

/// Represents a type that can be tessellated into [`V`]ertices.
pub trait MeshBrush<V> {
  /// Tessellates this object into the given [`MeshBuilder`].
  fn build(&self, builder: &mut MeshBuilder<V>);
}

impl MeshBrush<Vertex2> for crate::maths::Cube {
  fn build(&self, builder: &mut MeshBuilder<Vertex2>) {
    let min = self.min();
    let max = self.max();

    builder.add_quad(&[
      Vertex2::new([min.x, min.y], [0.0, 0.0], Color32::WHITE),
      Vertex2::new([max.x, min.y], [1.0, 0.0], Color32::WHITE),
      Vertex2::new([max.x, max.y], [1.0, 1.0], Color32::WHITE),
      Vertex2::new([min.x, max.y], [0.0, 1.0], Color32::WHITE),
    ]);
  }
}

/// Specialization for standard 2d meshes.
impl Mesh<Vertex2> {
  /// Constructs a simple triangle mesh of the given size.
  pub fn create_triangle(graphics: &GraphicsServer, size: f32) -> Self {
    Self::from_factory(graphics, |builder| {
      builder.add_triangle(&[
        Vertex2 {
          position: vec2(-size, -size),
          color: Color32::WHITE,
          uv: vec2(0., 0.),
        },
        Vertex2 {
          position: vec2(0., size),
          color: Color32::WHITE,
          uv: vec2(0.5, 1.),
        },
        Vertex2 {
          position: vec2(size, -size),
          color: Color32::WHITE,
          uv: vec2(1., 0.),
        },
      ]);
    })
  }

  /// Constructs a simple quad mesh of the given size.
  pub fn create_quad(graphics: &GraphicsServer, size: f32) -> Self {
    Self::from_factory(graphics, |builder| {
      builder.add_quad(&[
        Vertex2 {
          position: vec2(-size, -size),
          color: Color32::WHITE,
          uv: vec2(0., 1.),
        },
        Vertex2 {
          position: vec2(-size, size),
          color: Color32::WHITE,
          uv: vec2(0., 0.),
        },
        Vertex2 {
          position: vec2(size, size),
          color: Color32::WHITE,
          uv: vec2(1., 0.),
        },
        Vertex2 {
          position: vec2(size, -size),
          color: Color32::WHITE,
          uv: vec2(1., 1.),
        },
      ]);
    })
  }

  /// Constructs a simple circle mesh of the given size.
  pub fn create_circle(graphics: &GraphicsServer, radius: f32, segments: usize) -> Self {
    Self::from_factory(graphics, |builder| {
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
          color: Color32::WHITE,
          uv: vec2(u, v),
        })
      }

      builder.add_triangle_fan(&vertices);
    })
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
