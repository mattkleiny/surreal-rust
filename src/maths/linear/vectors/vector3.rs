use std::ops::{Add, Div, Mul, Sub};

use crate::maths::{Lerp, Numeric, Range};

/// Shorthand to construct a [`Vector3`].
#[inline(always)]
pub const fn vec3<T: Numeric>(x: T, y: T, z: T) -> Vector3<T> {
  Vector3::new(x, y, z)
}

/// A standard purpose 3d vector
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
  pub fn clamp(&self, range: Range<T>) -> Self {
    Self::new(range.clamp(self.x), range.clamp(self.y), range.clamp(self.z))
  }
}

impl Vector3<f32> {
  /// Computes the magnitude of this vector; the length essentially.
  pub fn magnitude(&self) -> f32 {
    let x2 = self.x * self.x;
    let y2 = self.y * self.y;
    let z2 = self.z * self.z;

    (x2 + y2 + z2).sqrt()
  }

  /// Normalizes the vector to the range (-1, 1) for all components.
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
  pub fn dot(&self, other: Self) -> f32 {
    let x = self.x * other.x;
    let y = self.y * other.y;
    let z = self.z * other.z;

    x + y + z
  }

  /// Computes the cross product of this vector and another.
  ///
  /// The cross product is a vector perpendicular to both vectors.
  pub fn cross(&self, other: Self) -> Self {
    let x = self.y * other.z - self.z * other.y;
    let y = self.z * other.x - self.x * other.z;
    let z = self.x * other.y - self.y * other.x;

    vec3(x, y, z)
  }

  /// Reflects a vector about the given normal.
  pub fn reflect(self, normal: Self) -> Self {
    self - normal * 2. * self.dot(normal)
  }
}

impl<T: Numeric> Add for Vector3<T> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl<T: Numeric> Sub for Vector3<T> {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl<T: Numeric> Mul for Vector3<T> {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
  }
}

impl<T: Numeric> Div for Vector3<T> {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
  }
}

impl<T: Numeric> Mul<T> for Vector3<T> {
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
  }
}

impl<T: Numeric> Div<T> for Vector3<T> {
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
  }
}

impl<T: Numeric> From<(T, T, T)> for Vector3<T> {
  fn from((x, y, z): (T, T, T)) -> Self {
    Self::new(x, y, z)
  }
}

impl<T: Numeric + Lerp> Lerp for Vector3<T> {
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self::new(T::lerp(a.x, b.x, t), T::lerp(a.y, b.y, t), T::lerp(a.z, b.z, t))
  }
}
