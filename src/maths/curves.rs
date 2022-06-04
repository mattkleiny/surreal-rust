use super::*;

/// Represents a curve on a plane in 2-space.
pub trait Curve {
  fn sample_at(&self, t: f32) -> Vector2<f32>;
}

/// A linear curve in 2-space.
#[derive(Copy, Clone, Debug)]
pub struct Line {
  pub a: Vector2<f32>,
  pub b: Vector2<f32>,
}

impl Curve for Line {
  fn sample_at(&self, t: f32) -> Vector2<f32> {
    Vector2::lerp(self.a, self.b, t)
  }
}

/// Represents a quadratic bezier curve in 2-space.
#[derive(Copy, Clone, Debug)]
pub struct QuadraticBezier {
  pub start: Vector2<f32>,
  pub control: Vector2<f32>,
  pub end: Vector2<f32>,
}

impl Curve for QuadraticBezier {
  fn sample_at(&self, t: f32) -> Vector2<f32> {
    let x = (1. - t).powf(2.) * self.start.x
      + 2. * (1. - t) * t * self.control.x
      + t.powf(2.) * self.end.x;

    let y = (1. - t).powf(2.) * self.start.y
      + 2. * (1. - t) * t * self.control.y
      + t.powf(2.) * self.end.y;

    vec2(x, y)
  }
}

/// Represents a cubic bezier curve in 2-space.
#[derive(Copy, Clone, Debug)]
pub struct CubicBezier {
  pub start: Vector2<f32>,
  pub control1: Vector2<f32>,
  pub control2: Vector2<f32>,
  pub end: Vector2<f32>,
}

impl Curve for CubicBezier {
  fn sample_at(&self, _t: f32) -> Vector2<f32> {
    todo!()
  }
}
