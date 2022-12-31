//! Mathematical utilities.

pub use curves::*;
pub use interpolation::*;
pub use linear::*;
pub use neighbours::*;
pub use numbers::*;
pub use paths::*;
pub use random::*;
pub use ranges::*;
pub use rectangles::*;
pub use tessellation::*;

mod curves;
mod interpolation;
mod linear;
mod neighbours;
mod numbers;
mod paths;
mod random;
mod ranges;
mod rectangles;
mod tessellation;

const EPSILON: f32 = 0.00001;

/// A globally unique identifier.
pub type Guid = uuid::Uuid;

/// Converts the given value to radians from degrees.
#[inline]
pub fn to_radians(degrees: f32) -> f32 {
  degrees * (std::f32::consts::PI / 180.0)
}

/// Converts the given value to degrees to radians.
#[inline]
pub fn to_degrees(radians: f32) -> f32 {
  (radians * 180.0) / std::f32::consts::PI
}

/// Allows approximate equality checks between values.
pub trait ApproxEq<T = Self> {
  fn approx_eq(&self, other: T) -> bool;
}

impl ApproxEq for f32 {
  #[inline]
  fn approx_eq(&self, other: Self) -> bool {
    (other - self).abs() < EPSILON
  }
}

impl ApproxEq for f64 {
  #[inline]
  fn approx_eq(&self, other: Self) -> bool {
    (other - self).abs() < EPSILON as f64
  }
}
