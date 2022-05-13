use crate::maths::Vector2;

/// Represents a plane in 3-space.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Plane<T> {
  pub distance: T,
  pub normal: Vector2<T>,
}

/// Represents a half-space in 3d; usually results from a plane split of the space.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum HalfSpace {
  Behind,
  Inline,
  Front,
}
