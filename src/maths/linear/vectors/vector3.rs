use std::ops::{Add, Div, Mul, Sub};

use crate::maths::{Lerp, Numeric};

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
