use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use crate::maths::{Lerp, Matrix4x4, Numeric, Range};

/// Shorthand to construct a [`Vector2`].
#[inline(always)]
pub const fn vec2<T: Numeric>(x: T, y: T) -> Vector2<T> {
  Vector2::new(x, y)
}

/// A standard purpose 2d vector
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Vector2<T> {
  pub x: T,
  pub y: T,
}

impl<T: Numeric> Vector2<T> {
  pub const ZERO: Self = Self::new(T::ZERO, T::ZERO);
  pub const UNIT_X: Self = Self::new(T::ONE, T::ZERO);
  pub const UNIT_Y: Self = Self::new(T::ZERO, T::ONE);
  pub const ONE: Self = Self::new(T::ONE, T::ONE);

  /// Creates a new vector from the given components.
  #[inline(always)]
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y }
  }

  /// Clamps the (x, y) components of the vector to the given range.
  #[inline(always)]
  pub fn clamp(&self, range: Range<T>) -> Self {
    Self::new(range.clamp(self.x), range.clamp(self.y))
  }
}

impl Vector2<f32> {
  /// Calculates the length of the vector.
  #[inline(always)]
  pub fn length(&self) -> f32 {
    self.length_squared().sqrt() as f32
  }

  /// Calculates the squared length of the vector.
  #[inline(always)]
  pub fn length_squared(&self) -> f32 {
    self.x * self.x + self.y * self.y
  }
}

impl Vector2<isize> {
  /// Calculates the length of the vector.
  #[inline(always)]
  pub fn length(&self) -> isize {
    (self.length_squared() as f32).sqrt() as isize
  }

  /// Calculates the squared length of the vector.
  #[inline(always)]
  pub fn length_squared(&self) -> isize {
    self.x * self.x + self.y * self.y
  }
}

impl<T: Numeric> From<[T; 2]> for Vector2<T> {
  fn from([x, y]: [T; 2]) -> Self {
    Vector2::new(x, y)
  }
}

impl<T: Numeric> Index<usize> for Vector2<T> {
  type Output = T;

  #[inline(always)]
  fn index(&self, index: usize) -> &Self::Output {
    match index {
      0 => &self.x,
      1 => &self.y,
      _ => panic!("Index out of range!"),
    }
  }
}

impl<T: Numeric> IndexMut<usize> for Vector2<T> {
  #[inline(always)]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    match index {
      0 => &mut self.x,
      1 => &mut self.y,
      _ => panic!("Index out of range!"),
    }
  }
}

impl<T: Numeric> Add for Vector2<T> {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y)
  }
}

impl<T: Numeric> AddAssign for Vector2<T> {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}

impl<T: Numeric> Sub for Vector2<T> {
  type Output = Self;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y)
  }
}

impl<T: Numeric> SubAssign for Vector2<T> {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    self.x -= rhs.x;
    self.y -= rhs.y;
  }
}

impl<T: Numeric> Mul for Vector2<T> {
  type Output = Self;

  #[inline(always)]
  fn mul(self, rhs: Self) -> Self::Output {
    Self::new(self.x * rhs.x, self.y * rhs.y)
  }
}

impl<T: Numeric> MulAssign for Vector2<T> {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: Self) {
    self.x *= rhs.x;
    self.y *= rhs.y;
  }
}

impl<T: Numeric> Div for Vector2<T> {
  type Output = Self;

  #[inline(always)]
  fn div(self, rhs: Self) -> Self::Output {
    Self::new(self.x / rhs.x, self.y / rhs.y)
  }
}

impl<T: Numeric> DivAssign for Vector2<T> {
  #[inline(always)]
  fn div_assign(&mut self, rhs: Self) {
    self.x /= rhs.x;
    self.y /= rhs.y;
  }
}

impl<T: Numeric> Mul<T> for Vector2<T> {
  type Output = Self;

  #[inline(always)]
  fn mul(self, rhs: T) -> Self::Output {
    Self::new(self.x * rhs, self.y * rhs)
  }
}

impl<T: Numeric> MulAssign<T> for Vector2<T> {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: T) {
    self.x *= rhs;
    self.y *= rhs;
  }
}

impl<T: Numeric> Div<T> for Vector2<T> {
  type Output = Self;

  #[inline(always)]
  fn div(self, rhs: T) -> Self::Output {
    Self::new(self.x / rhs, self.y / rhs)
  }
}

impl<T: Numeric> DivAssign<T> for Vector2<T> {
  #[inline(always)]
  fn div_assign(&mut self, rhs: T) {
    self.x /= rhs;
    self.y /= rhs;
  }
}

impl<T: Numeric> From<(T, T)> for Vector2<T> {
  #[inline(always)]
  fn from((x, y): (T, T)) -> Self {
    Self::new(x, y)
  }
}

impl<T: Numeric> Lerp for Vector2<T> {
  #[inline(always)]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self::new(T::lerp(a.x, b.x, t), T::lerp(a.y, b.y, t))
  }
}

impl Mul<Matrix4x4> for Vector2<f32> {
  type Output = Self;

  #[inline(always)]
  fn mul(self, rhs: Matrix4x4) -> Self::Output {
    Self::new(
      self.x * rhs[(0, 0)] + self.y * rhs[(1, 0)] + rhs[(3, 0)],
      self.x * rhs[(0, 1)] + self.y * rhs[(1, 1)] + rhs[(3, 1)],
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vector2_should_transform_by_identity_matrix() {
    let transform = Matrix4x4::IDENTITY;
    let position = vec2(1., 2.);

    let result = position * transform;

    assert_eq!(result, position);
  }

  #[test]
  fn vector2_should_transform_by_translation_matrix() {
    let transform = Matrix4x4::translate(1., 2., 0.);
    let result = vec2(1., 1.) * transform;

    assert_eq!(result, vec2(2., 3.));
  }
}
