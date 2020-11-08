use std::collections::HashMap;

use crate::maths::{vec2, Vector2};

/// Permits iterating over the cells in a grid.
#[derive(Copy, Clone, Debug)]
pub struct GridCellIterator {
  size: usize,
  stride: usize,
  index: usize,
}

impl Iterator for GridCellIterator {
  type Item = Vector2<usize>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index < self.size {
      let x = self.index % self.stride;
      let y = self.index / self.stride;

      self.index += 1;

      Some(vec2(x, y))
    } else {
      None
    }
  }
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

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn get(&self, x: usize, y: usize) -> &T {
    assert!(x < self.width);
    assert!(y < self.height);

    &self.elements[x + y * self.width]
  }

  pub fn set(&mut self, x: usize, y: usize, value: T) {
    assert!(x < self.width);
    assert!(y < self.height);

    self.elements[x + y * self.width] = value;
  }

  #[inline]
  pub fn cells(&self) -> GridCellIterator {
    GridCellIterator {
      size: self.width() * self.height(),
      stride: self.width(),
      index: 0,
    }
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
    Self {
      elements: HashMap::new(),
    }
  }

  pub fn get(&self, x: i32, y: i32) -> Option<&T> {
    self.elements.get(&vec2(x, y))
  }

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
