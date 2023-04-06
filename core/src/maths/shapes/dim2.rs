use crate::maths::Vec2;

/// A sphere in 2-space.
#[derive(Clone, Debug)]
pub struct Circle {
  pub radius: f32,
  pub center: Vec2,
}

impl Default for Circle {
  fn default() -> Self {
    Self {
      radius: 1.0,
      center: Vec2::ZERO,
    }
  }
}

impl Circle {
  /// Creates a new circle.
  pub const fn new(radius: f32, center: Vec2) -> Self {
    Self { radius, center }
  }

  /// Creates a new circle with the given radius.
  pub fn with_radius(self, radius: f32) -> Self {
    Self { radius, ..self }
  }

  /// Creates a new circle with the given center.
  pub fn with_center(self, center: Vec2) -> Self {
    Self { center, ..self }
  }
}
