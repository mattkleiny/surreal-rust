//! Linear algebra module.

use std::ops::Neg;

pub use glam::*;

use crate::maths::{Lerp, Random, RNG};

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

impl RNG for Vec2i {
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

impl From<(i32, i32)> for Vec2i {
  fn from((x, y): (i32, i32)) -> Self {
    Self::new(x, y)
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

impl RNG for Vec3i {
  fn random(random: &mut Random) -> Self {
    Self::new(random.next_i32(), random.next_i32(), random.next_i32())
  }
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

impl From<(i32, i32, i32)> for Vec3i {
  fn from((x, y, z): (i32, i32, i32)) -> Self {
    Self::new(x, y, z)
  }
}

/// An integral rectangular shape in 2-space.
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct RectI {
  left: i32,
  top: i32,
  right: i32,
  bottom: i32,
}

impl RectI {
  pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
    Self { left, top, right, bottom }
  }
}