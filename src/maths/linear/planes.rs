use super::*;

/// Represents a plane in 3-space.
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Plane<T> {
  pub normal: Vector3<T>,
  pub distance: T,
}

/// Represents a half-space in 3d; usually results from a plane split of the space.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum HalfSpace {
  Behind,
  Inline,
  Front,
}
