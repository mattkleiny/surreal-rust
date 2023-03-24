//! Constructive Solid Geometry for Surreal
//!
//! This module provides the basic operations for constructing and manipulating
//! 2D and 3D geometry using the Constructive Solid Geometry (CSG) paradigm.
//!
//! The core of this module is the [``] type, which represents a 2D/3D shape
//! that can be constructed from a set of [`Polygon`]s.

use surreal::maths::{vec3, Plane, Vec3};

#[derive(Default, Clone, Debug)]
pub struct Vertex {
  position: Vec3,
  normal: Vec3,
}

impl Vertex {
  /// Constructs a new vertex from the given position and normal.
  #[inline]
  pub const fn new(position: Vec3, normal: Vec3) -> Self {
    Self { position, normal }
  }
}

/// A polygon representation for use in [`CsgBrush`] construction.
#[derive(Default, Clone, Debug)]
pub struct Polygon {
  vertices: Vec<Vertex>,
  plane: Plane,
}

impl Polygon {
  /// Creates a new polygon from the given vertices.
  pub fn new(vertices: &[Vertex]) -> Self {
    let plane = Plane::from_points(
      vertices[0].position,
      vertices[1].position,
      vertices[2].position,
    );

    Self {
      vertices: vertices.to_vec(),
      plane,
    }
  }
}

/// A Constructive Solid Geometry (CSG) mesh.
///
/// A mesh is a collection of [`Polygon`]s that can be combined with other
/// meshes to produce a final mesh.
#[derive(Default, Clone, Debug)]
pub struct Mesh {
  polygons: Vec<Polygon>,
}

impl Mesh {
  /// Creates a new empty mesh.
  pub fn new() -> Self {
    Self::default()
  }

  /// Creates a new mesh from the given polygons.
  pub fn from_polygons(polygons: &[Polygon]) -> Self {
    Self {
      polygons: polygons.to_vec(),
    }
  }

  /// Returns the polygons that make up this mesh.
  pub fn to_polygons(&self) -> Vec<Polygon> {
    self.polygons.clone()
  }

  /// Returns the polygons that make up this mesh.
  pub fn polygons(&self) -> &[Polygon] {
    &self.polygons
  }

  /// Applies some transformation function to all polygon vertices in the mesh.
  pub fn transform(&mut self, transformation: impl Fn(&Vertex) -> Vertex) {
    for polygon in &mut self.polygons {
      for vertex in &mut polygon.vertices {
        *vertex = transformation(&vertex);
      }

      polygon.plane = Plane::from_points(
        polygon.vertices[0].position,
        polygon.vertices[1].position,
        polygon.vertices[2].position,
      );
    }
  }

  /// Applies a translation to all polygon vertices in the mesh.
  pub fn translate(&mut self, translation: Vec3) {
    self.transform(|vertex| Vertex {
      position: vertex.position + translation,
      ..*vertex
    });
  }

  /// Applies a translation to all polygon vertices in the mesh.
  pub fn scale(&mut self, scale: Vec3) {
    self.transform(|vertex| Vertex {
      position: vec3(
        vertex.position.x * scale.x,
        vertex.position.y * scale.y,
        vertex.position.z * scale.z,
      ),
      ..*vertex
    });
  }
}

impl From<&dyn Brush> for Mesh {
  /// Allows a brush to be converted into a mesh.
  fn from(brush: &dyn Brush) -> Self {
    Self::from_polygons(&brush.polygons())
  }
}

/// A Constructive Solid Geometry (CSG) brush.
///
/// A brush produces a collection of [`Polygon`]s. It represents the input to
/// some mesh construction operation.
pub trait Brush {
  /// Returns the [`Polygon`]s that make up this brush.
  fn polygons(&self) -> Vec<Polygon>;
}

impl Brush for surreal::maths::Plane {
  fn polygons(&self) -> Vec<Polygon> {
    // build a polygon from the plane
    // TODO: what about scaling the plane?
    let vertices = [
      Vertex::new(vec3(0.0, 0.0, 0.0), self.normal),
      Vertex::new(vec3(1.0, 0.0, 0.0), self.normal),
      Vertex::new(vec3(1.0, 1.0, 0.0), self.normal),
      Vertex::new(vec3(0.0, 1.0, 0.0), self.normal),
    ];

    vec![Polygon::new(&vertices)]
  }
}

impl Brush for surreal::maths::Sphere {
  fn polygons(&self) -> Vec<Polygon> {
    todo!()
  }
}

impl Brush for surreal::maths::Cylinder {
  fn polygons(&self) -> Vec<Polygon> {
    todo!()
  }
}

impl Brush for surreal::maths::Cube {
  fn polygons(&self) -> Vec<Polygon> {
    todo!()
  }
}

impl Brush for surreal::maths::Trapezoid {
  fn polygons(&self) -> Vec<Polygon> {
    todo!()
  }
}
