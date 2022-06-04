use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::maths::{Lerp, Numeric, Range};

/// Shorthand to construct a [`Vector2`].
pub const fn vec2<T>(x: T, y: T) -> Vector2<T>
where T: Numeric {
  Vector2::new(x, y)
}

/// A standard purpose 2d vector
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct Vector2<T> {
  pub x: T,
  pub y: T,
}

impl<T> Vector2<T>
where T: Numeric
{
  pub const ZERO: Self = Self::new(T::ZERO, T::ZERO);
  pub const UNIT_X: Self = Self::new(T::ONE, T::ZERO);
  pub const UNIT_Y: Self = Self::new(T::ZERO, T::ONE);
  pub const ONE: Self = Self::new(T::ONE, T::ONE);

  /// Creates a new vector from the given components.
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y }
  }

  /// Clamps the (x, y) components of the vector to the given range.
  pub fn clamp(&self, range: Range<T>) -> Self {
    Self::new(range.clamp(self.x), range.clamp(self.y))
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

impl<T> Add for Vector2<T>
where T: Numeric
{
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y)
  }
}

impl<T> AddAssign for Vector2<T>
where T: Numeric
{
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}

impl<T> Sub for Vector2<T>
where T: Numeric
{
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y)
  }
}

impl<T> SubAssign for Vector2<T>
where T: Numeric
{
  fn sub_assign(&mut self, rhs: Self) {
    self.x -= rhs.x;
    self.y -= rhs.y;
  }
}

impl<T> Mul for Vector2<T>
where T: Numeric
{
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Self::new(self.x * rhs.x, self.y * rhs.y)
  }
}

impl<T> MulAssign for Vector2<T>
where T: Numeric
{
  fn mul_assign(&mut self, rhs: Self) {
    self.x *= rhs.x;
    self.y *= rhs.y;
  }
}

impl<T> Div for Vector2<T>
where T: Numeric
{
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    Self::new(self.x / rhs.x, self.y / rhs.y)
  }
}

impl<T> DivAssign for Vector2<T>
where T: Numeric
{
  fn div_assign(&mut self, rhs: Self) {
    self.x /= rhs.x;
    self.y /= rhs.y;
  }
}

impl<T> Mul<T> for Vector2<T>
where T: Numeric
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::new(self.x * rhs, self.y * rhs)
  }
}

impl<T> MulAssign<T> for Vector2<T>
where T: Numeric
{
  fn mul_assign(&mut self, rhs: T) {
    self.x *= rhs;
    self.y *= rhs;
  }
}

impl<T> Div<T> for Vector2<T>
where T: Numeric
{
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::new(self.x / rhs, self.y / rhs)
  }
}

impl<T> DivAssign<T> for Vector2<T>
where T: Numeric
{
  fn div_assign(&mut self, rhs: T) {
    self.x /= rhs;
    self.y /= rhs;
  }
}

impl<T> From<(T, T)> for Vector2<T>
where T: Numeric
{
  fn from((x, y): (T, T)) -> Self {
    Self::new(x, y)
  }
}

impl<T> Lerp for Vector2<T>
where T: Numeric + Lerp
{
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self::new(T::lerp(a.x, b.x, t), T::lerp(a.y, b.y, t))
  }
}
