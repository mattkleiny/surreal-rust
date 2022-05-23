use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut};

use crate::maths::Numeric;

/// A standard 2x2 matrix.
pub type Matrix2x2<T> = Matrix<T, 2, 4>;

/// A standard 3x3 matrix.
pub type Matrix3x3<T> = Matrix<T, 3, 9>;

/// A standard 4x4 matrix.
pub type Matrix4x4<T> = Matrix<T, 4, 16>;

/// Specialization for 2x2 matrices.
impl<T> Matrix2x2<T> where T: Numeric {
  pub const IDENTITY: Self = Self::identity();

  /// Constructs a new 2x2 identity matrix (1 along the left to right diagonal).
  pub const fn identity() -> Self {
    Self::create(&[
      T::ONE, T::ZERO,
      T::ZERO, T::ONE,
    ])
  }
}

impl<T> Default for Matrix2x2<T> where T: Numeric {
  fn default() -> Self {
    Self::IDENTITY
  }
}

/// Specialization for 3x3 matrices.
impl<T> Matrix3x3<T> where T: Numeric {
  pub const IDENTITY: Self = Self::identity();

  /// Constructs a new 3x3 identity matrix (1 along the left to right diagonal).
  pub const fn identity() -> Self {
    Self::create(&[
      T::ONE, T::ZERO, T::ZERO,
      T::ZERO, T::ONE, T::ZERO,
      T::ZERO, T::ZERO, T::ONE,
    ])
  }
}

impl<T> Default for Matrix3x3<T> where T: Numeric {
  fn default() -> Self {
    Self::IDENTITY
  }
}

/// Specialization for 4x4 matrices.
impl<T> Matrix4x4<T> where T: Numeric {
  pub const IDENTITY: Self = Self::identity();

  /// Constructs a new 4x4 identity matrix (1 along the left to right diagonal).
  pub const fn identity() -> Self {
    Self::create(&[
      T::ONE, T::ZERO, T::ZERO, T::ZERO,
      T::ZERO, T::ONE, T::ZERO, T::ZERO,
      T::ZERO, T::ZERO, T::ONE, T::ZERO,
      T::ZERO, T::ZERO, T::ZERO, T::ONE,
    ])
  }
}

impl<T> Default for Matrix4x4<T> where T: Numeric {
  fn default() -> Self {
    Self::IDENTITY
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

impl<T, const S: usize, const L: usize> Matrix<T, S, L> where T: Numeric {
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

impl<T, const S: usize, const L: usize> Debug for Matrix<T, S, L> where T: Display {
  /// Formats the matrix in a semi-readable manner.
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    for y in 0..S {
      write!(formatter, "[ ")?;

      for x in 0..S {
        write!(formatter, "{: >5.2} ", self.elements[x + y * S])?;
      }

      write!(formatter, "]\n")?
    }

    Ok(())
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