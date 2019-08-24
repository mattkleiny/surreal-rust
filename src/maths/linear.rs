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

#[inline]
pub fn vec2(x: f32, y: f32) -> Vec2 { Vec2::new(x, y) }

#[inline]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 { Vec3::new(x, y, z) }

#[inline]
pub fn point2(x: u32, y: u32) -> Point2 { Point2::new(x, y) }

#[inline]
pub fn point3(x: u32, y: u32, z: u32) -> Point3 { Point3::new(x, y, z) }