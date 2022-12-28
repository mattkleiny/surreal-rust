use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut, Mul};

use super::*;

/// A standard 2x2 matrix.
pub type Matrix2x2<T> = Matrix<T, 2, 4>;

/// A standard 3x3 matrix.
pub type Matrix3x3<T> = Matrix<T, 3, 9>;

/// A standard 4x4 matrix.
pub type Matrix4x4<T> = Matrix<T, 4, 16>;

impl<T: Numeric> Default for Matrix2x2<T> {
  fn default() -> Self {
    Self::IDENTITY
  }
}

/// Specialization for 2x2 matrices.
impl<T: Numeric> Matrix2x2<T> {
  /// A 2x2 identity matrix.
  #[rustfmt::skip]
  pub const IDENTITY: Self = Self::create(&[
    T::ONE, T::ZERO,
    T::ZERO, T::ONE,
  ]);

  /// Computes the determinant of the matrix.
  ///
  /// A determinant 'determines' whether a system has a solution.
  pub fn determinant(&self) -> T {
    let [a, b, c, d] = self.elements;

    a * d - b * c
  }
}

impl<T: Numeric> Default for Matrix3x3<T> {
  fn default() -> Self {
    Self::IDENTITY
  }
}

/// Specialization for 3x3 matrices.
impl<T: Numeric> Matrix3x3<T> {
  /// A 3x3 identity matrix.
  #[rustfmt::skip]
  pub const IDENTITY: Self = Self::create(&[
    T::ONE, T::ZERO, T::ZERO,
    T::ZERO, T::ONE, T::ZERO,
    T::ZERO, T::ZERO, T::ONE,
  ]);
}

impl<T: Numeric> Default for Matrix4x4<T> {
  fn default() -> Self {
    Self::IDENTITY
  }
}

/// Specialization for 4x4 matrices.
impl<T: Numeric> Matrix4x4<T> {
  /// A 4x4 identity matrix.
  #[rustfmt::skip]
  pub const IDENTITY: Self = Self::create(&[
    T::ONE, T::ZERO, T::ZERO, T::ZERO,
    T::ZERO, T::ONE, T::ZERO, T::ZERO,
    T::ZERO, T::ZERO, T::ONE, T::ZERO,
    T::ZERO, T::ZERO, T::ZERO, T::ONE,
  ]);
}

// TODO: abstract over reals?
impl Matrix4x4<f32> {
  /// Creates a new translation matrix.
  #[rustfmt::skip]
  pub fn from_translation(x: f32, y: f32, z: f32) -> Self {
    Self::create(&[
      1., 0., 0., x,
      0., 1., 0., y,
      0., 0., 1., z,
      0., 0., 0., 1.,
    ])
  }

  /// Creates a new scale matrix.
  #[rustfmt::skip]
  pub fn from_scale(x: f32, y: f32, z: f32) -> Self {
    Self::create(&[
      x, 0., 0., 0.,
      0., y, 0., 0.,
      0., 0., z, 0.,
      0., 0., 0., 1.,
    ])
  }

  /// Creates a new shearing matrix with the given proportions.
  #[rustfmt::skip]
  pub fn from_shear(x1: f32, x2: f32, y1: f32, y2: f32, z1: f32, z2: f32) -> Self {
    Self::create(&[
      1., x1, x2, 0.,
      y1, 1., y2, 0.,
      z1, z2, 1., 0.,
      0., 0., 0., 1.,
    ])
  }

  /// Creates a new rotation matrix about the X axis.
  #[rustfmt::skip]
  pub fn from_rotate_x(r: f32) -> Self {
    Self::create(&[
      1., 0., 0., 0.,
      0., r.cos(), -r.sin(), 0.,
      0., r.sin(), r.cos(), 0.,
      0., 0., 0., 1.,
    ])
  }

  /// Creates a new rotation matrix about the Y axis.
  #[rustfmt::skip]
  pub fn from_rotate_y(r: f32) -> Self {
    Self::create(&[
      r.cos(), 0., r.sin(), 0.,
      0., 1., 0., 0.,
      -r.sin(), 0., r.cos(), 0.,
      0., 0., 0., 1.,
    ])
  }

  /// Creates a new rotation matrix about the Z axis.
  #[rustfmt::skip]
  pub fn from_rotate_z(r: f32) -> Self {
    Self::create(&[
      r.cos(), -r.sin(), 0., 0.,
      r.sin(), r.cos(), 0., 0.,
      0., 0., 1., 0.,
      0., 0., 0., 1.,
    ])
  }

  /// Creates a new view transformation that looks at the given point.
  #[rustfmt::skip]
  pub fn from_look_at(from: Vector3<f32>, to: Vector3<f32>, up: Vector3<f32>) -> Self {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());
    let true_up = left.cross(forward);

    let orientation = Self::create(&[
      left.x, left.y, left.z, 0.,
      true_up.x, true_up.y, true_up.z, 0.,
      -forward.x, -forward.y, -forward.z, 0.,
      0., 0., 0., 1.,
    ]);

    orientation * Self::from_translation(-from.x, -from.y, -from.z)
  }

  /// Creates a new orthographic projection matrix.
  #[rustfmt::skip]
  pub fn from_orthographic(width: f32, height: f32, near: f32, far: f32) -> Self {
    Self::from_orthographic_off_center(
      -width / 2.,
      width / 2.,
      -height / 2.,
      height / 2.,
      near,
      far
    )
  }

  /// Creates a new orthographic projection matrix.
  #[rustfmt::skip]
  pub fn from_orthographic_off_center(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Self {
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
/// This matrix is row-major, with the (x, y) coordinates of a row access indexing the
/// row first, column second.
///
/// T = Type of the matrix; must be a numeric type.
/// S = Stride of the matrix; how many columns between each row.
/// L = Length of the matrix; total number of elements.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Matrix<T: Numeric, const S: usize, const L: usize> {
  elements: [T; L],
}

impl<T: Numeric, const S: usize, const L: usize> Matrix<T, S, L> {
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

    for y in 0..S {
      for x in 0..S {
        result[(x, y)] = self[(y, x)];
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

impl<T: Numeric + Display, const S: usize, const L: usize> Debug for Matrix<T, S, L> {
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

impl<T: Numeric, const S: usize, const L: usize> Mul for Matrix<T, S, L> {
  type Output = Self;

  /// Multiplies two matrices together.
  fn mul(self, rhs: Self) -> Self::Output {
    let mut result = Self::new();

    for y in 0..S {
      for x in 0..S {
        let mut sum = T::ZERO;

        for i in 0..S {
          sum += self[(i, y)] * rhs[(x, i)];
        }

        result[(x, y)] = sum;
      }
    }

    result
  }
}

impl<T: Numeric, const S: usize, const L: usize> Index<(usize, usize)> for Matrix<T, S, L> {
  type Output = T;

  /// Indexes into the matrix by row and column.
  #[inline(always)]
  fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
    &self.elements[x + y * S]
  }
}

impl<T: Numeric, const S: usize, const L: usize> IndexMut<(usize, usize)> for Matrix<T, S, L> {
  /// Mutably indexes into the matrix by row and column.
  #[inline(always)]
  fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
    &mut self.elements[x + y * S]
  }
}

impl<T: Numeric, const S: usize, const L: usize> PartialEq for Matrix<T, S, L> {
  fn eq(&self, other: &Self) -> bool {
    for y in 0..S {
      for x in 0..S {
        let a = &self[(x, y)];
        let b = &other[(x, y)];

        if !a.eq(b) {
          return false;
        }
      }
    }

    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn matrix2x2_should_construct_from_elements() {
    #[rustfmt::skip]
    let matrix = Matrix2x2::create(&[
      -3., 5.,
      1., -2.,
    ]);

    assert_eq!(matrix[(0, 0)], -3.);
    assert_eq!(matrix[(1, 0)], 5.);
    assert_eq!(matrix[(0, 1)], 1.);
    assert_eq!(matrix[(1, 1)], -2.);
  }

  #[test]
  fn matrix3x3_should_construct_from_elements() {
    #[rustfmt::skip]
    let matrix = Matrix3x3::create(&[
      -3., 5., 0.,
      1., -2., -7.,
      0., 1., 1.,
    ]);

    assert_eq!(matrix[(0, 0)], -3.);
    assert_eq!(matrix[(1, 1)], -2.);
    assert_eq!(matrix[(2, 2)], 1.);
  }

  #[test]
  fn matrix4x4_should_construct_from_elements() {
    #[rustfmt::skip]
    let matrix = Matrix4x4::create(&[
      1., 2., 3., 4.,
      5.5, 6.5, 7.5, 8.5,
      9., 10., 11., 12.,
      13.5, 14.5, 15.5, 16.5,
    ]);

    assert_eq!(matrix[(0, 0)], 1.);
    assert_eq!(matrix[(3, 0)], 4.);
    assert_eq!(matrix[(0, 1)], 5.5);
    assert_eq!(matrix[(2, 1)], 7.5);
    assert_eq!(matrix[(2, 2)], 11.);
    assert_eq!(matrix[(0, 3)], 13.5);
    assert_eq!(matrix[(2, 3)], 15.5);
  }

  #[test]
  fn matrix_equality_should_work() {
    #[rustfmt::skip]
    let a = Matrix3x3::create(&[
      1., 2., 3.,
      4., 5., 6.,
      7., 8., 9.,
    ]);

    #[rustfmt::skip]
    let b = Matrix3x3::create(&[
      1., 2., 3.,
      4., 5., 6.,
      7., 8., 9.,
    ]);

    assert_eq!(a, b);
  }

  #[test]
  fn matrix_inequality_should_work() {
    #[rustfmt::skip]
    let a = Matrix3x3::create(&[
      1., 2., 3.,
      4., 5., 6.,
      7., 8., 9.,
    ]);

    #[rustfmt::skip]
    let b = Matrix3x3::create(&[
      2., 3., 4.,
      5., 6., 7.,
      8., 9., 10.,
    ]);

    assert_ne!(a, b);
  }

  #[test]
  fn matrices_can_multiply_by_other_matrices() {
    #[rustfmt::skip]
    let a = Matrix4x4::create(&[
      1., 2., 3., 4.,
      5., 6., 7., 8.,
      9., 8., 7., 6.,
      5., 4., 3., 2.,
    ]);

    #[rustfmt::skip]
    let b = Matrix4x4::create(&[
      -2., 1., 2., 3.,
      3., 2., 1., -1.,
      4., 3., 6., 5.,
      1., 2., 7., 8.,
    ]);

    #[rustfmt::skip]
    assert_eq!(a * b, Matrix4x4::create(&[
      20., 22., 50., 48.,
      44., 54., 114., 108.,
      40., 58., 110., 102.,
      16., 26., 46., 42.,
    ]));
  }

  #[test]
  fn matrix_multiplication_by_identity_should_be_inert() {
    #[rustfmt::skip]
    let a = Matrix4x4::create(&[
      0., 1., 2., 4.,
      1., 2., 4., 8.,
      2., 4., 8., 16.,
      4., 8., 16., 32.,
    ]);

    assert_eq!(a * Matrix4x4::IDENTITY, a);
  }

  #[test]
  fn matrix_transpose_should_work_correctly() {
    #[rustfmt::skip]
    let a = Matrix4x4::create(&[
      0., 9., 3., 0.,
      9., 8., 0., 8.,
      1., 8., 5., 3.,
      0., 0., 5., 8.,
    ]);

    #[rustfmt::skip]
    assert_eq!(a.transpose(), Matrix4x4::create(&[
      0., 9., 1., 0.,
      9., 8., 8., 0.,
      3., 0., 5., 5.,
      0., 8., 3., 8.,
    ]));
  }

  #[test]
  fn matrix_transpose_of_identity_is_identity() {
    assert_eq!(Matrix4x4::<f32>::IDENTITY.transpose(), Matrix4x4::IDENTITY);
  }

  #[test]
  fn determinant_should_be_calculated_correctly() {
    #[rustfmt::skip]
    let a = Matrix2x2::create(&[
      1., 5.,
      -3., 2.,
    ]);

    assert_eq!(a.determinant(), 17.);
  }
}
