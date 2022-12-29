use super::*;

/// Shorthand to construct a [`Ray2`]
#[inline(always)]
pub const fn ray2(origin: Vec2, direction: Vec2) -> Ray2 {
  Ray2::new(origin, direction)
}

/// Shorthand to construct a [`Ray3`]
#[inline(always)]
pub const fn ray3(origin: Vec3, direction: Vec3) -> Ray3 {
  Ray3::new(origin, direction)
}

/// Represents a ray into 2-space.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray2 {
  pub origin: Vec2,
  pub direction: Vec2,
}

impl Ray2 {
  #[inline(always)]
  pub const fn new(origin: Vec2, direction: Vec2) -> Self {
    Self { origin, direction }
  }
}

/// Represents a ray into 3-space.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray3 {
  pub origin: Vec3,
  pub direction: Vec3,
}

impl Ray3 {
  #[inline(always)]
  pub const fn new(origin: Vec3, direction: Vec3) -> Self {
    Self { origin, direction }
  }
}
