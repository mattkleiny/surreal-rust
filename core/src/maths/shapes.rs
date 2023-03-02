//! Commonly used shape definitions in 2 and 3 space.

use super::{vec3, Vec3};

/// A sphere in 3-space.
#[derive(Clone, Debug)]
pub struct Sphere {
  pub radius: f32,
  pub center: Vec3,
}

impl Default for Sphere {
  fn default() -> Self {
    Self {
      radius: 1.0,
      center: Vec3::ZERO,
    }
  }
}

/// A cube in 3-space.
#[derive(Clone, Debug)]
pub struct Cube {
  pub size: Vec3,
  pub center: Vec3,
}

impl Default for Cube {
  fn default() -> Self {
    Self {
      size: vec3(1.0, 1.0, 1.0),
      center: Vec3::ZERO,
    }
  }
}

impl Cube {
  /// The minimum point of the cube.
  #[inline]
  pub fn min(&self) -> Vec3 {
    self.center - self.size / 2.0
  }

  /// The maximum point of the cube.
  #[inline]
  pub fn max(&self) -> Vec3 {
    self.center + self.size / 2.0
  }
}

/// A cylinder in 3-space.
#[derive(Clone, Debug)]
pub struct Cylinder {
  pub radius: f32,
  pub height: f32,
  pub center: Vec3,
}

impl Default for Cylinder {
  fn default() -> Self {
    Self {
      radius: 1.0,
      height: 1.0,
      center: Vec3::ZERO,
    }
  }
}

/// A trapezoidal prism in 3-space.
#[derive(Clone, Debug)]
pub struct Trapezoid {
  pub size: Vec3,
  pub center: Vec3,
}

impl Default for Trapezoid {
  fn default() -> Self {
    Self {
      size: vec3(1.0, 1.0, 1.0),
      center: Vec3::ZERO,
    }
  }
}
