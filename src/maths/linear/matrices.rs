use crate::maths::{Numeric, vec2, vec3, vec4, Vector2, Vector3, Vector4};

/// A standard 2x2 matrix.
#[derive(Clone, Debug)]
pub struct Matrix2<T> {
  row1: Vector2<T>,
  row2: Vector2<T>,
}

impl<T> Default for Matrix2<T> where T: Numeric {
  fn default() -> Self {
    Self {
      row1: vec2(T::ONE, T::ZERO),
      row2: vec2(T::ZERO, T::ONE),
    }
  }
}

/// A standard 3x3 matrix.
#[derive(Clone, Debug)]
pub struct Matrix3<T> {
  row1: Vector3<T>,
  row2: Vector3<T>,
  row3: Vector3<T>,
}

impl<T> Default for Matrix3<T> where T: Numeric {
  fn default() -> Self {
    Self {
      row1: vec3(T::ONE, T::ZERO, T::ZERO),
      row2: vec3(T::ZERO, T::ONE, T::ZERO),
      row3: vec3(T::ZERO, T::ZERO, T::ONE),
    }
  }
}

/// A standard 4x4 matrix.
#[derive(Clone, Debug)]
pub struct Matrix4<T> {
  row1: Vector4<T>,
  row2: Vector4<T>,
  row3: Vector4<T>,
  row4: Vector4<T>,
}

impl<T> Default for Matrix4<T> where T: Numeric {
  fn default() -> Self {
    Self {
      row1: vec4(T::ONE, T::ZERO, T::ZERO, T::ZERO),
      row2: vec4(T::ZERO, T::ONE, T::ZERO, T::ZERO),
      row3: vec4(T::ZERO, T::ZERO, T::ONE, T::ZERO),
      row4: vec4(T::ZERO, T::ZERO, T::ZERO, T::ONE),
    }
  }
}
