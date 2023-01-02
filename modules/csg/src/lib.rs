//! Constructive Solid Geometry for Surreal
//!
//! This module provides the basic operations for constructing and manipulating
//! 2D and 3D geometry using the Constructive Solid Geometry (CSG) paradigm.
//!
//! The core of this module is the [`Brush`] type, which represents a 2D/3D shape
//! that can be constructed from a set of [`Polygon`]s.
//!
//! [`Brush`]es can be combined with [`Operation`]s to produce new [`Brush`] which
//! can be further combined, and so on.

use surreal::maths::{vec3, Vec3};

/// A polygon representation for use in CSG [`FromPolygon`] construction.
#[derive(Default, Clone)]
pub struct Polygon {
  _vertices: Vec<Vec3>,
  _normal: Vec3,
}

impl Polygon {
  /// Starts building a new [`Polygon`].
  pub fn create() -> PolygonBuilder {
    PolygonBuilder::default()
  }
}

/// A helper for building up [`Polygon`]s from vertices.
#[must_use]
#[derive(Default, Clone)]
pub struct PolygonBuilder {
  vertices: Vec<Vec3>,
  normal: Vec3,
}

impl PolygonBuilder {
  /// Sets the normal of the [`Polygon`] being built.
  pub fn with_normal(mut self, normal: Vec3) -> Self {
    self.normal = normal;
    self
  }

  /// Adds a vertex to the [`Polygon`].
  pub fn add_vertex(mut self, vertex: Vec3) -> Self {
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

  /// Moves the data into the resultant [`Polygon`].
  pub fn into_polygon(self) -> Polygon {
    Polygon {
      _vertices: self.vertices,
      _normal: self.normal,
    }
  }
}

impl From<&PolygonBuilder> for Polygon {
  fn from(value: &PolygonBuilder) -> Self {
    value.build()
  }
}

impl From<PolygonBuilder> for Polygon {
  fn from(value: PolygonBuilder) -> Self {
    value.into_polygon()
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
pub trait Operation {
  /// Applies this operation to the given [`Brush`]es.
  fn apply(&self, a: &impl Brush, b: &impl Brush) -> Vec<Polygon>;
}

pub struct Union;
pub struct Difference;
pub struct Intersection;

impl Operation for Union {
  fn apply(&self, _a: &impl Brush, _b: &impl Brush) -> Vec<Polygon> {
    todo!()
  }
}

impl Operation for Difference {
  fn apply(&self, _a: &impl Brush, _b: &impl Brush) -> Vec<Polygon> {
    todo!()
  }
}

impl Operation for Intersection {
  fn apply(&self, _a: &impl Brush, _b: &impl Brush) -> Vec<Polygon> {
    todo!()
  }
}

/// A sphere.
#[derive(Clone, Debug)]
pub struct Sphere {
  pub radius: f32,
  pub offset: Vec3,
}

impl Default for Sphere {
  fn default() -> Self {
    Self {
      radius: 1.0,
      offset: Vec3::ZERO,
    }
  }
}

impl Brush for Sphere {
  fn to_polygons(&self) -> Vec<Polygon> {
    let polygon = Polygon::create()
      .with_normal(vec3(0.0, 0.0, 1.0))
      .add_vertex(vec3(1.0, 0.0, 0.0))
      .add_vertex(vec3(1.0, 0.0, 0.0))
      .add_vertex(vec3(1.0, 0.0, 0.0))
      .add_vertex(vec3(1.0, 0.0, 0.0))
      .into_polygon();

    vec![polygon]
  }
}

/// A cylinder.
#[derive(Clone, Debug)]
pub struct Cylinder {
  pub radius: f32,
  pub height: f32,
  pub offset: Vec3,
}

/// A cube.
#[derive(Clone, Debug)]
pub struct Cube {
  pub size: Vec3,
  pub offset: Vec3,
}

/// A trapezoidal prism.
#[derive(Clone, Debug)]
pub struct Trapezoid {
  pub size: Vec3,
  pub offset: Vec3,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sphere_should_create_a_simple_brush() {
    let sphere = Sphere {
      radius: 1.0,
      offset: Vec3::ZERO,
    };

    let polygons = sphere.to_polygons();

    assert_eq!(polygons.len(), 1);
  }
}
