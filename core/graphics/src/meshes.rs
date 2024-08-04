//! Mesh creation and management.
//!
//! Meshes abstract over vertex and index data on the GPU as well, and
//! provide utilities for constructing data from pieces.

use common::{vec2, Color32, Size, Vec2, Vec3};

use super::*;

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

/// An index into a mesh buffer.
pub type MeshIndex = u32;

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
      VertexKind::U8 => size_of::<u8>(),
      VertexKind::U16 => size_of::<u16>(),
      VertexKind::U32 => size_of::<u32>(),
      VertexKind::I16 => size_of::<i16>(),
      VertexKind::I32 => size_of::<i32>(),
      VertexKind::F32 => size_of::<f32>(),
      VertexKind::F64 => size_of::<f64>(),
    })
  }
}

/// A simple vertex in 2-space with UV and color.
#[repr(C)]
#[derive(Clone, Debug, Vertex)]
pub struct Vertex2 {
  #[vertex(2, F32)]
  pub position: Vec2,
  #[vertex(2, F32)]
  pub uv: Vec2,
  #[vertex(4, U8, normalize)]
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

/// A simple vertex in 3-space with UV and color.
#[repr(C)]
#[derive(Clone, Debug, Vertex)]
pub struct Vertex3 {
  #[vertex(3, F32)]
  pub position: Vec3,
  #[vertex(2, F32)]
  pub uv: Vec2,
  #[vertex(4, U8, normalize)]
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

/// A mesh of vertices of [`V`] that has been uploaded to the GPU.
///
/// Meshes are stored on the GPU as vertex/index buffers and can be submitted
/// for rendering at any time, provided a valid [`Material`] is available.
#[derive(Clone)]
pub struct Mesh<V> {
  state: internal::GraphicsCell<MeshState<V>>,
}

/// The internal state for a mesh.
struct MeshState<V> {
  id: MeshId,
  vertices: Buffer<V>,
  indices: Buffer<MeshIndex>,
}

impl<V> MeshState<V> {
  /// Borrows the underlying graphics buffers from the state at the same time.
  pub fn borrow_buffers_mut(&mut self) -> (&mut Buffer<V>, &mut Buffer<MeshIndex>) {
    (&mut self.vertices, &mut self.indices)
  }
}

impl<V: Vertex> Mesh<V> {
  /// Constructs a new blank mesh on the GPU.
  pub fn new(usage: BufferUsage) -> Result<Self, MeshError> {
    let vertices = Buffer::new(BufferKind::Element, usage).map_err(|_| MeshError::FailedToCreate)?;
    let indices = Buffer::new(BufferKind::Index, usage).map_err(|_| MeshError::FailedToCreate)?;

    Ok(Self {
      state: internal::GraphicsCell::new(MeshState {
        id: graphics().mesh_create(vertices.id(), indices.id(), V::DESCRIPTORS)?,
        vertices,
        indices,
      }),
    })
  }

  /// Constructs a mesh with the given [`MeshBuilder`] factory method.
  pub fn from_factory(factory: impl Fn(&mut MeshBuilder<V>)) -> Self {
    let mut builder = MeshBuilder::new();

    factory(&mut builder);

    Self::from_builder(&builder)
  }

  /// Constructs a new mesh from the [`MeshBrush`].
  pub fn from_brush(brush: &dyn MeshBrush<V>) -> Self {
    let mut builder = MeshBuilder::new();

    brush.build(&mut builder);

    Self::from_builder(&builder)
  }

  /// Constructs a mesh with the given [`MeshBuilder`] factory method.
  pub fn from_builder(builder: &MeshBuilder<V>) -> Self {
    builder.to_mesh()
  }

  /// Returns the identifier of this mesh.
  pub fn id(&self) -> MeshId {
    self.state.read().id
  }

  /// Returns the number of vertices in the mesh.
  pub fn vertices(&self) -> usize {
    self.state.read().vertices.len()
  }

  /// Returns the number of indices in the mesh.
  pub fn indices(&self) -> usize {
    self.state.read().indices.len()
  }

  /// Draws this mesh with the given material and topology.
  pub fn draw(&self, material: &Material, topology: PrimitiveTopology) {
    let state = self.state.read();

    self.draw_sub(material, topology, state.vertices.len(), state.indices.len());
  }

  /// Draws a sub mesh of this mesh with the given material and topology.
  pub fn draw_sub(&self, material: &Material, topology: PrimitiveTopology, vertex_count: usize, index_count: usize) {
    let state = self.state.read();

    material.bind();

    graphics()
      .mesh_draw(state.id, topology, vertex_count, index_count)
      .expect("Failed to draw mesh");

    material.unbind();
  }

  /// Acquires mutable write access the mesh buffers.
  pub fn with_buffers(&mut self, body: impl FnOnce(&mut Buffer<V>, &mut Buffer<MeshIndex>)) {
    let state = &mut self.state.write();
    let (vertices, indices) = state.borrow_buffers_mut();

    body(vertices, indices);
  }
}

impl<V> Drop for MeshState<V> {
  fn drop(&mut self) {
    graphics().mesh_delete(self.id).expect("Failed to delete mesh");
  }
}

/// A builder pattern for [`Mesh`]es.
#[derive(Default)]
pub struct MeshBuilder<V> {
  vertices: Vec<V>,
  indices: Vec<MeshIndex>,
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
  pub fn vertex_count(&self) -> MeshIndex {
    self.vertices.len() as MeshIndex
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
  pub fn to_mesh(&self) -> Mesh<V> {
    let mut mesh = Mesh::new(BufferUsage::Static).expect("Failed to create mesh");

    self.upload_to(&mut mesh);

    mesh
  }
}

/// Specialization for standard 2d meshes.
impl Mesh<Vertex2> {
  /// Constructs a simple triangle mesh of the given size.
  pub fn create_triangle(size: f32) -> Self {
    Self::from_factory(|builder| {
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
  pub fn create_quad(size: f32) -> Self {
    Self::from_factory(|builder| {
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
  pub fn create_circle(radius: f32, segments: usize) -> Self {
    Self::from_factory(|builder| {
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

/// Represents a type that can be tessellated into [`V`]ertices.
pub trait MeshBrush<V> {
  /// Tessellates this object into the given [`MeshBuilder`].
  fn build(&self, builder: &mut MeshBuilder<V>);
}

impl MeshBrush<Vertex2> for common::Cube {
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

impl MeshBrush<Vertex2> for common::Circle {
  fn build(&self, builder: &mut MeshBuilder<Vertex2>) {
    use std::f32::consts::PI;

    let mut vertices = Vec::with_capacity(16);
    let mut theta = 0.;

    for _ in 0..16 {
      theta += 2. * PI / 16.;

      let cos = theta.cos();
      let sin = theta.sin();

      let x = self.radius * cos;
      let y = self.radius * sin;

      let u = (cos + 1.) / 2.;
      let v = (sin + 1.) / 2.;

      vertices.push(Vertex2::new([x, y], [u, v], Color32::WHITE))
    }

    builder.add_triangle_fan(&vertices);
  }
}

impl MeshBrush<Vertex3> for common::Cube {
  fn build(&self, builder: &mut MeshBuilder<Vertex3>) {
    let min = self.min();
    let max = self.max();

    builder.add_quad(&[
      Vertex3::new([min.x, min.y, min.z], [0.0, 0.0], Color32::WHITE),
      Vertex3::new([max.x, min.y, min.z], [1.0, 0.0], Color32::WHITE),
      Vertex3::new([max.x, max.y, min.z], [1.0, 1.0], Color32::WHITE),
      Vertex3::new([min.x, max.y, min.z], [0.0, 1.0], Color32::WHITE),
    ]);

    builder.add_quad(&[
      Vertex3::new([min.x, min.y, max.z], [0.0, 0.0], Color32::WHITE),
      Vertex3::new([max.x, min.y, max.z], [1.0, 0.0], Color32::WHITE),
      Vertex3::new([max.x, max.y, max.z], [1.0, 1.0], Color32::WHITE),
      Vertex3::new([min.x, max.y, max.z], [0.0, 1.0], Color32::WHITE),
    ]);

    builder.add_quad(&[
      Vertex3::new([min.x, min.y, min.z], [0.0, 0.0], Color32::WHITE),
      Vertex3::new([max.x, min.y, min.z], [1.0, 0.0], Color32::WHITE),
      Vertex3::new([max.x, min.y, max.z], [1.0, 1.0], Color32::WHITE),
      Vertex3::new([min.x, min.y, max.z], [0.0, 1.0], Color32::WHITE),
    ]);

    builder.add_quad(&[
      Vertex3::new([min.x, max.y, min.z], [0.0, 0.0], Color32::WHITE),
      Vertex3::new([max.x, max.y, min.z], [1.0, 0.0], Color32::WHITE),
      Vertex3::new([max.x, max.y, max.z], [1.0, 1.0], Color32::WHITE),
      Vertex3::new([min.x, max.y, max.z], [0.0, 1.0], Color32::WHITE),
    ]);

    builder.add_quad(&[
      Vertex3::new([min.x, min.y, min.z], [0.0, 0.0], Color32::WHITE),
      Vertex3::new([min.x, max.y, min.z], [1.0, 0.0], Color32::WHITE),
      Vertex3::new([min.x, max.y, max.z], [1.0, 1.0], Color32::WHITE),
      Vertex3::new([min.x, min.y, max.z], [0.0, 1.0], Color32::WHITE),
    ]);

    builder.add_quad(&[
      Vertex3::new([max.x, min.y, min.z], [0.0, 0.0], Color32::WHITE),
      Vertex3::new([max.x, max.y, min.z], [1.0, 0.0], Color32::WHITE),
      Vertex3::new([max.x, max.y, max.z], [1.0, 1.0], Color32::WHITE),
      Vertex3::new([max.x, min.y, max.z], [0.0, 1.0], Color32::WHITE),
    ]);
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
