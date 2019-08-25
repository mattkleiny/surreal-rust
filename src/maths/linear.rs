//! Linear algebra module.

pub use glam::*;
use std::ops::Neg;

/// An integral point in 2-space.
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vec2i {
  pub x: i32,
  pub y: i32,
}

impl Vec2i {
  pub const ZERO: Vec2i = Self::new(0, 0);

  #[inline]
  pub const fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  pub const fn unit_x() -> Self { Self::new(1, 0) }
  pub const fn unit_y() -> Self { Self::new(0, 1) }
}

impl Neg for Vec2i {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self {
      x: -self.x,
      y: -self.y,
    }
  }
}

/// An integral point in 3-space.
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vec3i {
  pub x: i32,
  pub y: i32,
  pub z: i32,
}

impl Vec3i {
  pub const ZERO: Vec3i = Self::new(0, 0, 0);

  #[inline]
  pub const fn new(x: i32, y: i32, z: i32) -> Self {
    Self { x, y, z }
  }

  pub const fn unit_x() -> Self { Self::new(1, 0, 0) }
  pub const fn unit_y() -> Self { Self::new(0, 1, 0) }
  pub const fn unit_z() -> Self { Self::new(0, 0, 1) }
}

impl Neg for Vec3i {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}
