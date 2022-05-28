use super::*;

/// A bounded space in 2 dimensions formed from the two corner points.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rectangle<T> {
  min: Vector2<T>,
  max: Vector2<T>,
}

impl<T> Rectangle<T> where T: Numeric {
  /// Creates a new rectangle from the given corner points.
  pub fn new(min: Vector2<T>, max: Vector2<T>) -> Self {
    Self { min, max }
  }

  /// Creates a new rectangle from the given center and size.
  pub fn from_size(center: Vector2<T>, size: Vector2<T>) -> Self {
    Self::new(
      center - size / T::from_f32(2.),
      center + size / T::from_f32(2.),
    )
  }

  /// Creates a new rectangle from the given corner points.
  pub fn from_corner_points(left: T, top: T, right: T, bottom: T) -> Self {
    Self::new(vec2(left, top), vec2(right, bottom))
  }

  pub fn min(&self) -> Vector2<T> { self.min }
  pub fn max(&self) -> Vector2<T> { self.max }

  pub fn left(&self) -> T { self.min.x }
  pub fn right(&self) -> T { self.max.x }
  pub fn top(&self) -> T { self.min.y }
  pub fn bottom(&self) -> T { self.max.y }

  pub fn top_left(&self) -> Vector2<T> { vec2(self.left(), self.top()) }
  pub fn top_right(&self) -> Vector2<T> { vec2(self.right(), self.top()) }
  pub fn bottom_left(&self) -> Vector2<T> { vec2(self.left(), self.bottom()) }
  pub fn bottom_right(&self) -> Vector2<T> { vec2(self.right(), self.bottom()) }

  pub fn width(&self) -> T { self.right() - self.left() }
  pub fn height(&self) -> T { self.bottom() - self.top() }

  pub fn area(&self) -> T { self.width() * self.height() }

  pub fn size(&self) -> Vector2<T> { vec2(self.width(), self.height()) }

  /// Creates a new rectangle clamped to the given (X, Y) bounds.
  pub fn clamp(&self, left: T, top: T, right: T, bottom: T) -> Self {
    Self::from_corner_points(
      clamp(self.left(), left, right),
      clamp(self.top(), top, bottom),
      clamp(self.right(), left, right),
      clamp(self.bottom(), top, bottom),
    )
  }

  /// Determines if the rectangle contains the given point.
  pub fn contains_point(&self, point: Vector2<T>) -> bool {
    point.x >= self.min.x &&
      point.y >= self.min.y &&
      point.y <= self.max.y &&
      point.y <= self.max.y
  }
}
