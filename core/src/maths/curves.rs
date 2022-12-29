use super::*;

/// Represents a curve on a plane in 2-space.
pub trait Curve {
  fn sample_at(&self, t: f32) -> Vec2;
}

/// A linear curve in 2-space.
#[derive(Copy, Clone, Debug)]
pub struct Line {
  pub a: Vec2,
  pub b: Vec2,
}

impl Curve for Line {
  fn sample_at(&self, t: f32) -> Vec2 {
    Vec2::lerp(self.a, self.b, t)
  }
}

/// Represents a quadratic bezier curve in 2-space.
#[derive(Copy, Clone, Debug)]
pub struct QuadraticBezier {
  pub start: Vec2,
  pub control: Vec2,
  pub end: Vec2,
}

impl Curve for QuadraticBezier {
  fn sample_at(&self, t: f32) -> Vec2 {
    let x = (1. - t).powf(2.) * self.start.x + 2. * (1. - t) * t * self.control.x + t.powf(2.) * self.end.x;
    let y = (1. - t).powf(2.) * self.start.y + 2. * (1. - t) * t * self.control.y + t.powf(2.) * self.end.y;

    vec2(x, y)
  }
}

/// Represents a cubic bezier curve in 2-space.
#[derive(Copy, Clone, Debug)]
pub struct CubicBezier {
  pub start: Vec2,
  pub control1: Vec2,
  pub control2: Vec2,
  pub end: Vec2,
}

impl Curve for CubicBezier {
  fn sample_at(&self, _t: f32) -> Vec2 {
    todo!()
  }
}
