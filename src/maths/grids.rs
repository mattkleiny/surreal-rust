use std::collections::HashMap;

use crate::maths::{vec2, Vector2};

/// Represents (x, y) position in a grid.
pub type GridPoint = (usize, usize);

/// Permits grid-like access to elements using (x, y) indices.
pub trait Grid {
  type Target;

  fn width(&self) -> usize;
  fn height(&self) -> usize;

  /// Determines if the given grid point is valid.
  fn is_valid(&self, point: GridPoint) -> bool {
    let (x, y) = point;

    x < self.width() && y < self.height()
  }

  /// Reads the grid at the given (x, y) position.
  fn get(&self, point: GridPoint) -> &Self::Target;
}

/// Permits mutable grid-like access to elements using (x, y) indices.
pub trait GridMut: Grid {
  /// Mutably reads the grid at the given (x, y) point.
  fn get_mut(&mut self, point: GridPoint) -> &mut Self::Target;

  /// Sets the contents of the grid at the given (x, y) point.
  fn set(&mut self, point: GridPoint, value: Self::Target) {
    *self.get_mut(point) = value;
  }
}

/// A densely packed grid of T.
#[derive(Clone, Debug)]
pub struct DenseGrid<T> {
  width: usize,
  height: usize,
  elements: Vec<T>,
}

impl<T> DenseGrid<T> {
  pub fn new(width: usize, height: usize, default: T) -> Self where T: Clone {
    Self {
      width,
      height,
      elements: vec![default; width * height],
    }
  }

  pub fn width(&self) -> usize { self.width }
  pub fn height(&self) -> usize { self.height }

  pub fn get(&self, x: usize, y: usize) -> &T {
    assert!(x < self.width);
    assert!(y < self.height);

    &self.elements[x + y * self.width]
  }

  pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
    assert!(x < self.width);
    assert!(y < self.height);

    &mut self.elements[x + y * self.width]
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
  pub fn fill(&mut self, value: T) where T: Clone {
    for element in self.elements.iter_mut() {
      *element = value.clone();
    }
  }

  pub fn as_slice(&self) -> &[T] {
    &self.elements
  }

  pub fn as_mut_slice(&mut self) -> &mut [T] {
    &mut self.elements
  }
}

impl<T> Grid for DenseGrid<T> {
  type Target = T;

  fn width(&self) -> usize { self.width }
  fn height(&self) -> usize { self.height }

  fn get(&self, (x, y): (usize, usize)) -> &Self::Target {
    self.get(x, y)
  }
}

impl<T> GridMut for DenseGrid<T> {
  fn get_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Target {
    self.get_mut(x, y)
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

  pub fn get(&self, x: i32, y: i32) -> Option<&T> {
    self.elements.get(&vec2(x, y))
  }

  pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
    self.elements.get_mut(&vec2(x, y))
  }

  pub fn set(&mut self, x: i32, y: i32, value: T) {
    self.elements.insert(vec2(x, y), value);
  }

  /// Clears the contents of the grid.
  pub fn clear(&mut self) {
    self.elements.clear();
  }
}

impl<T> Grid for SparseGrid<T> {
  type Target = T;

  fn width(&self) -> usize { unimplemented!() }
  fn height(&self) -> usize { unimplemented!() }

  fn get(&self, (x, y): (usize, usize)) -> &Self::Target {
    self.get(x as i32, y as i32).unwrap()
  }
}

impl<T> GridMut for SparseGrid<T> {
  fn get_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Target {
    self.get_mut(x as i32, y as i32).unwrap()
  }
}

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn dense_grid_should_read_and_write() {
    let mut grid = DenseGrid::new(16, 9, 0);

    grid.set((0, 0), 128);

    assert_eq!(*grid.get(0, 0), 128);
  }

  #[test]
  fn dense_grid_should_fill_with_values() {
    let mut grid = DenseGrid::new(16, 9, 0);

    grid.fill(128);

    assert_eq!(*grid.get(0, 0), 128);
    assert_eq!(*grid.get(8, 4), 128);
    assert_eq!(*grid.get(15, 8), 128);
  }

  #[test]
  fn sparse_grid_should_read_and_write() {
    let mut grid = SparseGrid::new();

    grid.set(0, 0, 128);

    assert_eq!(grid.get(0, 0), Some(&128));
  }

  #[test]
  fn sparse_grid_should_clear_values() {
    let mut grid = SparseGrid::new();

    grid.set(0, 0, 128);
    grid.clear();

    assert_eq!(grid.get(0, 0), None);
  }
}
