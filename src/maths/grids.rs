use crate::collections::Grid;

/// Represents a type that can be rasterized into a grid.
pub trait Raster {
  /// Rasterizes the given shape into the given grid.
  fn draw_to<T: Clone>(&self, grid: &mut Grid<T>, value: T);
}
