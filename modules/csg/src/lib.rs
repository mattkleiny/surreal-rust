//! Constructive Solid Geometry for Surreal
//!
//! This module provides the basic operations for constructing and manipulating
//! 2D and 3D geometry using the Constructive Solid Geometry (CSG) paradigm.
//!
//! The core of this module is the [`Brush`] type, which represents a 2D/3D
//! shape that can be converted into a set of [`Polygon`]s for use in mesh
//! construction.

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

/// A Constructive Solid Geometry (CSG) brush.
///
/// A brush produces a collection of [`Polygon`]s. It can be used to construct
/// a [`Mesh`] and start the CSG pipeline.
pub trait Brush {
  /// Returns the [`Polygon`]s that make up this brush.
  fn polygons(&self, options: &BrushOptions) -> Vec<Polygon>;
}

/// Options for constructing a brush.
pub struct BrushOptions {
  /// The offset to apply to the brush.
  pub offset: Vec3,
  /// The scale to apply to the brush.
  pub scale: Vec3,
}

impl Default for BrushOptions {
  /// Default options for a brush operation.
  fn default() -> Self {
    Self {
      offset: vec3(0.0, 0.0, 0.0),
      scale: vec3(1.0, 1.0, 1.0),
    }
  }
}

/// Possible kinds of merge operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MergeOperation {
  Union,
  Intersection,
  Difference,
}

/// A trait for types that can be merged with common operations.
pub trait Mergeable: Sized {
  /// Merges this value another.
  fn merge(&self, other: &Self, operation: MergeOperation) -> Self;

  /// Computes the union between two values.
  fn union(&self, other: &Self) -> Self {
    self.merge(other, MergeOperation::Union)
  }

  /// Computes the intersection between two values.
  fn intersect(&self, other: &Self) -> Self {
    self.merge(other, MergeOperation::Intersection)
  }

  /// Computes the difference between two values.
  fn diff(&self, other: &Self) -> Self {
    self.merge(other, MergeOperation::Difference)
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

  /// Builds a mesh from the given brush.
  pub fn from_brush(brush: &dyn Brush, options: &BrushOptions) -> Self {
    Self::from_polygons(&brush.polygons(options))
  }

  /// Returns the polygons that make up this mesh.
  pub fn to_polygons(&self) -> Vec<Polygon> {
    self.polygons.clone()
  }

  /// Returns the polygons that make up this mesh.
  pub fn polygons(&self) -> &[Polygon] {
    &self.polygons
  }

  /// Consumes the mesh converting it to a vector of polygons.
  pub fn into_polygons(self) -> Vec<Polygon> {
    self.polygons
  }

  /// Applies some transformation function to all polygon vertices in the mesh.
  pub fn transform(&mut self, transformation: impl Fn(&Vertex) -> Vertex) {
    for polygon in &mut self.polygons {
      // transform the polygon vertices
      for vertex in &mut polygon.vertices {
        *vertex = transformation(&vertex);
      }

      // recompute the polygon plane
      polygon.plane = Plane::from_points(
        polygon.vertices[0].position,
        polygon.vertices[1].position,
        polygon.vertices[2].position,
      );
    }
  }

  /// Applies translation to all polygon vertices in the mesh.
  pub fn translate(&mut self, translation: Vec3) {
    self.transform(|vertex| Vertex {
      position: vertex.position + translation,
      ..*vertex
    });
  }

  /// Applies scaling to all polygon vertices in the mesh.
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

impl From<Vec<Polygon>> for Mesh {
  /// Allows a vector of polygons to be converted into a mesh.
  fn from(polygons: Vec<Polygon>) -> Self {
    Self::from_polygons(&polygons)
  }
}

impl<B: Brush> From<&B> for Mesh {
  /// Allows a brush to be converted into a mesh.
  fn from(brush: &B) -> Self {
    Self::from_brush(brush, &BrushOptions::default())
  }
}

impl Mergeable for Mesh {
  fn merge(&self, _other: &Self, operation: MergeOperation) -> Self {
    match operation {
      MergeOperation::Union => todo!(),
      MergeOperation::Intersection => todo!(),
      MergeOperation::Difference => todo!(),
    }
  }
}

impl Brush for surreal::maths::Plane {
  fn polygons(&self, options: &BrushOptions) -> Vec<Polygon> {
    // build a polygon from the plane in the XZ axis
    let build_vertex = |x: f32, z: f32| {
      let position = vec3(x, 0.0, z) * options.scale + options.offset;
      let normal = self.normal;

      Vertex::new(position, normal)
    };

    let vertices = [
      build_vertex(-0.5, -0.5),
      build_vertex(0.5, -0.5),
      build_vertex(0.5, 0.5),
      build_vertex(-0.5, 0.5),
    ];

    vec![Polygon::new(&vertices)]
  }
}

impl Brush for surreal::maths::Sphere {
  fn polygons(&self, _options: &BrushOptions) -> Vec<Polygon> {
    todo!()
  }
}

impl Brush for surreal::maths::Cylinder {
  fn polygons(&self, _options: &BrushOptions) -> Vec<Polygon> {
    todo!()
  }
}

impl Brush for surreal::maths::Cube {
  fn polygons(&self, _options: &BrushOptions) -> Vec<Polygon> {
    todo!()
  }
}

impl Brush for surreal::maths::Trapezoid {
  fn polygons(&self, _options: &BrushOptions) -> Vec<Polygon> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn plane_should_convert_to_mesh() {
    let plane = Plane::default();
    let mesh = Mesh::from(&plane);

    assert_eq!(mesh.polygons().len(), 1);
  }
}
