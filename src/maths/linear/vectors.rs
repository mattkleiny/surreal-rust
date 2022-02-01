use std::ops::{Add, Sub};

/// A standard purpose 2d vector
#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
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

impl<T> Add for Vector2<T> where T: Add<Output = T> {
  type Output = Self;

  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y)
  }
}

impl<T> Sub for Vector2<T> where T: Sub<Output = T> {
  type Output = Self;

  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y)
  }
}

/// A standard purpose 3d vector
#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
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

impl<T> Add for Vector3<T> where T: Add<Output = T> {
  type Output = Self;

  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl<T> Sub for Vector3<T> where T: Sub<Output = T> {
  type Output = Self;

  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

/// A standard purpose 4d vector
#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
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

impl<T> Add for Vector4<T> where T: Add<Output = T> {
  type Output = Self;

  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
  }
}

impl<T> Sub for Vector4<T> where T: Sub<Output = T> {
  type Output = Self;

  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
  }
}
