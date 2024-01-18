use crate::maths::{Quat, Vec2, Vec3, Vec4};

/// Different kinds of [`Variant`]s that are supported.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VariantKind {
  Null,
  Bool,
  U8,
  U16,
  U32,
  U64,
  I8,
  I16,
  I32,
  I64,
  F32,
  F64,
  String,
  Vec2,
  Vec3,
  Vec4,
  Quat,
}

/// A type that can hold varying different values.
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
  Vec2(Vec2),
  Vec3(Vec3),
  Vec4(Vec4),
  Quat(Quat),
}

impl Variant {
  /// Determines the [`VariantKind`] of this value.
  pub const fn kind(&self) -> VariantKind {
    match self {
      Variant::Null => VariantKind::Null,
      Variant::Bool(_) => VariantKind::Bool,
      Variant::U8(_) => VariantKind::U8,
      Variant::U16(_) => VariantKind::U16,
      Variant::U32(_) => VariantKind::U32,
      Variant::U64(_) => VariantKind::U64,
      Variant::I8(_) => VariantKind::I8,
      Variant::I16(_) => VariantKind::I16,
      Variant::I32(_) => VariantKind::I32,
      Variant::I64(_) => VariantKind::I64,
      Variant::F32(_) => VariantKind::F32,
      Variant::F64(_) => VariantKind::F64,
      Variant::String(_) => VariantKind::String,
      Variant::Vec2(_) => VariantKind::Vec2,
      Variant::Vec3(_) => VariantKind::Vec3,
      Variant::Vec4(_) => VariantKind::Vec4,
      Variant::Quat(_) => VariantKind::Quat,
    }
  }
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
impl_variant!(Vec2, Vec2);
impl_variant!(Vec3, Vec3);
impl_variant!(Vec4, Vec4);
impl_variant!(Quat, Quat);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_variant_kind() {
    let variant = Variant::Null;
    assert_eq!(variant.kind(), VariantKind::Null);

    let variant = Variant::Bool(true);
    assert_eq!(variant.kind(), VariantKind::Bool);

    let variant = Variant::U8(10);
    assert_eq!(variant.kind(), VariantKind::U8);
  }

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
