use super::*;

/// Represents a ray into 2-space.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ray2<T> {
  pub origin: Vector2<T>,
  pub direction: Vector2<T>,
}

/// Represents a ray into 3-space.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ray3<T> {
  pub origin: Vector3<T>,
  pub direction: Vector3<T>,
}
