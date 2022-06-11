use crate::collections::GridPoint;

/// Represents a type that can receive [`Raster`]ized output.
pub trait RasterCanvas<T> {
  fn width(&self) -> usize;
  fn height(&self) -> usize;
  fn get(&self, point: impl Into<GridPoint>) -> Option<&T>;
  fn set(&mut self, point: impl Into<GridPoint>, value: T);

  /// Rasterizes the given object into the canvas.
  fn draw(&mut self, value: T, rasterable: &impl Rasterable)
  where
    T: Clone,
    Self: Sized,
  {
    rasterable.rasterize(value, self);
  }
}

/// Represents a type that can be rasterized into a [`Raster`].
pub trait Rasterable {
  fn rasterize<T: Clone>(&self, value: T, canvas: &mut impl RasterCanvas<T>);
}
