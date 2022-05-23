use crate::maths::Numeric;

use super::{vec2, Vector2};

/// A bounded space in 2 dimensions formed from the two corner points.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rectangle<T> {
  min: Vector2<T>,
  max: Vector2<T>,
}

impl<T> Rectangle<T> where T: Numeric {
  pub fn new(left: T, top: T, right: T, bottom: T) -> Self {
    Self::from(vec2(left, top), vec2(right, bottom))
  }

  pub fn from(min: Vector2<T>, max: Vector2<T>) -> Self {
    Self { min, max }
  }

  pub fn min(&self) -> Vector2<T> { self.min }
  pub fn max(&self) -> Vector2<T> { self.max }

  pub fn left(&self) -> T { self.min.x }
  pub fn right(&self) -> T { self.max.x }
  pub fn top(&self) -> T { self.min.y }
  pub fn bottom(&self) -> T { self.max.y }

  pub fn width(&self) -> T { self.right() - self.left() }
  pub fn height(&self) -> T { self.bottom() - self.top() }
  pub fn area(&self) -> T { self.width() * self.height() }

  pub fn size(&self) -> Vector2<T> {
    vec2(
      self.width(),
      self.height(),
    )
  }

  pub fn contains_point(&self, point: Vector2<T>) -> bool {
    point.x >= self.min.x &&
        point.y >= self.min.y &&
        point.y <= self.max.y &&
        point.y <= self.max.y
  }
}