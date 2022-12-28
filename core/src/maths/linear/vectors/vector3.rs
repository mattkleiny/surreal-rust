use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

use crate::maths::{Lerp, Matrix4x4, Numeric, Range};

/// Shorthand to construct a [`Vector3`].
#[inline(always)]
pub const fn vec3<T: Numeric>(x: T, y: T, z: T) -> Vector3<T> {
  Vector3::new(x, y, z)
}

/// A standard purpose 3d vector
#[repr(C)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Vector3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T: Numeric> Vector3<T> {
  pub const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO);
  pub const UNIT_X: Self = Self::new(T::ONE, T::ZERO, T::ZERO);
  pub const UNIT_Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO);
  pub const UNIT_Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE);
  pub const ONE: Self = Self::new(T::ONE, T::ONE, T::ONE);

  /// Creates a new vector from the given components.
  #[inline(always)]
  pub const fn new(x: T, y: T, z: T) -> Self {
    Self { x, y, z }
  }

  /// Clamps the (x, y, z) components of the vector to the given range.
  #[inline(always)]
  pub fn clamp(&self, range: Range<T>) -> Self {
    Self::new(range.clamp(self.x), range.clamp(self.y), range.clamp(self.z))
  }
}

impl Vector3<f32> {
  /// Computes the magnitude of this vector; the length essentially.
  #[inline(always)]
  pub fn magnitude(&self) -> f32 {
    let x2 = self.x * self.x;
    let y2 = self.y * self.y;
    let z2 = self.z * self.z;

    (x2 + y2 + z2).sqrt()
  }

  /// Normalizes the vector to the range (-1, 1) for all components.
  #[inline(always)]
  pub fn normalize(&self) -> Self {
    let magnitude = self.magnitude();

    Self {
      x: self.x / magnitude,
      y: self.y / magnitude,
      z: self.z / magnitude,
    }
  }
  /// Computes the dot product of this vector and another.
  ///
  /// The dot product represents the 'shadow' of the other vector on this one.
  #[inline(always)]
  pub fn dot(&self, other: Self) -> f32 {
    let x = self.x * other.x;
    let y = self.y * other.y;
    let z = self.z * other.z;

    x + y + z
  }

  /// Computes the cross product of this vector and another.
  ///
  /// The cross product is a vector perpendicular to both vectors.
  #[inline(always)]
  pub fn cross(&self, other: Self) -> Self {
    let x = self.y * other.z - self.z * other.y;
    let y = self.z * other.x - self.x * other.z;
    let z = self.x * other.y - self.y * other.x;

    vec3(x, y, z)
  }

  #[inline(always)]
  /// Reflects a vector about the given normal.
  pub fn reflect(self, normal: Self) -> Self {
    self - normal * 2. * self.dot(normal)
  }
}

impl<T: Numeric> From<[T; 3]> for Vector3<T> {
  fn from([x, y, z]: [T; 3]) -> Self {
    Vector3::new(x, y, z)
  }
}

impl<T: Numeric> Index<usize> for Vector3<T> {
  type Output = T;

  #[inline(always)]
  fn index(&self, index: usize) -> &Self::Output {
    match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => panic!("Index out of range!"),
    }
  }
}

impl<T: Numeric> IndexMut<usize> for Vector3<T> {
  #[inline(always)]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    match index {
      0 => &mut self.x,
      1 => &mut self.y,
      2 => &mut self.z,
      _ => panic!("Index out of range!"),
    }
  }
}

impl<T: Numeric> Add for Vector3<T> {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl<T: Numeric> Sub for Vector3<T> {
  type Output = Self;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl<T: Numeric> Mul for Vector3<T> {
  type Output = Self;

  #[inline(always)]
  fn mul(self, rhs: Self) -> Self::Output {
    Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
  }
}

impl<T: Numeric> Div for Vector3<T> {
  type Output = Self;

  #[inline(always)]
  fn div(self, rhs: Self) -> Self::Output {
    Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
  }
}

impl<T: Numeric> Mul<T> for Vector3<T> {
  type Output = Self;

  #[inline(always)]
  fn mul(self, rhs: T) -> Self::Output {
    Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
  }
}

impl<T: Numeric> Div<T> for Vector3<T> {
  type Output = Self;

  #[inline(always)]
  fn div(self, rhs: T) -> Self::Output {
    Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
  }
}

impl<T: Numeric> From<(T, T, T)> for Vector3<T> {
  #[inline(always)]
  fn from((x, y, z): (T, T, T)) -> Self {
    Self::new(x, y, z)
  }
}

impl<T: Numeric + Lerp> Lerp for Vector3<T> {
  #[inline(always)]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self::new(T::lerp(a.x, b.x, t), T::lerp(a.y, b.y, t), T::lerp(a.z, b.z, t))
  }
}

impl Mul<Matrix4x4<f32>> for Vector3<f32> {
  type Output = Self;

  #[inline(always)]
  fn mul(self, rhs: Matrix4x4<f32>) -> Self::Output {
    Self::new(
      self.x * rhs[(0, 0)] + self.y * rhs[(1, 0)] + self.z * rhs[(2, 0)] + rhs[(3, 0)],
      self.x * rhs[(0, 1)] + self.y * rhs[(1, 1)] + self.z * rhs[(2, 1)] + rhs[(3, 1)],
      self.x * rhs[(0, 2)] + self.y * rhs[(1, 2)] + self.z * rhs[(2, 2)] + rhs[(3, 2)],
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vector3_should_transform_by_identity_matrix() {
    let transform = Matrix4x4::IDENTITY;
    let position = vec3(1., 2., 3.);
    let result = position * transform;

    assert_eq!(result, position);
  }

  #[test]
  fn vector3_should_transform_by_translation_matrix() {
    let transform = Matrix4x4::from_translation(1., 2., 3.);
    let result = vec3(1., 1., 1.) * transform;

    assert_eq!(result, vec3(2., 3., 4.));
  }
}
