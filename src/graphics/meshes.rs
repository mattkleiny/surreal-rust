use crate::graphics::{Color, GraphicsContext, GraphicsHandle};
use crate::maths::{Tessellator, vec2, Vector2, Vector3};

/// A mesh of vertices of `V`.
pub struct Mesh<V> {
  handle: GraphicsHandle,
  context: GraphicsContext,
  vertices: Vec<V>,
  indices: Vec<u32>,
}

impl<V> Mesh<V> where V: Vertex {
  /// Constructs a new blank mesh.
  pub fn new(context: &GraphicsContext) -> Self {
    Self {
      handle: unsafe { context.create_mesh() },
      context: context.clone(),
      vertices: Vec::new(),
      indices: Vec::new(),
    }
  }

  /// Constructs a mesh with the given factory method.
  pub fn create(context: &GraphicsContext, factory: impl Fn(&mut Self)) -> Self {
    let mut mesh = Self::new(context);
    factory(&mut mesh);
    mesh
  }
}

impl Mesh<Vertex2> {
  /// Constructs a simple triangle mesh of the given size.
  pub fn create_triangle(context: &GraphicsContext, size: f32) -> Self {
    Self::create(context, |mesh| {
      mesh.add_triangle(&[
        Vertex2 { position: vec2(-size, -size), color: Color::WHITE, uv: vec2(0., 0.) },
        Vertex2 { position: vec2(0., size), color: Color::WHITE, uv: vec2(0.5, 1.) },
        Vertex2 { position: vec2(size, -size), color: Color::WHITE, uv: vec2(1., 0.) },
      ]);
    })
  }

  /// Constructs a simple quad mesh of the given size.
  pub fn create_quad(context: &GraphicsContext, size: f32) -> Self {
    Self::create(context, |mesh| {
      mesh.add_quad(&[
        Vertex2 { position: vec2(-size, -size), color: Color::WHITE, uv: vec2(0., 1.) },
        Vertex2 { position: vec2(-size, size), color: Color::WHITE, uv: vec2(0., 0.) },
        Vertex2 { position: vec2(size, size), color: Color::WHITE, uv: vec2(1., 0.) },
        Vertex2 { position: vec2(size, -size), color: Color::WHITE, uv: vec2(1., 1.) },
      ]);
    })
  }

  /// Constructs a simple circle mesh of the given size.
  pub fn create_circle(context: &GraphicsContext, radius: f32, segments: usize) -> Self {
    Self::create(context, |mesh| {
      use std::f32::consts::PI;

      let mut vertices = Vec::with_capacity(segments);
      let mut theta = 0.;

      for i in 0..segments {
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

/// Default tessellation support for meshes.
impl<V> Tessellator for Mesh<V> where V: Copy {
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

/// Describes a kind of vertex.
pub trait Vertex: Copy {
  fn vertex_descriptors() -> &'static [VertexDescriptor];
}

/// Describes a single vertex field.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct VertexDescriptor {
  offset: usize,
  count: usize,
  kind: VertexKind,
  should_normalize: bool,
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

/// A simple vertex in 2-space.
#[derive(Copy, Clone, Debug)]
pub struct Vertex2 {
  pub position: Vector2<f32>,
  pub color: Color,
  pub uv: Vector2<f32>,
}

impl Vertex for Vertex2 {
  fn vertex_descriptors() -> &'static [VertexDescriptor] {
    // TODO: use a proc macro for this instead?
    static DESCRIPTORS: &[VertexDescriptor] = &[
      VertexDescriptor { offset: 0, count: 2, kind: VertexKind::F32, should_normalize: false },
      VertexDescriptor { offset: 8, count: 4, kind: VertexKind::U8, should_normalize: true },
      VertexDescriptor { offset: 16, count: 2, kind: VertexKind::F32, should_normalize: false },
    ];

    DESCRIPTORS
  }
}

/// A simple vertex in 3-space.
#[derive(Copy, Clone, Debug)]
pub struct Vertex3 {
  pub position: Vector3<f32>,
  pub color: Color,
  pub uv: Vector2<f32>,
}

impl Vertex for Vertex3 {
  fn vertex_descriptors() -> &'static [VertexDescriptor] {
    static DESCRIPTORS: &[VertexDescriptor] = &[
      VertexDescriptor { offset: 0, count: 3, kind: VertexKind::F32, should_normalize: false },
      VertexDescriptor { offset: 12, count: 4, kind: VertexKind::U8, should_normalize: true },
      VertexDescriptor { offset: 20, count: 2, kind: VertexKind::F32, should_normalize: false },
    ];

    DESCRIPTORS
  }
}