//! Image loading and manipulation.

/// Represents an image of pixels.
pub trait Image<P> {
  /// Returns the width of the image.
  fn width(&self) -> u32;

  /// Returns the width of the image.
  fn height(&self) -> u32;

  /// Returns the pixels of the image.
  fn pixels(&self) -> &[P];
}
