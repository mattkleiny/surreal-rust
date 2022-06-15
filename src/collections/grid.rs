use crate::maths::{RasterTarget, Vector2};

/// Represents a point in a [`Grid`].
pub struct GridPoint(pub isize, pub isize);

/// A simple 2d grid of [`T`]s.
#[derive(Clone, Debug)]
pub struct Grid<T> {
  stride: usize,
  items: Vec<T>,
}

impl<T> Grid<T> {
  /// Creates a new grid with the given dimensions.
  pub fn new(width: usize, height: usize) -> Self
  where
    T: Clone + Default,
  {
    Self {
      stride: width,
      items: vec![T::default(); width * height],
    }
  }

  /// Converts the given slice into a grid.
  pub fn from_slice(stride: usize, slice: &[T]) -> Self
  where
    T: Clone,
  {
    Self {
      stride,
      items: slice.to_vec(),
    }
  }

  /// Returns the total length of the grid (width * height).
  #[inline]
  pub fn len(&self) -> usize {
    self.items.len()
  }

  /// Returns the stride/size between each row of the grid.
  #[inline]
  pub fn stride(&self) -> usize {
    self.stride
  }

  /// Returns the width of the grid.
  #[inline]
  pub fn width(&self) -> usize {
    self.stride()
  }

  /// Returns the height of the grid.
  #[inline]
  pub fn height(&self) -> usize {
    self.len() / self.stride()
  }

  /// Is the given point a valid index into the grid?
  #[inline]
  pub fn is_valid(&self, point: impl Into<GridPoint>) -> bool {
    let GridPoint(x, y) = point.into();

    self.is_valid_position(x, y)
  }

  /// Is the given position a valid index into the grid?
  #[inline]
  pub fn is_valid_position(&self, x: isize, y: isize) -> bool {
    x >= 0 && x < self.width() as isize && y >= 0 && y < self.height() as isize
  }

  /// Accesses an item from the grid.
  #[inline]
  pub fn get(&self, point: impl Into<GridPoint>) -> Option<&T> {
    let GridPoint(x, y) = point.into();

    if self.is_valid_position(x, y) {
      self.items.get(x as usize + y as usize * self.stride)
    } else {
      None
    }
  }

  /// Accesses an item from the grid without checking bounds.
  #[inline]
  pub fn get_unchecked(&self, point: impl Into<GridPoint>) -> &T {
    let GridPoint(x, y) = point.into();

    &self.items[x as usize + y as usize * self.stride]
  }

  /// Sets an item from the grid.
  #[inline]
  pub fn set(&mut self, point: impl Into<GridPoint>, value: T) {
    let GridPoint(x, y) = point.into();

    if self.is_valid_position(x, y) {
      self.items[x as usize + y as usize * self.stride] = value
    }
  }

  /// Sets an item from the grid without checking bounds.
  #[inline]
  pub fn set_unchecked(&mut self, point: impl Into<GridPoint>, value: T) {
    let GridPoint(x, y) = point.into();

    self.items[x as usize + y as usize * self.stride] = value
  }

  /// Fills the grid with the given value.
  pub fn fill(&mut self, value: T)
  where
    T: Clone,
  {
    self.items.fill(value);
  }

  /// Clears the grid.
  pub fn clear(&mut self)
  where
    T: Clone + Default,
  {
    self.fill(T::default());
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

/// Allow rasterization of shapes into the grid.
impl<T> RasterTarget<T> for Grid<T> {
  fn width(&self) -> usize {
    self.width()
  }

  fn height(&self) -> usize {
    self.height()
  }

  fn get(&self, point: impl Into<GridPoint>) -> Option<&T> {
    self.get(point)
  }

  fn set(&mut self, point: impl Into<GridPoint>, value: T) {
    self.set(point, value);
  }
}

/// Allows conversion into a GridPoint from tuples.
macro_rules! tuple_grid_point {
  ($type:ty) => {
    impl From<($type, $type)> for GridPoint {
      fn from(point: ($type, $type)) -> Self {
        Self(point.0 as isize, point.1 as isize)
      }
    }
  };
}

tuple_grid_point!(u8);
tuple_grid_point!(u16);
tuple_grid_point!(u32);
tuple_grid_point!(u64);
tuple_grid_point!(usize);
tuple_grid_point!(i8);
tuple_grid_point!(i16);
tuple_grid_point!(i32);
tuple_grid_point!(i64);
tuple_grid_point!(isize);

impl From<(f32, f32)> for GridPoint {
  fn from(point: (f32, f32)) -> Self {
    Self(point.0.floor() as isize, point.1.floor() as isize)
  }
}

impl From<(f64, f64)> for GridPoint {
  fn from(point: (f64, f64)) -> Self {
    Self(point.0.floor() as isize, point.1.floor() as isize)
  }
}

/// Allows conversion into a GridPoint from vectors.
macro_rules! vector_grid_point {
  ($type:ty) => {
    impl From<crate::maths::Vector2<$type>> for GridPoint {
      fn from(point: Vector2<$type>) -> Self {
        Self(point.x as isize, point.y as isize)
      }
    }
  };
}

vector_grid_point!(u8);
vector_grid_point!(u16);
vector_grid_point!(u32);
vector_grid_point!(u64);
vector_grid_point!(usize);
vector_grid_point!(i8);
vector_grid_point!(i16);
vector_grid_point!(i32);
vector_grid_point!(i64);
vector_grid_point!(isize);

impl From<Vector2<f32>> for GridPoint {
  fn from(point: Vector2<f32>) -> Self {
    Self(point.x.floor() as isize, point.y.floor() as isize)
  }
}

impl From<Vector2<f64>> for GridPoint {
  fn from(point: Vector2<f64>) -> Self {
    Self(point.x.floor() as isize, point.y.floor() as isize)
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
