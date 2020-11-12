use std::ops::Sub;

use crate::maths::{vec2, vec3, Vector2, Vector3};

pub type Rect<T> = Bounds2<T>;
pub type Volume<T> = Bounds3<T>;

/// A bounded space in 2 dimensions formed from the two corner points.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bounds2<T> {
  min: Vector2<T>,
  max: Vector2<T>,
}

impl<T: Copy> Bounds2<T> {
  pub fn new(left: T, top: T, right: T, bottom: T) -> Self {
    Self {
      min: vec2(left, top),
      max: vec2(right, bottom),
    }
  }

  pub fn min(&self) -> Vector<T> { self.min }
  pub fn max(&self) -> Vector<T> { self.max }

  pub fn left(&self) -> T { self.min.x }
  pub fn right(&self) -> T { self.max.x }
  pub fn top(&self) -> T { self.min.y }
  pub fn bottom(&self) -> T { self.max.y }

  pub fn width(&self) -> T where T: Sub<Output=T> { self.right() - self.left() }
  pub fn height(&self) -> T where T: Sub<Output=T> { self.bottom() - self.top() }

  pub fn size(&self) -> Vector2<T> where T: Sub<Output=T> {
    vec2(
      self.max.x - self.min.x,
      self.max.y - self.min.y,
    )
  }

  pub fn contains_point(&self, point: Vector2<T>) -> bool where T: PartialOrd {
    point.x >= self.min.x &&
        point.y >= self.min.y &&
        point.y <= self.max.y &&
        point.y <= self.max.y
  }
}

/// A bounded space in 3 dimensions formed from the two corner points.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bounds3<T> {
  min: Vector3<T>,
  max: Vector3<T>,
}

impl<T> Bounds3<T> {
  pub fn new(min: Vector3<T>, max: Vector3<T>) -> Self {
    Self { min, max }
  }

  pub fn size(&self) -> Vector3<T>
    where
        T: Copy + Sub<Output=T>,
  {
    vec3(
      self.max.x - self.min.x,
      self.max.y - self.min.y,
      self.max.z - self.min.z,
    )
  }

  pub fn contains_point(&self, point: Vector3<T>) -> bool
    where
        T: PartialOrd,
  {
    point.x >= self.min.x
        && point.y >= self.min.y
        && point.y <= self.max.y
        && point.y <= self.max.y
        && point.z >= self.min.z
        && point.z <= self.max.z
  }
}
