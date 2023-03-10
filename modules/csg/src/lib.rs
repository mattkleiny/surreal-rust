//! Constructive Solid Geometry for Surreal
//!
//! This module provides the basic operations for constructing and manipulating
//! 2D and 3D geometry using the Constructive Solid Geometry (CSG) paradigm.
//!
//! The core of this module is the [`CsgBrush`] type, which represents a 2D/3D
//! shape that can be constructed from a set of [`Face`]s.
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
  /// Creates a [`Face`] from the given vertices in clockwise ordering.
  pub fn from_points_cw(_vertices: &[Vec3]) -> Self {
    todo!()
  }

  /// Creates a [`Face`] from the given vertices in counter-clockwise ordering.
  pub fn from_points_ccw(_vertices: &[Vec3]) -> Self {
    todo!()
  }
}

/// A Constructive Solid Geometry (CSG) brush.
///
/// A brush produces a collection of [`Face`]s. It represents a shape that
/// produces a collection of polygons that can be used to build a final shape
/// and mesh.
///
/// Brushes can be combined by a [`Mergable`] operation to produce another
/// [`Brush`], allowing the set-theoretic combination of geometry.
pub trait Brush {
  /// Returns the [`Face`]s that make up this brush.
  fn faces(&self) -> Vec<Face>;

  /// Merges this brush with another using the given operation.
  fn merge_with(&self, _other: &dyn Brush, _operation: MergeOperation) -> Box<dyn Brush> {
    todo!()
  }
}

/// Possible merge operations for [`Brush`]es.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MergeOperation {
  Union,
  Intersection,
  Difference,
}

impl Brush for surreal::maths::Plane {
  fn faces(&self) -> Vec<Face> {
    vec![Face::from_points_cw(&[
      Vec3::new(-1.0, -1.0, 0.0),
      Vec3::new(1.0, -1.0, 0.0),
      Vec3::new(1.0, 1.0, 0.0),
      Vec3::new(-1.0, 1.0, 0.0),
    ])]
  }
}

impl Brush for surreal::maths::Sphere {
  fn faces(&self) -> Vec<Face> {
    // build up vertical slices of the sphere
    let mut faces = Vec::new();

    // top and bottom caps
    faces.push(Face::from_points_cw(&[
      Vec3::new(0.0, 0.0, 1.0),
      Vec3::new(0.0, 1.0, 0.0),
      Vec3::new(1.0, 0.0, 0.0),
    ]));

    faces.push(Face::from_points_cw(&[
      Vec3::new(0.0, 0.0, -1.0),
      Vec3::new(1.0, 0.0, 0.0),
      Vec3::new(0.0, 1.0, 0.0),
    ]));

    // vertical slices
    for i in 0..16 {
      let angle = i as f32 * std::f32::consts::PI / 8.0;
      let next_angle = (i + 1) as f32 * std::f32::consts::PI / 8.0;

      let x = angle.cos();
      let y = angle.sin();
      let next_x = next_angle.cos();
      let next_y = next_angle.sin();

      faces.push(Face::from_points_cw(&[
        Vec3::new(x, y, 0.0),
        Vec3::new(next_x, next_y, 0.0),
        Vec3::new(next_x, next_y, 1.0),
        Vec3::new(x, y, 1.0),
      ]));
    }

    faces
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

#[cfg(test)]
mod tests {
  use surreal::maths::Plane;

  use super::*;

  #[test]
  fn plane_should_build_brush() {
    let plane = Plane {
      normal: Vec3::new(0.0, 0.0, 1.0),
      distance: 0.0,
    };

    let faces = plane.faces();

    assert_eq!(faces.len(), 1);
  }

  #[test]
  fn sphere_should_build_brush() {
    let sphere = surreal::maths::Sphere {
      radius: 1.0,
      center: Vec3::new(0.0, 0.0, 0.0),
    };

    let faces = sphere.faces();

    assert_eq!(faces.len(), 18);
  }
}
