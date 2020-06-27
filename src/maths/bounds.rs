use std::ops::{Add, Div, Mul, Sub};

use crate::maths::{vec2, vec3, Vector2, Vector3};

/// A bounded space in 2 dimensions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bounds2<T> {
  min: Vector2<T>,
  max: Vector2<T>,
}

impl<T> Bounds2<T> where T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + PartialOrd {
  pub fn new(min: Vector2<T>, max: Vector2<T>) -> Self {
    Self { min, max }
  }

  pub fn size(&self) -> Vector2<T> {
    vec2(self.max.x - self.min.x,
         self.max.y - self.min.y)
  }

  pub fn contains_point(&self, point: Vector2<T>) -> bool {
    point.x >= self.min.x &&
        point.y >= self.min.y &&
        point.y <= self.max.y &&
        point.y <= self.max.y
  }
}

/// A bounded space in 3 dimensions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bounds3<T> {
  min: Vector3<T>,
  max: Vector3<T>,
}

impl<T> Bounds3<T> where T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + PartialOrd {
  pub fn new(min: Vector3<T>, max: Vector3<T>) -> Self {
    Self { min, max }
  }

  pub fn size(&self) -> Vector3<T> {
    vec3(self.max.x - self.min.x,
         self.max.y - self.min.y,
         self.max.z - self.min.z)
  }

  pub fn contains_point(&self, point: Vector3<T>) -> bool {
    point.x >= self.min.x &&
        point.y >= self.min.y &&
        point.y <= self.max.y &&
        point.y <= self.max.y &&
        point.z >= self.min.z &&
        point.z <= self.max.z
  }
}
