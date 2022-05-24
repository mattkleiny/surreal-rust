use crate::maths::*;

/// A simple variant type that can hold different kinds of objects.
#[derive(Clone, Debug)]
pub enum Variant<'a> {
  Bool(bool),
  I32(i32),
  I64(i64),
  F32(f32),
  F64(f64),
  Point2(&'a Vector2<i32>),
  Point3(&'a Vector3<i32>),
  Point4(&'a Vector4<i32>),
  Vector2(&'a Vector2<f32>),
  Vector3(&'a Vector3<f32>),
  Vector4(&'a Vector4<f32>),
  Matrix2x2(&'a Matrix2x2<f32>),
  Matrix3x3(&'a Matrix3x3<f32>),
  Matrix4x4(&'a Matrix4x4<f32>),
}

/// Implements variant value transformation for common variant types.
macro_rules! implement_variant {
  ($type:ty, $value:ident) => {
    impl<'a> From<$type> for Variant<'a> {
      fn from(value: $type) -> Variant<'a> {
        Variant::$value(value)
      }
    }
  };
}

implement_variant!(i32, I32);
implement_variant!(i64, I64);
implement_variant!(f32, F32);
implement_variant!(f64, F64);
implement_variant!(&'a Vector2<i32>, Point2);
implement_variant!(&'a Vector3<i32>, Point3);
implement_variant!(&'a Vector4<i32>, Point4);
implement_variant!(&'a Vector2<f32>, Vector2);
implement_variant!(&'a Vector3<f32>, Vector3);
implement_variant!(&'a Vector4<f32>, Vector4);
implement_variant!(&'a Matrix2x2<f32>, Matrix2x2);
implement_variant!(&'a Matrix3x3<f32>, Matrix3x3);
implement_variant!(&'a Matrix4x4<f32>, Matrix4x4);

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn variant_should_have_predictable_size() {
    assert_eq!(std::mem::size_of::<Variant>(), 16);
  }
}
