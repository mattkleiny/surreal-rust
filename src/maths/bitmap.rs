use crate::graphics::Pixel;

use super::*;

/// A bitmap allows reified access to store a set of `P` in a 2d grid.
///
/// Each P is expected to have `CHANNEL_COUNT` elements.
pub trait Bitmap<P: Pixel>: Grid<P> {
  /// The number of elements per channel in P.
  const CHANNEL_COUNT: usize;
}

// TODO: bitmap set operations?
// TODO: sdf operations?