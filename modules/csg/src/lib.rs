//! Constructive Solid Geometry for Surreal
//!
//! This module provides the basic operations for constructing and manipulating
//! 3D geometry using the Constructive Solid Geometry (CSG) paradigm.
//!
//! The core of this module is the [`Brush`] type, which represents a 2D/3D shape
//! that can be constructed from a simple set of [`Polygon`]s.

use core::graphics::{Color32, Index, Vertex, VertexDescriptor, VertexKind};
use core::maths::{Vector2, Vector3};

/// A Constructive Solid Geometry (CSG) mesh.
///
/// A mesh can be built from a series of CSG operations and brushes.
#[derive(Default)]
pub struct Mesh {
  _vertices: Vec<MeshVertex>,
  _indices: Vec<Index>,
}

/// Vertex representation for the [`Mesh`] type used in this module.
#[repr(C)]
#[derive(Clone, Debug)]
struct MeshVertex {
  pub position: Vector3<f32>,
  pub uv: Vector2<f32>,
  pub color: Color32,
}

impl Vertex for MeshVertex {
  #[rustfmt::skip]
  const DESCRIPTORS: &'static [VertexDescriptor] = &[
    VertexDescriptor { count: 3, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 2, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 4, kind: VertexKind::U8, should_normalize: true },
  ];
}

/// A polygon representation for use in CSG [`Mesh`] construction.
#[derive(Default, Clone)]
pub struct Polygon {
  _vertices: Vec<Vector3<f32>>,
  _normal: Vector3<f32>,
}

impl Polygon {
  /// Starts building a new [`Polygon`].
  pub fn create() -> PolygonBuilder {
    PolygonBuilder::default()
  }
}

/// A helper for building up [`Polygon`]s from vertices.
#[derive(Default, Clone)]
pub struct PolygonBuilder {
  vertices: Vec<Vector3<f32>>,
  normal: Vector3<f32>,
}

impl PolygonBuilder {
  pub fn add_vertex(&mut self, vertex: Vector3<f32>) -> &mut Self {
    self.vertices.push(vertex);
    self
  }

  /// Builds the resultant [`Polygon`].
  pub fn build(&self) -> Polygon {
    Polygon {
      _vertices: self.vertices.clone(),
      _normal: self.normal,
    }
  }
}

/// A Constructive Solid Geometry (CSG) brush.
///
/// A brush is an input to a CSG operation. It represents a shape that produces
/// a collection of polygons that can be used to build the final shape and mesh.
pub trait Brush {
  /// Returns the [`Polygon`]s that make up this brush.
  fn to_polygons(&self) -> Vec<Polygon>;
}

/// A Constructive Solid Geometry (CSG) operation.
///
/// An operation is a function that takes a collection of brushes and produces
/// a new brush representing their logical combination.
pub enum Operation {
  Union,
  Difference,
  Intersection,
}

impl Operation {
  /// Applies this operation to the given [`Brush`]es.
  fn apply(&self, _brushes: &[&dyn Brush]) -> Box<dyn Brush> {
    match self {
      Operation::Union => todo!(),
      Operation::Difference => todo!(),
      Operation::Intersection => todo!(),
    }
  }
}

/// A sphere.
#[derive(Clone, Debug)]
pub struct Sphere {
  pub radius: f32,
  pub offset: Vector3<f32>,
}

impl Default for Sphere {
  fn default() -> Self {
    Self {
      radius: 1.0,
      offset: Vector3::ZERO,
    }
  }
}

impl Brush for Sphere {
  fn to_polygons(&self) -> Vec<Polygon> {
    let polygon = Polygon::create()
      .add_vertex(Vector3::new(1.0, 0.0, 0.0))
      .add_vertex(Vector3::new(1.0, 0.0, 0.0))
      .add_vertex(Vector3::new(1.0, 0.0, 0.0))
      .add_vertex(Vector3::new(1.0, 0.0, 0.0))
      .build();

    vec![polygon]
  }
}

/// A cylinder.
#[derive(Clone, Debug)]
pub struct Cylinder {
  pub radius: f32,
  pub height: f32,
  pub offset: Vector3<f32>,
}

/// A cube.
#[derive(Clone, Debug)]
pub struct Cube {
  pub size: Vector3<f32>,
  pub offset: Vector3<f32>,
}

/// A trapezoidal prism.
#[derive(Clone, Debug)]
pub struct Trapezoid {
  pub size: Vector3<f32>,
  pub offset: Vector3<f32>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sphere_should_create_a_simple_brush() {
    let sphere = Sphere {
      radius: 1.0,
      offset: Vector3::ZERO,
    };

    let polygons = sphere.to_polygons();

    assert_eq!(polygons.len(), 1);
  }
}
