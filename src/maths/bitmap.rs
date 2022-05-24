use super::*;

/// A bitmap allows reified access to store a set of `P` in a 2d grid.
///
/// Each P is expected to have `CHANNEL_COUNT` elements.
pub trait Bitmap<P> {
  /// The number of elements per channel in P.
  const CHANNEL_COUNT: usize;

  /// The width of the bitmap, in units.
  fn width(&self) -> usize;

  /// The height of the bitmap, in units.
  fn height(&self) -> usize;

  /// Accesses a single item in the bitmap at the given (x, y) coordinates.
  fn get(&self, point: (usize, usize)) -> &P;

  /// Mutably accesses a single item in the bitmap at the given (x, y) coordinates.
  fn get_mut(&mut self, point: (usize, usize)) -> &mut P;
}

/// Blank implementation for any grid of pixels.
impl<P> Bitmap<P> for Grid<P> where P: crate::graphics::Pixel {
  const CHANNEL_COUNT: usize = P::CHANNEL_COUNT;

  fn width(&self) -> usize {
    self.width()
  }

  fn height(&self) -> usize {
    self.height()
  }

  fn get(&self, point: (usize, usize)) -> &P {
    &self[point]
  }

  fn get_mut(&mut self, point: (usize, usize)) -> &mut P {
    &mut self[point]
  }
}
