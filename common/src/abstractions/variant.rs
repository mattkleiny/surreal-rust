use crate::{
  maths::{Quat, Vec2, Vec3, Vec4},
  strings::StringName,
};

/// A type that can hold varying different values.
///
/// This is an abstraction over the different primitive types that are often
/// shuffled around in the engine. It allows for a more generic API that can
/// handle any type of value.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum Variant {
  Null,
  Bool(bool),
  U8(u8),
  U16(u16),
  U32(u32),
  U64(u64),
  I8(i8),
  I16(i16),
  I32(i32),
  I64(i64),
  F32(f32),
  F64(f64),
  String(String),
  StringName(StringName),
  Vec2(Vec2),
  Vec3(Vec3),
  Vec4(Vec4),
  Quat(Quat),
  Packed(Vec<u8>),
}

macro_rules! impl_variant {
  ($type:ty, $kind:ident) => {
    impl From<$type> for Variant {
      #[inline]
      fn from(value: $type) -> Self {
        Self::$kind(value)
      }
    }

    impl From<Variant> for $type {
      #[inline]
      fn from(value: Variant) -> Self {
        match value {
          Variant::$kind(value) => value,
          _ => panic!("Variant is not convertible"),
        }
      }
    }
  };
}

impl_variant!(bool, Bool);
impl_variant!(u8, U8);
impl_variant!(u16, U16);
impl_variant!(u32, U32);
impl_variant!(u64, U64);
impl_variant!(i8, I8);
impl_variant!(i16, I16);
impl_variant!(i32, I32);
impl_variant!(i64, I64);
impl_variant!(f32, F32);
impl_variant!(f64, F64);
impl_variant!(String, String);
impl_variant!(StringName, StringName);
impl_variant!(Vec2, Vec2);
impl_variant!(Vec3, Vec3);
impl_variant!(Vec4, Vec4);
impl_variant!(Quat, Quat);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_variant_conversion() {
    let value: bool = true;
    let variant: Variant = value.into();

    assert_eq!(variant, Variant::Bool(true));

    let variant: Variant = Variant::U8(10);
    let value: u8 = variant.into();

    assert_eq!(value, 10);
  }
}
