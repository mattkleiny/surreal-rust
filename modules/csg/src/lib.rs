//! Constructive Solid Geometry for Surreal
//!
//! This module provides the basic operations for constructing and manipulating
//! 2D and 3D geometry using the Constructive Solid Geometry (CSG) paradigm.
//!
//! The core of this module is the [`CsgBrush`] type, which represents a 2D/3D shape
//! that can be constructed from a set of [`Face`]s.
//!
//! [`CsgBrush`]es can be combined with [`CsgOperation`]s to produce new
//! [`CsgBrush`]es which can be further combined, and so on.

use surreal::maths::{Vec2, Vec3};

/// A face representation for use in [`CsgBrush`] construction.
#[derive(Default, Clone)]
pub struct Face {
  _vertices: [Vec3; 3],
  _uvs: [Vec2; 3],
  _normal: Vec3,
}

impl Face {
  /// Creates a [`Face`] from the given vertices, expected in clockwise ordering.
  pub fn from_points_clockwise(_vertices: [Vec3; 3]) -> Self {
    todo!()
  }

  /// Creates a [`Face`] from the given vertices, expected in counter-clockwise ordering.
  pub fn from_points_counter_clockwise(_vertices: [Vec3; 3]) -> Self {
    todo!()
  }
}

/// A Constructive Solid Geometry (CSG) brush.
///
/// A brush produces a collection of [`Face`]s. It represents a shape that produces
/// a collection of polygons that can be used to build a final shape and mesh.
///
/// Brushes can be combined by a [`CsgMerge`] operation to produce another [`CsgBrush`],
/// allowing the set-theoretic combination of geometry.
pub trait CsgBrush {
  /// Returns the [`Face`]s that make up this brush.
  fn faces(&self) -> Vec<Face>;
}

/// Allows merging [`CsgBrush`]es to build new [`CsgBrush`]es.
pub trait CsgMerge {
  /// Merge the given two [`CsgBrush`]es to produce a new [`CsgBrush`].
  fn merge(&self, a: &impl CsgBrush, b: &impl CsgBrush) -> Box<dyn CsgBrush>;
}

/// The default [`CsgMerge`] operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsgOperation {
  Union,
  Intersection,
  Subtraction,
}

impl CsgMerge for CsgOperation {
  fn merge(&self, a: &impl CsgBrush, b: &impl CsgBrush) -> Box<dyn CsgBrush> {
    let _a = a.faces();
    let _b = b.faces();

    match self {
      CsgOperation::Union => todo!(),
      CsgOperation::Intersection => todo!(),
      CsgOperation::Subtraction => todo!(),
    }
  }
}

impl CsgBrush for surreal::maths::Plane {
  fn faces(&self) -> Vec<Face> {
    todo!()
  }
}

impl CsgBrush for surreal::maths::Sphere {
  fn faces(&self) -> Vec<Face> {
    todo!()
  }
}

impl CsgBrush for surreal::maths::Cylinder {
  fn faces(&self) -> Vec<Face> {
    todo!()
  }
}

impl CsgBrush for surreal::maths::Cube {
  fn faces(&self) -> Vec<Face> {
    todo!()
  }
}

impl CsgBrush for surreal::maths::Trapezoid {
  fn faces(&self) -> Vec<Face> {
    todo!()
  }
}
