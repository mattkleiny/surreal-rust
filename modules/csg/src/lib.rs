//! Constructive Solid Geometry for Surreal
//!
//! This module provides the basic operations for constructing and manipulating
//! 2D and 3D geometry using the Constructive Solid Geometry (CSG) paradigm.
//!
//! The core of this module is the [`Brush`] type, which represents a 2D/3D shape
//! that can be constructed from a set of [`Face`]s.
//!
//! [`Brush`]es can be combined with [`BrushOperation`]s to produce new [`Brush`] which
//! can be further combined, and so on.

use surreal::maths::{Vec2, Vec3, AABB};

/// A face representation for use in CSG [`Brush`] construction.
#[derive(Default, Clone)]
pub struct Face {
  aabb: AABB,
  uvs: [Vec2; 3],
  vertices: [Vec3; 3],
}

impl Face {
  pub fn from_clockwise_corner_points(vertices: [Vec3; 3]) -> Self {
    Self {
      aabb: AABB::from_points(&vertices),
      uvs: [Vec2::ZERO, Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0)],
      vertices,
    }
  }
}

/// A Constructive Solid Geometry (CSG) brush.
///
/// A brush is an input to a CSG operation. It represents a shape that produces
/// a collection of polygons that can be used to build the final shape and mesh.
pub trait Brush {
  /// Returns the [`Face`]s that make up this brush.
  fn faces(&self) -> Vec<Face>;
}

/// Allows merging [`Brush`]es to build new [`Brush`]es.
pub trait BrushMerge {
  /// Merges two [`Brush`]es to produce a new [`Brush`].
  fn merge(&self, a: &impl Brush, b: &impl Brush) -> Box<dyn Brush>;
}

/// A [`BrushMerge`] operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrushOperation {
  Union,
  Intersection,
  Subtraction,
}

impl BrushMerge for BrushOperation {
  fn merge(&self, a: &impl Brush, b: &impl Brush) -> Box<dyn Brush> {
    let _a = a.faces();
    let _b = b.faces();

    match self {
      BrushOperation::Union => todo!(),
      BrushOperation::Intersection => todo!(),
      BrushOperation::Subtraction => todo!(),
    }
  }
}

impl Brush for surreal::maths::Sphere {
  fn faces(&self) -> Vec<Face> {
    todo!()
  }
}

impl Brush for surreal::maths::Cylinder {
  fn faces(&self) -> Vec<Face> {
    todo!()
  }
}

impl Brush for surreal::maths::Cube {
  fn faces(&self) -> Vec<Face> {
    todo!()
  }
}

impl Brush for surreal::maths::Trapezoid {
  fn faces(&self) -> Vec<Face> {
    todo!()
  }
}
