use std::ops::{Add, Div, Mul, Sub};

use crate::maths::{Lerp, Numeric, Range};

/// Shorthand to construct a [`Vector4`].
pub const fn vec4<T>(x: T, y: T, z: T, w: T) -> Vector4<T> where T: Numeric {
  Vector4::new(x, y, z, w)
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

  /// Clamps the (x, y, z, w) components of the vector to the given range.
  pub fn clamp(&self, range: Range<T>) -> Self {
    Self::new(
      range.clamp(self.x),
      range.clamp(self.y),
      range.clamp(self.z),
      range.clamp(self.w),
    )
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
