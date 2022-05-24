/// A simple variant type that can hold different kinds of objects.
pub enum Variant {
  Bool(bool),
  I32(i32),
  I64(i64),
  F32(f32),
  F64(f64),
  Point2(Vector2<i32>),
  Point3(Vector3<i32>),
  Point4(Vector4<i32>),
  Vector2(Vector2<f32>),
  Vector3(Vector3<f32>),
  Vector4(Vector4<f32>),
}

/// Implements variant value transformation for common variant types.
macro_rules! implement_variant {
  ($type:ty, $value:ident) => {
    impl Into<Variant> for $type {
      fn into(self) -> Variant {
        Variant::$value(self)
      }
    }
  };
}

implement_variant!(i32, I32);
implement_variant!(i64, I64);
implement_variant!(f32, F32);
implement_variant!(f64, F64);
implement_variant!(Vector2<i32>, Point2);
implement_variant!(Vector3<i32>, Point3);
implement_variant!(Vector4<i32>, Point4);
implement_variant!(Vector2<f32>, Vector2);
implement_variant!(Vector3<f32>, Vector3);
implement_variant!(Vector4<f32>, Vector4);