use std::ops::{Add, Div, Mul, Sub};

use super::*;

/// Shorthand to construct a [`Vector2`].
pub const fn vec2<T>(x: T, y: T) -> Vector2<T> where T: Numeric {
  Vector2::new(x, y)
}

/// Shorthand to construct a [`Vector3`].
pub const fn vec3<T>(x: T, y: T, z: T) -> Vector3<T> where T: Numeric {
  Vector3::new(x, y, z)
}

/// Shorthand to construct a [`Vector4`].
pub const fn vec4<T>(x: T, y: T, z: T, w: T) -> Vector4<T> where T: Numeric {
  Vector4::new(x, y, z, w)
}

/// A standard purpose 2d vector
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct Vector2<T> {
  pub x: T,
  pub y: T,
}

impl<T> Vector2<T> where T: Numeric {
  pub const ZERO: Self = Self::new(T::ZERO, T::ZERO);
  pub const UNIT_X: Self = Self::new(T::ONE, T::ZERO);
  pub const UNIT_Y: Self = Self::new(T::ZERO, T::ONE);
  pub const ONE: Self = Self::new(T::ONE, T::ONE);

  /// Creates a new vector from the given components.
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl Vector2<f32> {
  /// Calculates the length of the vector.
  pub fn length(&self) -> f32 {
    self.length_squared().sqrt() as f32
  }

  /// Calculates the squared length of the vector.
  pub fn length_squared(&self) -> f32 {
    self.x * self.x + self.y * self.y
  }
}

impl Vector2<isize> {
  /// Calculates the length of the vector.
  pub fn length(&self) -> isize {
    (self.length_squared() as f32).sqrt() as isize
  }

  /// Calculates the squared length of the vector.
  pub fn length_squared(&self) -> isize {
    self.x * self.x + self.y * self.y
  }
}

impl<T> Add for Vector2<T> where T: Numeric {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y)
  }
}

impl<T> Sub for Vector2<T> where T: Numeric {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y)
  }
}

impl<T> Mul for Vector2<T> where T: Numeric {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Self::new(self.x * rhs.x, self.y * rhs.y)
  }
}

impl<T> Div for Vector2<T> where T: Numeric {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    Self::new(self.x / rhs.x, self.y / rhs.y)
  }
}

impl<T> Mul<T> for Vector2<T> where T: Numeric {
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::new(self.x * rhs, self.y * rhs)
  }
}

impl<T> Div<T> for Vector2<T> where T: Numeric {
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::new(self.x / rhs, self.y / rhs)
  }
}

impl<T> From<(T, T)> for Vector2<T> where T: Numeric {
  fn from((x, y): (T, T)) -> Self {
    Self::new(x, y)
  }
}

impl<T> Lerp for Vector2<T> where T: Numeric + Lerp {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self::new(
      T::lerp(a.x, b.x, t),
      T::lerp(a.y, b.y, t),
    )
  }
}

/// A standard purpose 3d vector
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct Vector3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T> Vector3<T> where T: Numeric {
  pub const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO);
  pub const UNIT_X: Self = Self::new(T::ONE, T::ZERO, T::ZERO);
  pub const UNIT_Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO);
  pub const UNIT_Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE);
  pub const ONE: Self = Self::new(T::ONE, T::ONE, T::ONE);

  pub const fn new(x: T, y: T, z: T) -> Self {
    Self { x, y, z }
  }
}

impl<T> Add for Vector3<T> where T: Numeric {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl<T> Sub for Vector3<T> where T: Numeric {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl<T> Mul for Vector3<T> where T: Numeric {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
  }
}

impl<T> Div for Vector3<T> where T: Numeric {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
  }
}

impl<T> Mul<T> for Vector3<T> where T: Numeric {
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
  }
}

impl<T> Div<T> for Vector3<T> where T: Numeric {
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
  }
}

impl<T> From<(T, T, T)> for Vector3<T> where T: Numeric {
  fn from((x, y, z): (T, T, T)) -> Self {
    Self::new(x, y, z)
  }
}

impl<T> Lerp for Vector3<T> where T: Numeric + Lerp {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self::new(
      T::lerp(a.x, b.x, t),
      T::lerp(a.y, b.y, t),
      T::lerp(a.z, b.z, t),
    )
  }
}

/// A standard purpose 4d vector
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct Vector4<T> {
  pub x: T,
  pub y: T,
  pub z: T,
  pub w: T,
}

impl<T> Vector4<T> where T: Numeric {
  pub const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
  pub const UNIT_X: Self = Self::new(T::ONE, T::ZERO, T::ZERO, T::ZERO);
  pub const UNIT_Y: Self = Self::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);
  pub const UNIT_Z: Self = Self::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);
  pub const UNIT_W: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
  pub const ONE: Self = Self::new(T::ONE, T::ONE, T::ONE, T::ONE);

  pub const fn new(x: T, y: T, z: T, w: T) -> Self {
    Self { x, y, z, w }
  }
}

impl<T> Add for Vector4<T> where T: Numeric {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
  }
}

impl<T> Sub for Vector4<T> where T: Numeric {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
  }
}

impl<T> Mul for Vector4<T> where T: Numeric {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.w)
  }
}

impl<T> Div for Vector4<T> where T: Numeric {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z, self.w / rhs.w)
  }
}


impl<T> Mul<T> for Vector4<T> where T: Numeric {
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
  }
}

impl<T> Div<T> for Vector4<T> where T: Numeric {
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
  }
}

impl<T> From<(T, T, T, T)> for Vector4<T> where T: Numeric {
  fn from((x, y, z, w): (T, T, T, T)) -> Self {
    Self::new(x, y, z, w)
  }
}

impl<T> Lerp for Vector4<T> where T: Numeric + Lerp {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self::new(
      T::lerp(a.x, b.x, t),
      T::lerp(a.y, b.y, t),
      T::lerp(a.z, b.z, t),
      T::lerp(a.w, b.w, t),
    )
  }
}
