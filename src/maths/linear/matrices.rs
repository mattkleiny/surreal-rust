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

/// Specialization for floating point 4x4 matrices.
impl Matrix4x4<f32> {
  /// Creates a new translation matrix.
  pub fn create_translation(x: f32, y: f32, z: f32) -> Self {
    Self::create(&[
      1.0, 0.0, 0.0, x,
      0.0, 1.0, 0.0, y,
      0.0, 0.0, 1.0, z,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new scale matrix.
  pub fn create_scale(x: f32, y: f32, z: f32) -> Self {
    Self::create(&[
      x, 0.0, 0.0, 0.0,
      0.0, y, 0.0, 0.0,
      0.0, 0.0, z, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new rotation matrix about the X axis.
  pub fn create_rotate_x(r: f32) -> Self {
    Self::create(&[
      1.0, 0.0, 0.0, 0.0,
      0.0, r.cos(), -r.sin(), 0.0,
      0.0, r.sin(), r.cos(), 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new rotation matrix about the Y axis.
  pub fn create_rotate_y(r: f32) -> Self {
    Self::create(&[
      r.cos(), 0.0, r.sin(), 0.0,
      0.0, 1.0, 0.0, 0.0,
      -r.sin(), 0.0, r.cos(), 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new rotation matrix about the Z axis.
  pub fn create_rotate_z(r: f32) -> Self {
    Self::create(&[
      r.cos(), -r.sin(), 0.0, 0.0,
      r.sin(), r.cos(), 0.0, 0.0,
      0.0, 0.0, 1.0, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new shearing matrix with the given proportions.
  pub fn create_shear(x1: f32, x2: f32, y1: f32, y2: f32, z1: f32, z2: f32) -> Self {
    Self::create(&[
      1.0, x1, x2, 0.0,
      y1, 1.0, y2, 0.0,
      z1, z2, 1.0, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ])
  }

  /// Creates a new view transformation that looks at the given point.
  pub fn create_look_at(from: Vector3<f32>, to: Vector3<f32>, up: Vector3<f32>) -> Self {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());
    let true_up = left.cross(forward);

    let orientation = Self::create(&[
      left.x, left.y, left.z, 0.0,
      true_up.x, true_up.y, true_up.z, 0.0,
      -forward.x, -forward.y, -forward.z, 0.0,
      0.0, 0.0, 0.0, 1.0,
    ]);

    orientation * Self::create_translation(-from.x, -from.y, -from.z)
  }

  /// Creates a new orthographic projection matrix.
  pub fn create_orthographic(width: f32, height: f32, near: f32, far: f32) -> Self {
    Self::create_orthographic_off_center(-width / 2., width / 2., -height / 2., height / 2., near, far)
  }

  /// Creates a new orthographic projection matrix.
  pub fn create_orthographic_off_center(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Self {
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

#[cfg(test)]
mod tests {
  use std::f32::consts::PI;

  use super::*;

  #[test]
  fn translation_should_transform_point() {
    let transform = Matrix4x4::create_translation(5.0, -3.0, 2.0);
    let p = point(-3.0, 4.0, 5.0);

    assert_eq!(transform * p, point(2.0, 1.0, 7.0));
  }

  #[test]
  fn inverse_translation_should_transform_point() {
    let transform = Matrix4x4::create_translation(5.0, -3.0, 2.0);
    let inverse = transform.invert().expect("Failed to invert");

    let p = point(-3.0, 4.0, 5.0);

    assert_eq!(inverse * p, point(-8.0, 7.0, 3.0));
  }

  #[test]
  fn translation_does_not_affect_vectors() {
    let transform = Matrix4x4::create_translation(5.0, -3.0, 2.0);
    let v = vec3(3.0, 4.0, 5.0);

    assert_eq!(transform * v, v);
  }

  #[test]
  fn scale_should_transform_point() {
    let transform = Matrix4x4::create_scale(2., 3., 4.);
    let p = point(-4., 6., 8.);

    assert_eq!(transform * p, point(-8., 18., 32.));
  }

  #[test]
  fn scale_should_transform_vector() {
    let transform = Matrix4x4::create_scale(2., 3., 4.);
    let p = vec3(-4., 6., 8.);

    assert_eq!(transform * p, vec3(-8., 18., 32.));
  }

  #[test]
  fn inverse_scale_should_transform_point() {
    let transform = Matrix4x4::create_scale(2., 3., 4.);
    let inverse = transform.invert().expect("Failed to invert");

    let p = point(-4., 6., 8.);

    assert_eq!(inverse * p, point(-2., 2., 2.));
  }

  #[test]
  fn scale_should_reflect_point() {
    let transform = Matrix4x4::create_scale(-1., 1., 1.);
    let p = point(2., 3., 4.);

    assert_eq!(transform * p, point(-2., 3., 4.));
  }

  #[test]
  fn rotate_around_x_axis() {
    let p = point(0., 1., 0.);

    let half_quarter = Matrix4x4::create_rotate_x(PI / 4.);
    let full_quarter = Matrix4x4::create_rotate_x(PI / 2.);

    assert_eq!(half_quarter * p, point(0., 2f64.sqrt() / 2., 2f64.sqrt() / 2.));
    assert_eq!(full_quarter * p, point(0., 0., 1.));
  }

  #[test]
  fn inverse_rotate_around_x_axis() {
    let p = point(0., 1., 0.);

    let half_quarter = Matrix4x4::create_rotate_x(PI / 4.);
    let inverse = half_quarter.invert().expect("Failed to invert");

    assert_eq!(inverse * p, point(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.));
  }

  #[test]
  fn rotate_around_y_axis() {
    let p = point(0., 0., 1.);

    let half_quarter = Matrix4x4::create_rotate_y(PI / 4.);
    let full_quarter = Matrix4x4::create_rotate_y(PI / 2.);

    assert_eq!(half_quarter * p, point(2f64.sqrt() / 2., 0., 2f64.sqrt() / 2.));
    assert_eq!(full_quarter * p, point(1., 0., 0.));
  }

  #[test]
  fn rotate_around_z_axis() {
    let p = point(0., 1., 0.);

    let half_quarter = Matrix4x4::create_rotate_z(PI / 4.);
    let full_quarter = Matrix4x4::create_rotate_z(PI / 2.);

    assert_eq!(half_quarter * p, point(-2f64.sqrt() / 2., 2f64.sqrt() / 2., 0.));
    assert_eq!(full_quarter * p, point(-1., 0., 0.));
  }

  #[test]
  fn shearing_should_move_x_in_proportion_to_y() {
    let transform = Matrix4x4::create_shear(1., 0., 0., 0., 0., 0.);
    let p = point(2., 3., 4.);

    assert_eq!(transform * p, point(5., 3., 4.));
  }

  #[test]
  fn shearing_should_move_x_in_proportion_to_z() {
    let transform = Matrix4x4::create_shear(0., 1., 0., 0., 0., 0.);
    let p = point(2., 3., 4.);

    assert_eq!(transform * p, point(6., 3., 4.));
  }

  #[test]
  fn shearing_should_move_y_in_proportion_to_x() {
    let transform = Matrix4x4::create_shear(0., 0., 1., 0., 0., 0.);
    let p = point(2., 3., 4.);

    assert_eq!(transform * p, point(2., 5., 4.));
  }

  #[test]
  fn shearing_should_move_y_in_proportion_to_z() {
    let transform = Matrix4x4::create_shear(0., 0., 0., 1., 0., 0.);
    let p = point(2., 3., 4.);

    assert_eq!(transform * p, point(2., 7., 4.));
  }

  #[test]
  fn shearing_should_move_z_in_proportion_to_x() {
    let transform = Matrix4x4::create_shear(0., 0., 0., 0., 1., 0.);
    let p = point(2., 3., 4.);

    assert_eq!(transform * p, point(2., 3., 6.));
  }

  #[test]
  fn shearing_should_move_z_in_proportion_to_y() {
    let transform = Matrix4x4::create_shear(0., 0., 0., 0., 0., 1.);
    let p = point(2., 3., 4.);

    assert_eq!(transform * p, point(2., 3., 7.));
  }

  #[test]
  fn individual_transforms_are_applied_in_sequence() {
    let p = point(1., 0., 1.);

    let a = Matrix4x4::create_rotate_x(PI / 2.);
    let b = Matrix4x4::create_scale(5., 5., 5.);
    let c = Matrix4x4::create_translation(10., 5., 7.);

    let p2 = a * p;
    assert_eq!(p2, point(1., -1., 0.));

    let p3 = b * p2;
    assert_eq!(p3, point(5., -5., 0.));

    let p4 = c * p3;
    assert_eq!(p4, point(15., 0., 7.));
  }

  #[test]
  fn chained_transformations_are_applied_in_reverse_order() {
    let p = point(1., 0., 1.);

    let a = Matrix4x4::create_rotate_x(PI / 2.);
    let b = Matrix4x4::create_scale(5., 5., 5.);
    let c = Matrix4x4::create_translation(10., 5., 7.);

    let transform = c * b * a;

    assert_eq!(transform * p, point(15., 0., 7.));
  }

  #[test]
  fn look_at_default_orientation() {
    let from = point(0., 0., 0.);
    let to = point(0., 0., -1.);
    let up = vec3(0., 1., 0.);

    let transform = Matrix4x4::create_look_at(from, to, up);

    assert_eq!(transform, Matrix4x4::IDENTITY);
  }

  #[test]
  fn look_at_positive_z_direction() {
    let from = point(0., 0., 0.);
    let to = point(0., 0., 1.);
    let up = vec3(0., 1., 0.);

    let transform = Matrix4x4::create_look_at(from, to, up);

    assert_eq!(transform, Matrix4x4::create_scale(-1., 1., -1.));
  }

  #[test]
  fn look_at_moves_the_world() {
    let from = point(0., 0., 8.);
    let to = point(0., 0., 0.);
    let up = vec3(0., 1., 0.);

    let transform = Matrix4x4::create_look_at(from, to, up);

    assert_eq!(transform, Matrix4x4::create_translation(0., 0., -8.));
  }

  #[test]
  fn look_at_arbitrary_point() {
    let from = point(1., 3., 2.);
    let to = point(4., -2., 8.);
    let up = vec3(1., 1., 0.);

    let transform = Matrix4x4::create_look_at(from, to, up);

    assert_eq!(transform, Matrix4x4::create(&[
      -0.50709, 0.50709, 0.67612, -2.36643,
      0.76772, 0.60609, 0.12122, -2.82843,
      -0.35857, 0.59761, -0.71714, 0.00000,
      0.000000, 0.00000, 0.00000, 1.00000
    ]));
  }
}