use crate::collections::GridPoint;

/// Represents a type that can receive [`Raster`]ized output.
pub trait RasterTarget<T> {
  fn width(&self) -> usize;
  fn height(&self) -> usize;
  fn get(&self, point: impl Into<GridPoint>) -> Option<&T>;
  fn set(&mut self, point: impl Into<GridPoint>, value: T);

  /// Rasterizes the given object into the canvas.
  fn draw(&mut self, value: T, rasterable: &impl RasterSource)
  where
    T: Clone,
    Self: Sized,
  {
    rasterable.rasterize(value, self);
  }
}

/// Represents a type that can be rasterized into a [`Raster`].
pub trait RasterSource {
  fn rasterize<T: Clone>(&self, value: T, target: &mut impl RasterTarget<T>);
}
