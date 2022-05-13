use crate::maths::{Vector2, Vector3, Vector4};

#[derive(Clone, Debug)]
pub struct Matrix2<T> {
  row1: Vector2<T>,
  row2: Vector2<T>,
}

#[derive(Clone, Debug)]
pub struct Matrix3<T> {
  row1: Vector3<T>,
  row2: Vector3<T>,
  row3: Vector3<T>,
}

#[derive(Clone, Debug)]
pub struct Matrix4<T> {
  row1: Vector4<T>,
  row2: Vector4<T>,
  row3: Vector4<T>,
  row4: Vector4<T>,
}