//! Linear algebra module.

use std::ops::Neg;

pub use glam::*;

use crate::maths::{Lerp, Random, RandomGenerator};

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

impl RandomGenerator for Vec2i {
  fn random(random: &mut Random) -> Self {
    Self::new(random.next_i32(), random.next_i32())
  }
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

impl Lerp for Vec2i {
  fn lerp(from: Self, to: Self, amount: f32) -> Self {
    Vec2i::new(
      i32::lerp(from.x, to.x, amount),
      i32::lerp(from.y, to.y, amount),
    )
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

impl Lerp for Vec3i {
  fn lerp(from: Self, to: Self, amount: f32) -> Self {
    Vec3i::new(
      i32::lerp(from.x, to.x, amount),
      i32::lerp(from.y, to.y, amount),
      i32::lerp(from.z, to.z, amount),
    )
  }
}

impl RandomGenerator for Vec3i {
  fn random(random: &mut Random) -> Self {
    Self::new(random.next_i32(), random.next_i32(), random.next_i32())
  }
}