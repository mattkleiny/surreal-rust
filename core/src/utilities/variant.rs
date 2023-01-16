use crate::{
  graphics::{Color, Color32},
  maths::{Quat, Vec2, Vec3, Vec4},
};

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
  Color,
  Color32,
}

/// A type that can hold varying different values.
#[derive(Clone, Debug)]
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
  Color(Color),
  Color32(Color32),
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
      Variant::Color(_) => VariantKind::Color,
      Variant::Color32(_) => VariantKind::Color32,
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

    impl TryFrom<Variant> for $type {
      type Error = anyhow::Error;

      fn try_from(value: Variant) -> Result<Self, Self::Error> {
        match value {
          Variant::$kind(value) => Ok(value),
          _ => Err(anyhow::anyhow!("Variant is not a string")),
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
impl_variant!(Color, Color);
impl_variant!(Color32, Color32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn variant_should_convert_from_values() {
    let value = Variant::from(Color32::WHITE);
    assert_eq!(value.kind(), VariantKind::Color32);

    let color: Color32 = value.try_into().expect("Failed to convert");
    assert_eq!(color, Color32::WHITE);
  }
}
