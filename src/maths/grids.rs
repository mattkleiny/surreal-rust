use super::*;

/// Represents a 2d point in a grid.
pub type GridPoint = (usize, usize);

/// Represents a type that can be rasterized into a grid.
pub trait GridRaster {
  /// Rasterizes the given shape into the given grid.
  fn draw_to<G, T>(&self, grid: &mut G, value: T) where G: Grid<T>, T: Clone;
}

/// Represents a 2d grid that allows efficient random access.
pub trait Grid<T>: Sized {
  /// Returns the stride/size between each row of the grid.
  fn stride(&self) -> usize;

  /// Returns the total length of the grid (width * height).
  fn length(&self) -> usize;

  /// Returns the width of the grid.
  fn width(&self) -> usize {
    self.stride()
  }

  /// Returns the height of the grid.
  fn height(&self) -> usize {
    self.length() / self.stride()
  }

  /// Accesses an item from the grid.
  fn get(&self, point: impl Into<GridPoint>) -> &T;

  /// Sets an item in the grid.
  fn set(&mut self, point: impl Into<GridPoint>, value: T);

  /// Fills the grid with the given value.
  fn fill(&mut self, value: T) where T: Clone {
    for y in 0..self.height() {
      for x in 0..self.width() {
        self.set((x, y), value.clone());
      }
    }
  }

  /// Clears the grid.
  fn clear(&mut self) where T: Clone + Default {
    self.fill(T::default());
  }

  /// Rasterizes a shape onto the grid.
  fn draw_shape(&mut self, shape: &impl GridRaster, value: T) where T: Clone {
    shape.draw_to(self, value);
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