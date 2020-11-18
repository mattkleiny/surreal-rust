use crate::maths::Vector2;

/// Represents a curve on a plane in 2-space.
pub trait PlanarCurve {
  fn sample(&self, normal: f32) -> Vector2<f32>;
}

/// A linear curve in 2-space.
#[derive(Copy, Clone, Debug)]
pub struct Line {
  pub from: Vector2<f32>,
  pub to: Vector2<f32>,
}

impl PlanarCurve for Line {
  fn sample(&self, normal: f32) -> Vector2<f32> {
    unimplemented!()
  }
}

/// Represents a quadratic bezier curve in 2-space.
#[derive(Copy, Clone, Debug)]
pub struct QuadraticBezierCurve {
  pub start: Vector2<f32>,
  pub control: Vector2<f32>,
  pub end: Vector2<f32>,
}

impl PlanarCurve for QuadraticBezierCurve {
  fn sample(&self, normal: f32) -> Vector2<f32> {
    unimplemented!()
  }
}

/// Represents a cubic bezier curve in 2-space.
#[derive(Copy, Clone, Debug)]
pub struct CubicBezierCurve {
  pub start: Vector2<f32>,
  pub control1: Vector2<f32>,
  pub control2: Vector2<f32>,
  pub end: Vector2<f32>,
}

impl PlanarCurve for CubicBezierCurve {
  fn sample(&self, normal: f32) -> Vector2<f32> {
    unimplemented!()
  }
}