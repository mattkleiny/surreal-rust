use crate::maths::{Vector2, Vector3};

/// A sampler that is capable of interpolating `Property`s for animation.
pub trait Sampler<P> {}

pub struct AnimationCurve {}

/// A property that can be sampled by a `Sampler`.
trait Property: Sized {
  fn add(&self, other: &Self) -> Self;
  fn sub(&self, other: &Self) -> Self;
  fn mul(&self, scalar: f32) -> Self;
  fn magnitude_sqr(&self) -> f32;

  fn magnitude(&self) -> f32 {
    self.magnitude_sqr().sqrt()
  }

  fn normalize(&self) -> Self {
    self.mul(1. / self.magnitude())
  }
}

impl Property for Vector2<f32> {
  fn add(&self, other: &Self) -> Self { self + other }
  fn sub(&self, other: &Self) -> Self { self - other }
  fn mul(&self, scalar: f32) -> Self { self * scalar }
  fn magnitude_sqr(&self) -> f32 { self.magnitude() }
}

impl Property for Vector3<f32> {
  fn add(&self, other: &Self) -> Self { self + other }
  fn sub(&self, other: &Self) -> Self { self - other }
  fn mul(&self, scalar: f32) -> Self { self * scalar }
  fn magnitude_sqr(&self) -> f32 { self.magnitude() }
}

impl Property for f32 {
  fn add(&self, other: &Self) -> Self { self + other }
  fn sub(&self, other: &Self) -> Self { self - other }
  fn mul(&self, scalar: f32) -> Self { self * scalar }
  fn magnitude_sqr(&self) -> f32 { *self }
}