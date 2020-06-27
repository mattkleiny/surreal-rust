use crate::maths::Vector2;

pub trait Curve {
  fn evaluate(&self, t: f32) -> f32;
}

#[derive(Copy, Clone, Debug)]
struct Line {
  from: f32,
  to: f32,
}

impl Curve for Line {
  #[inline]
  fn evaluate(&self, t: f32) -> f32 {
    unimplemented!()
  }
}

#[derive(Copy, Clone, Debug)]
struct Bezier {
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
