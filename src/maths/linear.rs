//! Linear algebra module.

pub use glam::*;

/// An integral point in 2-space.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point2d {
  pub x: usize,
  pub y: usize,
}

impl Point2d {
  pub fn new(x: usize, y: usize) -> Self {
    Self { x, y }
  }
}

/// An integral point in 3-space.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point3d {
  pub x: usize,
  pub y: usize,
  pub z: usize,
}

impl Point3d {
  pub fn new(x: usize, y: usize, z: usize) -> Self {
    Self { x, y, z }
  }
}
