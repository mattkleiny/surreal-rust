use super::*;

/// A bounded rectangle in 2 dimensions formed from the two corner points.
#[derive(Default, Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rectangle {
  pub min: Vec2,
  pub max: Vec2,
}

impl Rectangle {
  /// Creates a new rectangle from the given corner points.
  #[inline]
  pub const fn new(min: Vec2, max: Vec2) -> Self {
    Self { min, max }
  }

  /// Creates a new rectangle from the given center and size.
  #[inline]
  pub fn from_size(center: Vec2, size: Vec2) -> Self {
    Self::new(center - size / 2., center + size / 2.)
  }

  /// Creates a new rectangle from the given corner points.
  #[inline]
  pub const fn from_corner_points(left: f32, top: f32, right: f32, bottom: f32) -> Self {
    Self::new(vec2(left, top), vec2(right, bottom))
  }

  /// The X position of the rectangle's lower-left corner.
  #[inline]
  pub fn x(&self) -> f32 {
    self.min.x
  }

  /// The Y position of the rectangle's lower-left corner.
  #[inline]
  pub fn y(&self) -> f32 {
    self.min.y
  }

  /// The minimum corner of the rectangle.
  #[inline]
  pub fn min(&self) -> Vec2 {
    self.min
  }

  /// The maximum corner of the rectangle.
  #[inline]
  pub fn max(&self) -> Vec2 {
    self.max
  }

  /// The left hand side of the rectangle.
  #[inline]
  pub fn left(&self) -> f32 {
    self.min.x
  }

  /// The right hand side of the rectangle.
  #[inline]
  pub fn right(&self) -> f32 {
    self.max.x
  }

  /// The top hand side of the rectangle.
  #[inline]
  pub fn top(&self) -> f32 {
    self.min.y
  }

  /// The bottom hand side of the rectangle.
  #[inline]
  pub fn bottom(&self) -> f32 {
    self.max.y
  }

  /// The center of the rectangle.
  #[inline]
  pub fn center(&self) -> Vec2 {
    (self.min + self.max) / 2.
  }

  /// The top left corner of the rectangle.
  #[inline]
  pub fn top_left(&self) -> Vec2 {
    vec2(self.left(), self.top())
  }

  /// The top right corner of the rectangle.
  #[inline]
  pub fn top_right(&self) -> Vec2 {
    vec2(self.right(), self.top())
  }

  /// The bottom left corner of the rectangle.
  #[inline]
  pub fn bottom_left(&self) -> Vec2 {
    vec2(self.left(), self.bottom())
  }

  /// The bottom right corner of the rectangle.
  #[inline]
  pub fn bottom_right(&self) -> Vec2 {
    vec2(self.right(), self.bottom())
  }

  /// The width of the rectangle.
  #[inline]
  pub fn width(&self) -> f32 {
    self.right() - self.left()
  }

  /// The height of the rectangle.
  #[inline]
  pub fn height(&self) -> f32 {
    self.bottom() - self.top()
  }

  /// The total area of the rectangle.
  #[inline]
  pub fn area(&self) -> f32 {
    self.width() * self.height()
  }

  /// The size of the rectangle in vector form.
  #[inline]
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
  #[allow(clippy::nonminimal_bool)]
  pub fn contains_point(&self, point: Vec2) -> bool {
    point.x >= self.min.x && point.y >= self.min.y && point.y <= self.max.y && point.y <= self.max.y
  }

  /// Extends this rectangle to include the given other rectangle.
  pub fn extend(&mut self, other: &Self) {
    self.min.x = self.min.x.min(other.min.x);
    self.min.y = self.min.y.min(other.min.y);
    self.max.x = self.max.x.max(other.max.x);
    self.max.y = self.max.y.max(other.max.y);
  }

  /// Determines if the rectangle contains the given other rectangle.
  pub fn intersects(&self, other: &Self) -> bool {
    // check if this rectangle intersects the given other rectangle
    self.min.x <= other.max.x && self.max.x >= other.min.x && self.min.y <= other.max.y && self.max.y >= other.min.y
  }

  /// Splits the rectangle into four quadrants.
  ///
  /// The quadrants are returned in the following order:
  /// * Top left
  /// * Top right
  /// * Bottom left
  /// * Bottom right
  pub fn into_quadrants(&self) -> [Self; 4] {
    let center = self.center();
    [
      Self::from_corner_points(self.left(), self.top(), center.x, center.y),
      Self::from_corner_points(center.x, self.top(), self.right(), center.y),
      Self::from_corner_points(self.left(), center.y, center.x, self.bottom()),
      Self::from_corner_points(center.x, center.y, self.right(), self.bottom()),
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_test_for_points() {
    let rect = Rectangle::from_corner_points(0., 0., 1., 1.);

    assert!(rect.contains_point(vec2(0.5, 0.5)));
    assert!(rect.contains_point(vec2(0., 0.)));
    assert!(rect.contains_point(vec2(1., 1.)));
    assert!(!rect.contains_point(vec2(1.1, 1.1)));
    assert!(!rect.contains_point(vec2(-0.1, -0.1)));
  }

  #[test]
  fn test_clamp_to_given_bounds() {
    let rect = Rectangle::from_corner_points(-1., -1., 1., 1.).clamp(0., 0., 1., 1.);

    assert_eq!(rect, Rectangle::from_corner_points(0., 0., 1., 1.));
  }

  #[test]
  fn test_extend_to_encapsulate_other_rectangle() {
    let mut rect = Rectangle::default();

    rect.extend(&Rectangle::from_corner_points(0., 0., 1., 1.));
    assert_eq!(rect, Rectangle::from_corner_points(0., 0., 1., 1.));

    rect.extend(&Rectangle::from_corner_points(2., 2., 3., 3.));
    assert_eq!(rect, Rectangle::from_corner_points(0., 0., 3., 3.));

    rect.extend(&Rectangle::from_corner_points(-1., -1., 0., 0.));
    assert_eq!(rect, Rectangle::from_corner_points(-1., -1., 3., 3.));
  }

  #[test]
  fn test_detect_intersection() {
    let rect = Rectangle::from_corner_points(0., 0., 1., 1.);

    assert!(rect.intersects(&Rectangle::from_corner_points(0., 0., 1., 1.)));
    assert!(rect.intersects(&Rectangle::from_corner_points(0.5, 0.5, 1.5, 1.5)));
    assert!(rect.intersects(&Rectangle::from_corner_points(-1., -1., 0.5, 0.5)));
    assert!(rect.intersects(&Rectangle::from_corner_points(0.5, 0.5, 2., 2.)));

    assert!(!rect.intersects(&Rectangle::from_corner_points(-1., -1., -0.1, -0.1)));
    assert!(!rect.intersects(&Rectangle::from_corner_points(1.1, 1.1, 2., 2.)));
  }

  #[test]
  fn test_compute_center() {
    let rect = Rectangle::from_corner_points(0., 0., 1., 1.);

    assert_eq!(rect.center(), vec2(0.5, 0.5));
  }

  #[test]
  fn test_compute_quadrants() {
    let rect = Rectangle::from_corner_points(0., 0., 2., 2.);

    let quadrants = rect.into_quadrants();

    assert_eq!(quadrants[0], Rectangle::from_corner_points(0., 0., 1., 1.));
    assert_eq!(quadrants[1], Rectangle::from_corner_points(1., 0., 2., 1.));
    assert_eq!(quadrants[2], Rectangle::from_corner_points(0., 1., 1., 2.));
    assert_eq!(quadrants[3], Rectangle::from_corner_points(1., 1., 2., 2.));
  }
}
