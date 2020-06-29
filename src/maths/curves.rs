use crate::maths::Vector2;

/// Represents a curve on a plane in 2-space.
pub trait PlanarCurve {
  fn sample(&self, t: f32) -> Vector2<f32>;
}

/// A linear curve in 2-space.
#[derive(Copy, Clone, Debug)]
pub struct Line {
  from: Vector2<f32>,
  to: Vector2<f32>,
}

impl Line {
  pub const fn new(from: Vector2<f32>, to: Vector2<f32>) -> Self {
    Self { from, to }
  }
}

impl PlanarCurve for Line {
  fn sample(&self, t: f32) -> Vector2<f32> {
    unimplemented!()
  }
}

/// A simple representation of bezier curves in 2-space.
#[derive(Copy, Clone, Debug)]
pub enum Bezier {
  Quadratic {
    start: Vector2<f32>,
    control: Vector2<f32>,
    end: Vector2<f32>,
  },
  Cubic {
    start: Vector2<f32>,
    control: Vector2<f32>,
    control2: Vector2<f32>,
    end: Vector2<f32>,
  },
}

impl PlanarCurve for Bezier {
  fn sample(&self, t: f32) -> Vector2<f32> {
    match self {
      Bezier::Quadratic { .. } => unimplemented!(),
      Bezier::Cubic { .. } => unimplemented!(),
    }
  }
}
