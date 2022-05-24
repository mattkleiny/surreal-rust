//! Mathematical utilities.

pub use bitmap::*;
pub use bounds::*;
pub use curves::*;
pub use grids::*;
pub use interpolation::*;
pub use linear::*;
pub use neighbours::*;
pub use numbers::*;
pub use paths::*;
pub use random::*;
pub use ranges::*;
pub use tessellation::*;

mod bitmap;
mod bounds;
mod curves;
mod grids;
mod interpolation;
mod linear;
mod neighbours;
mod numbers;
mod paths;
mod random;
mod ranges;
mod tessellation;

/// Clamps the given value between the given lower and upper bounds.
pub fn clamp<T>(value: T, lower: T, upper: T) -> T
where
  T: Numeric,
{
  match () {
    _ if value > upper => upper,
    _ if value < lower => lower,
    _ => value,
  }
}

/// Converts the given value to radians from degrees.
pub fn to_radians(degrees: f64) -> f64 {
  degrees * (std::f64::consts::PI / 180.0)
}

/// Converts the given value to degrees to radians.
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
