//! Mathematical utilities.

pub use automata::*;
pub use bounds::*;
pub use curves::*;
pub use directions::*;
pub use grids::*;
pub use interp::*;
pub use linear::*;
pub use random::*;
pub use ranges::*;

mod automata;
mod bounds;
mod curves;
mod directions;
mod interp;
mod linear;
mod grids;
mod random;
mod ranges;

/// Clamps the given value between the given lower and upper bounds.
pub fn clamp<T: PartialOrd>(value: T, lower: T, upper: T) -> T {
  match () {
    _ if value > upper => upper,
    _ if value < lower => lower,
    _ => value
  }
}

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

/// Permits slicing the object into pieces.
pub trait Sliceable {
  type Output;

  fn subdivide(&self, size: (usize, usize)) -> &[Self::Output];
}
