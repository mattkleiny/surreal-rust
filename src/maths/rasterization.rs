/// Represents a type that can receive [`Raster`]ized output.
pub trait Raster<T> {
  fn width(&self) -> usize;
  fn height(&self) -> usize;
  fn get(&self, x: isize, y: isize) -> &T;
  fn set(&mut self, x: isize, y: isize, value: T);
}

/// Represents a type that can be rasterized into a [`Raster`].
pub trait Rasterable {
  fn rasterize<T: Clone>(&self, value: T, raster: &mut impl Raster<T>);
}
