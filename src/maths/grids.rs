use std::collections::HashMap;

use crate::maths::{vec2, Vector2};

/// A densely packed grid of T.
#[derive(Clone, Debug)]
pub struct DenseGrid<T> {
  width: usize,
  height: usize,
  elements: Vec<T>,
}

impl<T: Clone> DenseGrid<T> {
  pub fn new(width: usize, height: usize, default: T) -> Self {
    Self { width, height, elements: vec![default; width * height] }
  }

  #[inline]
  pub fn width(&self) -> usize { self.width }

  #[inline]
  pub fn height(&self) -> usize { self.height }

  #[inline]
  pub fn get(&self, x: usize, y: usize) -> &T {
    assert!(x < self.width);
    assert!(y < self.height);

    &self.elements[x + y * self.width]
  }

  #[inline]
  pub fn set(&mut self, x: usize, y: usize, value: T) {
    assert!(x < self.width);
    assert!(y < self.height);

    self.elements[x + y * self.width] = value;
  }

  /// Fills the grid with the given value
  pub fn fill(&mut self, value: T) {
    for y in 0..self.height {
      for x in 0..self.width {
        self.set(x, y, value.clone())
      }
    }
  }
}

/// A sparsely packed grid of T.
#[derive(Clone, Debug)]
pub struct SparseGrid<T> {
  elements: HashMap<Vector2<i32>, T>,
}

impl<T> SparseGrid<T> {
  pub fn new() -> Self {
    Self { elements: HashMap::new() }
  }

  #[inline]
  pub fn get(&self, x: i32, y: i32) -> Option<&T> {
    self.elements.get(&vec2(x, y))
  }

  #[inline]
  pub fn set(&mut self, x: i32, y: i32, value: T) {
    self.elements.insert(vec2(x, y), value);
  }

  /// Clears the contents of the grid.
  pub fn clear(&mut self) {
    self.elements.clear();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn dense_grid_should_read_and_write() {
    let mut grid = DenseGrid::<i32>::new(16, 9, 0);

    grid.set(0, 0, 128);

    assert_eq!(*grid.get(0, 0), 128);
  }

  #[test]
  fn dense_grid_should_fill_with_values() {
    let mut grid = DenseGrid::<i32>::new(16, 9, 0);

    grid.fill(128);

    assert_eq!(*grid.get(0, 0), 128);
    assert_eq!(*grid.get(8, 4), 128);
    assert_eq!(*grid.get(15, 8), 128);
  }

  #[test]
  fn sparse_grid_should_read_and_write() {
    let mut grid = SparseGrid::<i32>::new();

    grid.set(0, 0, 128);

    assert_eq!(grid.get(0, 0), Some(&128));
  }

  #[test]
  fn sparse_grid_should_clear_values() {
    let mut grid = SparseGrid::<i32>::new();

    grid.set(0, 0, 128);
    grid.clear();

    assert_eq!(grid.get(0, 0), None);
  }
}
