use std::ops::{Add, Div, Mul, Sub};

use crate::maths::{Lerp, Numeric};

/// Shorthand to construct a `Vector2`
pub const fn vec2<T>(x: T, y: T) -> Vector2<T> {
  Vector2::new(x, y)
}

/// Shorthand to construct a `Vector3`
pub const fn vec3<T>(x: T, y: T, z: T) -> Vector3<T> {
  Vector3::new(x, y, z)
}

/// Shorthand to construct a `Vector4`
pub const fn vec4<T>(x: T, y: T, z: T, w: T) -> Vector4<T> {
  Vector4::new(x, y, z, w)
}

/// A standard purpose 2d vector
#[derive(Hash, Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct Vector2<T> {
  pub x: T,
  pub y: T,
}

impl<T> Vector2<T> {
  #[inline(always)]
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl<T> Add for Vector2<T> where T: Add<Output=T> {
  type Output = Self;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y)
  }
}

impl<T> Sub for Vector2<T> where T: Sub<Output=T> {
  type Output = Self;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y)
  }
}

impl<T> Mul<T> for Vector2<T> where T: Numeric {
  type Output = Self;

  #[inline(always)]
  fn mul(self, rhs: T) -> Self::Output {
    Self::new(self.x * rhs, self.y * rhs)
  }
}

impl<T> Div<T> for Vector2<T> where T: Numeric {
  type Output = Self;

  #[inline(always)]
  fn div(self, rhs: T) -> Self::Output {
    Self::new(self.x / rhs, self.y / rhs)
  }
}

impl<T> From<(T, T)> for Vector2<T> {
  fn from((x, y): (T, T)) -> Self {
    Self::new(x, y)
  }
}

impl<T> Lerp for Vector2<T> where T: Lerp {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self::new(
      T::lerp(a.x, b.x, t),
      T::lerp(a.y, b.y, t),
    )
  }
}

/// A standard purpose 3d vector
#[derive(Hash, Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct Vector3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T> Vector3<T> {
  #[inline(always)]
  pub const fn new(x: T, y: T, z: T) -> Self {
    Self { x, y, z }
  }
}

impl<T> Add for Vector3<T> where T: Add<Output=T> {
  type Output = Self;

  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl<T> Sub for Vector3<T> where T: Sub<Output=T> {
  type Output = Self;

  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl<T> From<(T, T, T)> for Vector3<T> {
  fn from((x, y, z): (T, T, T)) -> Self {
    Self::new(x, y, z)
  }
}

impl<T> Lerp for Vector3<T> where T: Lerp {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self::new(
      T::lerp(a.x, b.x, t),
      T::lerp(a.y, b.y, t),
      T::lerp(a.z, b.z, t),
    )
  }
}

/// A standard purpose 4d vector
#[derive(Hash, Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct Vector4<T> {
  pub x: T,
  pub y: T,
  pub z: T,
  pub w: T,
}

impl<T> Vector4<T> {
  #[inline(always)]
  pub const fn new(x: T, y: T, z: T, w: T) -> Self {
    Self { x, y, z, w }
  }
}

impl<T> Add for Vector4<T> where T: Add<Output=T> {
  type Output = Self;

  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
  }
}

impl<T> Sub for Vector4<T> where T: Sub<Output=T> {
  type Output = Self;

  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
  }
}

impl<T> From<(T, T, T, T)> for Vector4<T> {
  fn from((x, y, z, w): (T, T, T, T)) -> Self {
    Self::new(x, y, z, w)
  }
}

impl<T> Lerp for Vector4<T> where T: Lerp {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self::new(
      T::lerp(a.x, b.x, t),
      T::lerp(a.y, b.y, t),
      T::lerp(a.z, b.z, t),
      T::lerp(a.w, b.w, t),
    )
  }
}
