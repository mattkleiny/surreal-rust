use crate::maths::{Raster, Vector2};

/// Represents a point in a [`Grid`].
pub type GridPoint = (usize, usize);

/// A simple 2d grid of [`T`]s.
#[derive(Clone, Debug)]
pub struct Grid<T> {
  stride: usize,
  items: Vec<T>,
}

impl<T> Grid<T> {
  /// Creates a new grid with the given dimensions.
  pub fn new(width: usize, height: usize) -> Self
  where T: Clone + Default {
    Self {
      stride: width,
      items: vec![T::default(); width * height],
    }
  }

  /// Converts the given slice into a grid.
  pub fn from_slice(stride: usize, slice: &[T]) -> Self
  where T: Clone {
    Self {
      stride,
      items: slice.to_vec(),
    }
  }

  /// Returns the total length of the grid (width * height).
  pub fn len(&self) -> usize {
    self.items.len()
  }

  /// Returns the stride/size between each row of the grid.
  pub fn stride(&self) -> usize {
    self.stride
  }

  /// Returns the width of the grid.
  pub fn width(&self) -> usize {
    self.stride()
  }

  /// Returns the height of the grid.
  pub fn height(&self) -> usize {
    self.len() / self.stride()
  }

  /// Is the given point a valid index into the grid?
  pub fn is_valid(&self, point: impl Into<GridPoint>) -> bool {
    let point = point.into();

    point.0 > 0 && point.0 < self.width() && point.1 > 0 && point.1 < self.height()
  }

  /// Accesses an item from the grid.
  pub fn get(&self, point: impl Into<GridPoint>) -> &T {
    let (x, y) = point.into();

    &self.items[x + y * self.stride]
  }

  /// Sets an item from the grid.
  pub fn set(&mut self, point: impl Into<GridPoint>, value: T) {
    let (x, y) = point.into();

    self.items[x + y * self.stride] = value
  }

  /// Fills the grid with the given value.
  pub fn fill(&mut self, value: T)
  where T: Clone {
    self.items.fill(value);
  }

  /// Clears the grid.
  pub fn clear(&mut self)
  where T: Clone + Default {
    self.fill(T::default());
  }

  /// Rasterizes a shape onto the grid.
  pub fn draw_shape(&mut self, shape: &impl Raster, value: T)
  where T: Clone {
    // TODO: split this into two traits (Raster and Rasterable)
    shape.draw_to(self, value);
  }

  /// Returns the items as a slice.
  pub fn as_slice(&self) -> &[T] {
    self.items.as_slice()
  }

  /// Returns the items as a mutable slice.
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    self.items.as_mut_slice()
  }
}

/// Allows conversion into a GridPoint.
macro_rules! implement_grid_point {
  ($type:ty) => {
    impl From<crate::maths::Vector2<$type>> for GridPoint {
      fn from(point: Vector2<$type>) -> Self {
        (point.x as usize, point.y as usize)
      }
    }
  };
}

implement_grid_point!(u8);
implement_grid_point!(u16);
implement_grid_point!(u32);
implement_grid_point!(u64);
implement_grid_point!(usize);
implement_grid_point!(i16);
implement_grid_point!(i32);
implement_grid_point!(i64);
implement_grid_point!(isize);

impl From<Vector2<f32>> for GridPoint {
  fn from(point: Vector2<f32>) -> Self {
    (point.x.floor() as usize, point.y.floor() as usize)
  }
}

impl From<Vector2<f64>> for GridPoint {
  fn from(point: Vector2<f64>) -> Self {
    (point.x.floor() as usize, point.y.floor() as usize)
  }
}
#[cfg(test)]
mod tests {
  use crate::graphics::Color32;
  use crate::maths::FromRandom;

  use super::*;

  #[test]
  fn grid_should_read_and_write_elements() {
    let mut grid = Grid::new(128, 128);

    grid.fill(Color32::BLACK);

    for y in 0..grid.height() {
      for x in 0..grid.width() {
        grid.set((x, y), Color32::random());
      }
    }
  }
}
