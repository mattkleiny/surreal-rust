//! Grid abstractions.

use std::collections::HashMap;

use super::*;

/// Represents a grid of T elements in 2-space.
pub trait Grid<T> {
  fn width(&self) -> usize;
  fn height(&self) -> usize;
  fn get(&self, x: usize, y: usize) -> &T;
  fn set(&mut self, x: usize, y: usize, element: T);
}

/// A densely packed grid of T.
#[derive(Clone, Debug)]
pub struct DenseGrid<T> {
  width: usize,
  height: usize,
  elements: Vec<T>,
}

impl<T: Clone> DenseGrid<T> {
  pub fn new(width: usize, height: usize, default: T) -> Self {
    Self {
      width,
      height,
      elements: vec![default; width * height],
    }
  }
}

impl<T> Grid<T> for DenseGrid<T> {
  fn width(&self) -> usize { self.width }
  fn height(&self) -> usize { self.height }

  #[inline]
  fn get(&self, x: usize, y: usize) -> &T {
    &self.elements[x + y * self.width]
  }

  #[inline]
  fn set(&mut self, x: usize, y: usize, element: T) {
    self.elements[x + y * self.width] = element;
  }
}

/// A sparsely packed grid of T.
#[derive(Clone, Debug)]
pub struct SparseGrid<T> {
  elements: HashMap<Point2d, T>,
}

impl<T> SparseGrid<T> {
  pub fn new() -> Self {
    Self {
      elements: HashMap::new(),
    }
  }
}

impl<T> Grid<T> for SparseGrid<T> {
  // TODO: compute width/height
  fn width(&self) -> usize { 0 }
  fn height(&self) -> usize { 0 }

  #[inline]
  fn get(&self, x: usize, y: usize) -> &T {
    let point = Point2d::new(x, y);
    self.elements.get(&point).unwrap()
  }

  #[inline]
  fn set(&mut self, x: usize, y: usize, element: T) {
    let point = Point2d::new(x, y);
    self.elements.insert(point, element);
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
  fn sparse_grid_should_read_and_write() {
    let mut grid = SparseGrid::<i32>::new();

    grid.set(0, 0, 128);

    assert_eq!(*grid.get(0, 0), 128);
  }
}