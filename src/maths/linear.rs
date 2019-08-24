//! Linear algebra module.

pub use glam::*;

/// An integral point in 2-space.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point2 {
  pub x: u32,
  pub y: u32,
}

impl Point2 {
  pub fn new(x: u32, y: u32) -> Self {
    Self { x, y }
  }
}

/// An integral point in 3-space.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point3 {
  pub x: u32,
  pub y: u32,
  pub z: u32,
}

impl Point3 {
  pub fn new(x: u32, y: u32, z: u32) -> Self {
    Self { x, y, z }
  }
}