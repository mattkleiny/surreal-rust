use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

use crate::maths::{Lerp, Matrix4x4, Numeric, Range};

/// Shorthand to construct a [`Vector4`].
#[inline(always)]
pub const fn vec4<T: Numeric>(x: T, y: T, z: T, w: T) -> Vector4<T> {
  Vector4::new(x, y, z, w)
}

/// A standard purpose 4d vector
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Vector4<T> {
  pub x: T,
  pub y: T,
  pub z: T,
  pub w: T,
}

impl<T: Numeric> Vector4<T> {
  pub const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
  pub const UNIT_X: Self = Self::new(T::ONE, T::ZERO, T::ZERO, T::ZERO);
  pub const UNIT_Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);
  pub const UNIT_Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);
  pub const UNIT_W: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
  pub const ONE: Self = Self::new(T::ONE, T::ONE, T::ONE, T::ONE);

  /// Creates a new vector from the given components.
  #[inline(always)]
  pub const fn new(x: T, y: T, z: T, w: T) -> Self {
    Self { x, y, z, w }
  }

  /// Clamps the (x, y, z, w) components of the vector to the given range.
  #[inline(always)]
  pub fn clamp(&self, range: Range<T>) -> Self {
    Self::new(
      range.clamp(self.x),
      range.clamp(self.y),
      range.clamp(self.z),
      range.clamp(self.w),
    )
  }
}

impl<T: Numeric> Index<usize> for Vector4<T> {
  type Output = T;

  #[inline(always)]
  fn index(&self, index: usize) -> &Self::Output {
    match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      3 => &self.w,
      _ => panic!("Index out of range!"),
    }
  }
}

impl<T: Numeric> IndexMut<usize> for Vector4<T> {
  #[inline(always)]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    match index {
      0 => &mut self.x,
      1 => &mut self.y,
      2 => &mut self.z,
      3 => &mut self.w,
      _ => panic!("Index out of range!"),
    }
  }
}

impl<T: Numeric> Add for Vector4<T> {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
  }
}

impl<T: Numeric> Sub for Vector4<T> {
  type Output = Self;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
  }
}

impl<T: Numeric> Mul for Vector4<T> {
  type Output = Self;

  #[inline(always)]
  fn mul(self, rhs: Self) -> Self::Output {
    Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.w)
  }
}

impl<T: Numeric> Div for Vector4<T> {
  type Output = Self;

  #[inline(always)]
  fn div(self, rhs: Self) -> Self::Output {
    Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z, self.w / rhs.w)
  }
}

impl<T: Numeric> Mul<T> for Vector4<T> {
  type Output = Self;

  #[inline(always)]
  fn mul(self, rhs: T) -> Self::Output {
    Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
  }
}

impl<T: Numeric> Div<T> for Vector4<T> {
  type Output = Self;

  #[inline(always)]
  fn div(self, rhs: T) -> Self::Output {
    Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
  }
}

impl<T: Numeric> From<(T, T, T, T)> for Vector4<T> {
  #[inline(always)]
  fn from((x, y, z, w): (T, T, T, T)) -> Self {
    Self::new(x, y, z, w)
  }
}

impl<T: Numeric> Lerp for Vector4<T> {
  #[inline(always)]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self::new(
      T::lerp(a.x, b.x, t),
      T::lerp(a.y, b.y, t),
      T::lerp(a.z, b.z, t),
      T::lerp(a.w, b.w, t),
    )
  }
}

impl Mul<Matrix4x4> for Vector4<f32> {
  type Output = Self;

  #[inline(always)]
  fn mul(self, rhs: Matrix4x4) -> Self::Output {
    Self::new(
      self.x * rhs[(0, 0)] + self.y * rhs[(1, 0)] + self.z * rhs[(2, 0)] + self.w * rhs[(3, 0)],
      self.x * rhs[(0, 1)] + self.y * rhs[(1, 1)] + self.z * rhs[(2, 1)] + self.w * rhs[(3, 1)],
      self.x * rhs[(0, 2)] + self.y * rhs[(1, 2)] + self.z * rhs[(2, 2)] + self.w * rhs[(3, 2)],
      self.x * rhs[(0, 3)] + self.y * rhs[(1, 3)] + self.z * rhs[(2, 3)] + self.w * rhs[(3, 3)],
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vector4_should_transform_by_identity_matrix() {
    let transform = Matrix4x4::IDENTITY;
    let position = vec4(1., 2., 3., 1.);
    let result = position * transform;

    assert_eq!(result, position);
  }

  #[test]
  fn vector4_should_transform_by_translation_matrix() {
    let transform = Matrix4x4::translate(1., 2., 3.);
    let result = vec4(1., 1., 1., 1.) * transform;

    assert_eq!(result, vec4(2., 3., 4., 1.));
  }
}
