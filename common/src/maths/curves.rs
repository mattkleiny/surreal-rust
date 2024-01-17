use super::*;

/// Represents a curve on a plane in 2-space.
pub trait Curve {
  fn evaluate(&self, t: f32) -> f32;
}

/// A linear curve in 2-space.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Line {
  pub a: Vec2,
  pub b: Vec2,
}

/// Represents a quadratic bezier curve in 2-space.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct QuadraticBezier {
  pub start: Vec2,
  pub control: Vec2,
  pub end: Vec2,
}

/// Represents a cubic bezier curve in 2-space.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct CubicBezier {
  pub start: Vec2,
  pub control1: Vec2,
  pub control2: Vec2,
  pub end: Vec2,
}

/// Allow arbitrary functions as curves.
impl<F: Fn(f32) -> f32> Curve for F {
  fn evaluate(&self, t: f32) -> f32 {
    self(t)
  }
}
