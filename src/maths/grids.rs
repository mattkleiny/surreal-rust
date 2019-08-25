//! Grid abstractions.

use std::collections::HashMap;

use super::*;

/// Represents a grid of T elements in 2-space.
pub trait Grid<T> {
  fn width(&self) -> u32;
  fn height(&self) -> u32;
  fn get(&self, x: u32, y: u32) -> &T;
  fn set(&mut self, x: u32, y: u32, element: T);
}

/// A densely packed grid of T.
#[derive(Clone, Debug)]
pub struct DenseGrid<T> {
  width: u32,
  height: u32,
  elements: Vec<T>,
}

impl<T: Clone> DenseGrid<T> {
  pub fn new(width: u32, height: u32, default: T) -> Self {
    Self {
      width,
      height,
      elements: vec![default; (width * height) as usize],
    }
  }
}

impl<T> Grid<T> for DenseGrid<T> {
  #[inline]
  fn width(&self) -> u32 { self.width }

  #[inline]
  fn height(&self) -> u32 { self.height }

  #[inline]
  fn get(&self, x: u32, y: u32) -> &T {
    let index = (x + y * self.width) as usize;

    // TODO: find a way to do this without panicking
    &self.elements.get(index).expect(&format!("Out of bounds grid access at ({}, {})", x, y))
  }

  #[inline]
  fn set(&mut self, x: u32, y: u32, element: T) {
    let index = (x + y * self.width) as usize;

    self.elements[index] = element;
  }
}

/// A sparsely packed grid of T.
#[derive(Clone, Debug)]
pub struct SparseGrid<T> {
  elements: HashMap<Vec2i, T>,
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
  #[inline]
  fn width(&self) -> u32 { 0 }

  #[inline]
  fn height(&self) -> u32 { 0 }

  #[inline]
  fn get(&self, x: u32, y: u32) -> &T {
    let point = Vec2i::new(x as i32, y as i32);

    // TODO: find a way to do this without panicking
    self.elements.get(&point).expect(&format!("Out of bounds grid access at ({}, {})", x, y))
  }

  #[inline]
  fn set(&mut self, x: u32, y: u32, element: T) {
    let point = Vec2i::new(x as i32, y as i32);

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