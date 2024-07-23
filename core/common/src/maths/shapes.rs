//! Commonly used shape definitions in 2 and 3 space.

use crate::{
  maths::{vec3, Vec2, Vec3},
  Lerp,
};

/// A sphere in 2-space.
#[derive(Clone, Debug)]
pub struct Circle {
  pub radius: f32,
  pub center: Vec2,
}

impl Default for Circle {
  fn default() -> Self {
    Self {
      radius: 1.0,
      center: Vec2::ZERO,
    }
  }
}

impl Lerp for Circle {
  #[inline]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self {
      radius: f32::lerp(a.radius, b.radius, t),
      center: Vec2::lerp(a.center, b.center, t),
    }
  }
}

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

impl Lerp for Sphere {
  #[inline]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self {
      radius: f32::lerp(a.radius, b.radius, t),
      center: Vec3::lerp(a.center, b.center, t),
    }
  }
}

/// A cube in 3-space.
#[derive(Clone, Debug)]
pub struct Cube {
  pub size: Vec3,
  pub center: Vec3,
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

impl Default for Cube {
  fn default() -> Self {
    Self {
      size: vec3(1.0, 1.0, 1.0),
      center: Vec3::ZERO,
    }
  }
}

impl Lerp for Cube {
  #[inline]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self {
      size: Vec3::lerp(a.size, b.size, t),
      center: Vec3::lerp(a.center, b.center, t),
    }
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

impl Lerp for Cylinder {
  #[inline]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self {
      radius: f32::lerp(a.radius, b.radius, t),
      height: f32::lerp(a.height, b.height, t),
      center: Vec3::lerp(a.center, b.center, t),
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

impl Lerp for Trapezoid {
  #[inline]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self {
      size: Vec3::lerp(a.size, b.size, t),
      center: Vec3::lerp(a.center, b.center, t),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_circle_lerp() {
    let a = Circle {
      radius: 1.0,
      center: Vec2::new(1.0, 1.0),
    };

    let b = Circle {
      radius: 2.0,
      center: Vec2::new(2.0, 2.0),
    };

    let c = Circle::lerp(a, b, 0.5);

    assert_eq!(c.radius, 1.5);
  }
}
