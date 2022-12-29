use super::*;

/// A bounded rectangle in 2 dimensions formed from the two corner points.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rectangle {
  min: Vec2,
  max: Vec2,
}

impl Rectangle {
  /// Creates a new rectangle from the given corner points.
  pub fn new(min: Vec2, max: Vec2) -> Self {
    Self { min, max }
  }

  /// Creates a new rectangle from the given center and size.
  pub fn from_size(center: Vec2, size: Vec2) -> Self {
    Self::new(center - size / 2., center + size / 2.)
  }

  /// Creates a new rectangle from the given corner points.
  pub fn from_corner_points(left: f32, top: f32, right: f32, bottom: f32) -> Self {
    Self::new(vec2(left, top), vec2(right, bottom))
  }

  /// The minimum corner of the rectangle.
  pub fn min(&self) -> Vec2 {
    self.min
  }

  /// The maximum corner of the rectangle.
  pub fn max(&self) -> Vec2 {
    self.max
  }

  /// The left hand side of the rectangle.
  pub fn left(&self) -> f32 {
    self.min.x
  }

  /// The right hand side of the rectangle.
  pub fn right(&self) -> f32 {
    self.max.x
  }

  /// The top hand side of the rectangle.
  pub fn top(&self) -> f32 {
    self.min.y
  }

  /// The bottom hand side of the rectangle.
  pub fn bottom(&self) -> f32 {
    self.max.y
  }

  /// The top left corner of the rectangle.
  pub fn top_left(&self) -> Vec2 {
    vec2(self.left(), self.top())
  }

  /// The top right corner of the rectangle.
  pub fn top_right(&self) -> Vec2 {
    vec2(self.right(), self.top())
  }

  /// The bottom left corner of the rectangle.
  pub fn bottom_left(&self) -> Vec2 {
    vec2(self.left(), self.bottom())
  }

  /// The bottom right corner of the rectangle.
  pub fn bottom_right(&self) -> Vec2 {
    vec2(self.right(), self.bottom())
  }

  /// The width of the rectangle.
  pub fn width(&self) -> f32 {
    self.right() - self.left()
  }

  /// The height of the rectangle.
  pub fn height(&self) -> f32 {
    self.bottom() - self.top()
  }

  /// The total area of the rectangle.
  pub fn area(&self) -> f32 {
    self.width() * self.height()
  }

  /// The size of the rectangle in vector form.
  pub fn size(&self) -> Vec2 {
    vec2(self.width(), self.height())
  }

  /// Creates a new rectangle clamped to the given (X, Y) bounds.
  pub fn clamp(&self, left: f32, top: f32, right: f32, bottom: f32) -> Self {
    Self::from_corner_points(
      self.left().clamp(left, right),
      self.top().clamp(top, bottom),
      self.right().clamp(left, right),
      self.bottom().clamp(top, bottom),
    )
  }

  /// Determines if the rectangle contains the given point.
  pub fn contains_point(&self, point: Vec2) -> bool {
    point.x >= self.min.x && point.y >= self.min.y && point.y <= self.max.y && point.y <= self.max.y
  }
}
