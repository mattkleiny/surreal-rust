use std::collections::HashMap;

use super::*;

// TODO: abstract over grid position?

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

  #[inline]
  pub fn width(&self) -> u32 { self.width }

  #[inline]
  pub fn height(&self) -> u32 { self.height }

  #[inline]
  pub fn get(&self, x: u32, y: u32) -> &T {
    let index = (x + y * self.width) as usize;

    // TODO: find a way to do this without panicking
    &self.elements.get(index).expect(&format!("Out of bounds grid access at ({}, {})", x, y))
  }

  #[inline]
  pub fn set(&mut self, x: u32, y: u32, element: T) {
    let index = (x + y * self.width) as usize;

    self.elements[index] = element;
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
  elements: HashMap<Vec2i, T>,
}

impl<T> SparseGrid<T> {
  pub fn new() -> Self {
    Self {
      elements: HashMap::new(),
    }
  }

  #[inline]
  pub fn width(&self) -> u32 { 0 }

  #[inline]
  pub fn height(&self) -> u32 { 0 }

  #[inline]
  pub fn get(&self, x: u32, y: u32) -> Option<&T> {
    let point = Vec2i::new(x as i32, y as i32);

    self.elements.get(&point)
  }

  #[inline]
  pub fn set(&mut self, x: u32, y: u32, element: T) {
    let point = Vec2i::new(x as i32, y as i32);

    self.elements.insert(point, element);
  }

  /// Clears the contents from the grid.
  pub fn clear(&mut self) {
    self.elements.clear();
  }
}

#[derive(Copy, Clone, Debug)]
pub enum Neighbourhood {
  Moore,
  VonNeumann,
}

impl Neighbourhood {
  pub fn get_adjacents(&self, point: Vec2i) -> Vec<Vec2i> {
    let results = match self {
      Neighbourhood::VonNeumann => {
        let mut results = Vec::with_capacity(4);

        results.push(Vec2i::new(point.x - 1, point.y));
        results.push(Vec2i::new(point.x + 1, point.y));
        results.push(Vec2i::new(point.x, point.y - 1));
        results.push(Vec2i::new(point.x, point.y + 1));

        results
      }
      Neighbourhood::Moore => {
        let mut results = Vec::with_capacity(8);

        results.push(Vec2i::new(point.x - 1, point.y));
        results.push(Vec2i::new(point.x + 1, point.y));
        results.push(Vec2i::new(point.x, point.y - 1));
        results.push(Vec2i::new(point.x, point.y + 1));

        results.push(Vec2i::new(point.x - 1, point.y - 1));
        results.push(Vec2i::new(point.x - 1, point.y + 1));
        results.push(Vec2i::new(point.x + 1, point.y - 1));
        results.push(Vec2i::new(point.x + 1, point.y + 1));

        results
      }
    };

    results
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

  #[test]
  fn neighbourood_should_produce_valid_adjacent_points() {
    assert_eq!(Neighbourhood::VonNeumann.get_adjacents(Vec2i::ZERO).len(), 4);
    assert_eq!(Neighbourhood::Moore.get_adjacents(Vec2i::ZERO).len(), 8);
  }
}
