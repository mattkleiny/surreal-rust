//! Mathematical utilities.

pub use bounds::*;
pub use curves::*;
pub use interpolation::*;
pub use linear::*;
pub use neighbours::*;
pub use numbers::*;
pub use paths::*;
pub use random::*;
pub use ranges::*;
pub use rasterization::*;
pub use shapes::*;
pub use tessellation::*;

mod bounds;
mod curves;
mod interpolation;
mod linear;
mod neighbours;
mod numbers;
mod paths;
mod random;
mod ranges;
mod rasterization;
mod shapes;
mod tessellation;

/// Clamps the given value between the given lower and upper bounds.
pub fn clamp<T: Numeric>(value: T, lower: T, upper: T) -> T {
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

/// A symmetric matrix allows mapping one dimensional array ta symmetric square matrix and vice versa.
///
/// Such a matrix could be used to represent intersections or layer properties, for instance.
pub trait SymmetricMatrix<T>: std::ops::Index<usize, Output = T> {
  fn get_symmetric(&self, point: impl Into<crate::collections::GridPoint>) -> &T {
    let (x, y) = point.into();
    let index = calculate_symmetric_index(x, y);

    &self[index]
  }
}

// Maps (row, col) or (col, row) indices into a symmetric matrix to a 1D index.
fn calculate_symmetric_index(index_a: usize, index_b: usize) -> usize {
  // get the low and high indices
  let low = index_a.min(index_b);
  let high = index_a.max(index_b);

  // calculate the index (triangle number + offset into the row)
  let tri = triangle_number(high);
  let col = low;

  // Calculate the resulting index
  tri + col
}

/// Calculates the triangle number for N.
#[inline(always)]
fn triangle_number(n: usize) -> usize {
  n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn triangle_number_should_calculate_correctly() {
    assert_eq!(triangle_number(0), 0);
    assert_eq!(triangle_number(1), 1);
    assert_eq!(triangle_number(2), 3);
    assert_eq!(triangle_number(3), 6);
    assert_eq!(triangle_number(4), 10);

    let mut entries = Vec::new();
    
    entries.push(0);
    entries.push(1);
    entries.push(2);
    entries.push(3);
  }
}
