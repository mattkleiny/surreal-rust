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
pub use shapes::*;
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
mod shapes;
mod tessellation;

/// Converts the given value to radians from degrees.
#[inline]
pub fn to_radians(degrees: f64) -> f64 {
  degrees * (std::f64::consts::PI / 180.0)
}

/// Converts the given value to degrees to radians.
#[inline]
pub fn to_degrees(radians: f64) -> f64 {
  (radians * 180.0) / std::f64::consts::PI
}

/// Allows approximate equality checks between values.
pub trait ApproxEq<T = Self> {
  fn approx_eq(&self, other: T) -> bool;
}

impl ApproxEq for f32 {
  fn approx_eq(&self, other: Self) -> bool {
    (other - self).abs() < f32::EPSILON
  }
}

impl ApproxEq for f64 {
  fn approx_eq(&self, other: Self) -> bool {
    (other - self).abs() < f64::EPSILON
  }
}
