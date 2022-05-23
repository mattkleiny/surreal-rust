use super::*;

/// Shorthand to construct a [`Ray2`]
pub const fn ray2<T>(origin: Vector2<T>, direction: Vector2<T>) -> Ray2<T> {
  Ray2::new(origin, direction)
}

/// Shorthand to construct a [`Ray3`]
pub const fn ray3<T>(origin: Vector3<T>, direction: Vector3<T>) -> Ray3<T> {
  Ray3::new(origin, direction)
}

/// Represents a ray into 2-space.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ray2<T> {
  pub origin: Vector2<T>,
  pub direction: Vector2<T>,
}

impl<T> Ray2<T> {
  pub const fn new(origin: Vector2<T>, direction: Vector2<T>) -> Self {
    Self { origin, direction }
  }
}

/// Represents a ray into 3-space.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ray3<T> {
  pub origin: Vector3<T>,
  pub direction: Vector3<T>,
}

impl<T> Ray3<T> {
  pub const fn new(origin: Vector3<T>, direction: Vector3<T>) -> Self {
    Self { origin, direction }
  }
}
