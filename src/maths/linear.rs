//! Linear algebra module.

pub use glam::*;

/// An integral point in 2-space.
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vec2i {
  pub x: u32,
  pub y: u32,
}

impl Vec2i {
  pub const ZERO: Vec2i = Self::new(0, 0);

  #[inline]
  pub const fn new(x: u32, y: u32) -> Self {
    Self { x, y }
  }
}

/// An integral point in 3-space.
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vec3i {
  pub x: u32,
  pub y: u32,
  pub z: u32,
}

impl Vec3i {
  pub const ZERO: Vec3i = Self::new(0, 0, 0);

  #[inline]
  pub const fn new(x: u32, y: u32, z: u32) -> Self {
    Self { x, y, z }
  }
}