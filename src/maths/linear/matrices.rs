use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut, Mul};

use super::*;

/// A standard 2x2 matrix.
pub type Matrix2x2<T> = Matrix<T, 2, 4>;

/// A standard 3x3 matrix.
pub type Matrix3x3<T> = Matrix<T, 3, 9>;

/// A standard 4x4 matrix.
pub type Matrix4x4<T> = Matrix<T, 4, 16>;

/// Specialization for 2x2 matrices.
impl<T> Matrix2x2<T>
where T: Numeric
{
  pub const IDENTITY: Self = Self::identity();

  /// Constructs a new 2x2 identity matrix (1 along the left to right diagonal).
  #[rustfmt::skip]
  pub const fn identity() -> Self {
    Self::create(&[
      T::ONE, T::ZERO,
      T::ZERO, T::ONE,
    ])
  }
}

impl<T> Default for Matrix2x2<T>
where T: Numeric
{
  fn default() -> Self {
    Self::IDENTITY
  }
}

/// Specialization for 3x3 matrices.
impl<T> Matrix3x3<T>
where T: Numeric
{
  pub const IDENTITY: Self = Self::identity();

  /// Constructs a new 3x3 identity matrix (1 along the left to right diagonal).
  #[rustfmt::skip]
  pub const fn identity() -> Self {
    Self::create(&[
      T::ONE, T::ZERO, T::ZERO,
      T::ZERO, T::ONE, T::ZERO,
      T::ZERO, T::ZERO, T::ONE,
    ])
  }
}

impl<T> Default for Matrix3x3<T>
where T: Numeric
{
  fn default() -> Self {
    Self::IDENTITY
  }
}

/// Specialization for 4x4 matrices.
impl<T> Matrix4x4<T>
where T: Numeric
{
  pub const IDENTITY: Self = Self::identity();

  /// Constructs a new 4x4 identity matrix (1 along the left to right diagonal).
  #[rustfmt::skip]
  pub const fn identity() -> Self {
    Self::create(&[
      T::ONE, T::ZERO, T::ZERO, T::ZERO,
      T::ZERO, T::ONE, T::ZERO, T::ZERO,
      T::ZERO, T::ZERO, T::ONE, T::ZERO,
      T::ZERO, T::ZERO, T::ZERO, T::ONE,
    ])
  }
}

impl<T> Default for Matrix4x4<T>
where T: Numeric
{
  fn default() -> Self {
    Self::IDENTITY
  }
}

/// Specialization for floating point 4x4 matrices.
impl Matrix4x4<f32> {
  /// Creates a new translation matrix.
  #[rustfmt::skip]
  pub fn translate(x: f32, y: f32, z: f32) -> Self {
    Self::create(&[
      1.0, 0.0, 0.0, x,
      0.0, 1.0, 0.0, y,
      0.0, 0.0, 1.0, z,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new scale matrix.
  #[rustfmt::skip]
  pub fn scale(x: f32, y: f32, z: f32) -> Self {
    Self::create(&[
      x, 0.0, 0.0, 0.0,
      0.0, y, 0.0, 0.0,
      0.0, 0.0, z, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new rotation matrix about the X axis.
  #[rustfmt::skip]
  pub fn rotate_x(r: f32) -> Self {
    Self::create(&[
      1.0, 0.0, 0.0, 0.0,
      0.0, r.cos(), -r.sin(), 0.0,
      0.0, r.sin(), r.cos(), 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new rotation matrix about the Y axis.
  #[rustfmt::skip]
  pub fn rotate_y(r: f32) -> Self {
    Self::create(&[
      r.cos(), 0.0, r.sin(), 0.0,
      0.0, 1.0, 0.0, 0.0,
      -r.sin(), 0.0, r.cos(), 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new rotation matrix about the Z axis.
  #[rustfmt::skip]
  pub fn rotate_z(r: f32) -> Self {
    Self::create(&[
      r.cos(), -r.sin(), 0.0, 0.0,
      r.sin(), r.cos(), 0.0, 0.0,
      0.0, 0.0, 1.0, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new shearing matrix with the given proportions.
  #[rustfmt::skip]
  pub fn shear(x1: f32, x2: f32, y1: f32, y2: f32, z1: f32, z2: f32) -> Self {
    Self::create(&[
      1.0, x1, x2, 0.0,
      y1, 1.0, y2, 0.0,
      z1, z2, 1.0, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new view transformation that looks at the given point.
  #[rustfmt::skip]
  pub fn look_at(from: Vector3<f32>, to: Vector3<f32>, up: Vector3<f32>) -> Self {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());
    let true_up = left.cross(forward);

    let orientation = Self::create(&[
      left.x, left.y, left.z, 0.0,
      true_up.x, true_up.y, true_up.z, 0.0,
      -forward.x, -forward.y, -forward.z, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ]);

    orientation * Self::translate(-from.x, -from.y, -from.z)
  }

  /// Creates a new orthographic projection matrix.
  #[rustfmt::skip]
  pub fn orthographic(width: f32, height: f32, near: f32, far: f32) -> Self {
    Self::orthographic_off_center(-width / 2., width / 2., -height / 2., height / 2., near, far)
  }

  /// Creates a new orthographic projection matrix.
  #[rustfmt::skip]
  pub fn orthographic_off_center(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Self {
    let dx = right - left;
    let dy = top - bottom;
    let dz = far - near;

    Self::create(&[
      2.0 / dx, 0.0, 0.0, -(right + left) / dx,
      0.0, 2.0 / dy, 0.0, -(top + bottom) / dy,
      0.0, 0.0, -2.0 / dz, -(far + near) / dz,
      0.0, 0.0, 0.0, 1.0,
    ])
  }
}

/// A rectangular matrix of N by N elements, with the given row stride.
///
/// T = Type of the matrix; must be a numeric type.
/// S = Stride of the matrix; how many columns between each row.
/// L = Length of the matrix; total number of elements.
#[derive(Copy, Clone)]
pub struct Matrix<T, const S: usize, const L: usize> {
  elements: [T; L],
}

impl<T, const S: usize, const L: usize> Matrix<T, S, L>
where T: Numeric
{
  /// Constructs a new empty matrix.
  pub const fn new() -> Self {
    Self { elements: [T::ZERO; L] }
  }

  /// Constructs a matrix from the given elements.
  pub const fn create(elements: &[T; L]) -> Self {
    Self { elements: *elements }
  }

  /// Transposes the matrix.
  pub fn transpose(&self) -> Self {
    let mut result = Self::new();

    for i in 0..S {
      for j in 0..S {
        result[(i, j)] = self[(j, i)];
      }
    }

    result
  }

  /// Converts the matrix to a slice.
  pub fn as_slice(&self) -> &[T] {
    self.elements.as_slice()
  }

  /// Converts the matrix to a mutable slice.
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    self.elements.as_mut_slice()
  }
}

impl<T, const S: usize, const L: usize> Debug for Matrix<T, S, L>
where T: Display
{
  /// Formats the matrix in a semi-readable manner.
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    for y in 0..S {
      write!(formatter, "[ ")?;

      for x in 0..S {
        write!(formatter, "{: >5.2} ", self.elements[x + y * S])?;
      }

      writeln!(formatter, "]")?
    }

    Ok(())
  }
}

impl<const S: usize, const L: usize> Mul for Matrix<f32, S, L> {
  type Output = Self;

  /// Multiplies two matrices together.
  fn mul(self, rhs: Self) -> Self::Output {
    let mut result = Self::new();

    for row in 0..S {
      for column in 0..S {
        let mut sum = 0.;

        for i in 0..S {
          sum += self[(row, i)] * rhs[(i, column)];
        }

        result[(row, column)] = sum;
      }
    }

    result
  }
}

impl<T, const S: usize, const L: usize> Index<(usize, usize)> for Matrix<T, S, L> {
  type Output = T;

  fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
    &self.elements[column + row * S]
  }
}

impl<T, const S: usize, const L: usize> IndexMut<(usize, usize)> for Matrix<T, S, L> {
  fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
    &mut self.elements[column + row * S]
  }
}
