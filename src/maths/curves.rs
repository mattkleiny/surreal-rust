use crate::maths::{vec2, Vector2};

pub const LINEAR: Line = Line::new(vec2(0., 0.), vec2(1., 1.));

pub trait Curve {
  fn evaluate(&self, t: f32) -> f32;
}

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

impl Curve for Line {
  #[inline]
  fn evaluate(&self, t: f32) -> f32 {
    unimplemented!()
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Bezier {
  pub point1: Vector2<f32>,
  pub point2: Vector2<f32>,
  pub control: Vector2<f32>,
}

impl Curve for Bezier {
  #[inline]
  fn evaluate(&self, t: f32) -> f32 {
    unimplemented!()
  }
}
