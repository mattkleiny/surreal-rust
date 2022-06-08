//! Collections and data structures.

use std::ops::Index;

pub use anymap::*;
pub use arena::*;
pub use grid::*;
pub use multimap::*;
pub use ringbuffer::*;

mod anymap;
mod arena;
mod grid;
mod multimap;
mod ringbuffer;

/// A symmetric matrix allows mapping one dimensional array ta symmetric square matrix and vice versa.
///
/// Such a matrix could be used to represent intersections or layer properties, for instance.
pub trait SymmetricMatrix<T>: Index<usize, Output = T> {
  fn get_symmetric(&self, point: impl Into<GridPoint>) -> &T {
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
