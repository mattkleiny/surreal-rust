use std::{any::Any, cmp::Ordering};

use crate::{Color, Color32, Pointer, Quat, StringName, Vec2, Vec3, Vec4};

/// An error that can occur when working with variants.
#[derive(Debug)]
pub enum VariantError {
  InvalidNegation,
  InvalidConversion,
  NonArithmetic,
}

/// Allows for a type to be converted to a [`Variant`].
pub trait ToVariant {
  /// Converts the type into a [`Variant`].
  fn to_variant(&self) -> Variant;
}

/// Allows for a type to be converted from a [`Variant`].
pub trait FromVariant: Sized {
  /// Converts a [`Variant`] into the type.
  fn from_variant(variant: Variant) -> Result<Self, VariantError>;
}

/// The different kinds of values that a [`Variant`] can hold.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum VariantKind {
  Null,
  Bool,
  Char,
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
  StringName,
  Vec2,
  Vec3,
  Vec4,
  Quat,
  Color,
  Color32,
  UserData,
}

/// A type that can hold varying different values.
///
/// This is an abstraction over the different primitive types that are often
/// shuffled around in the engine. It allows for a more generic API that can
/// handle any type of value.
#[derive(Default, Debug)]
pub enum Variant {
  #[default]
  Null,
  Bool(bool),
  Char(char),
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
  Color(Color),
  Color32(Color32),
  UserData(Pointer<dyn Any>),
}

impl PartialEq for Variant {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Variant::Null, Variant::Null) => true,
      (Variant::Bool(a), Variant::Bool(b)) => a == b,
      (Variant::Char(a), Variant::Char(b)) => a == b,
      (Variant::U8(a), Variant::U8(b)) => a == b,
      (Variant::U16(a), Variant::U16(b)) => a == b,
      (Variant::U32(a), Variant::U32(b)) => a == b,
      (Variant::U64(a), Variant::U64(b)) => a == b,
      (Variant::I8(a), Variant::I8(b)) => a == b,
      (Variant::I16(a), Variant::I16(b)) => a == b,
      (Variant::I32(a), Variant::I32(b)) => a == b,
      (Variant::I64(a), Variant::I64(b)) => a == b,
      (Variant::F32(a), Variant::F32(b)) => a == b,
      (Variant::F64(a), Variant::F64(b)) => a == b,
      (Variant::String(a), Variant::String(b)) => a == b,
      (Variant::StringName(a), Variant::StringName(b)) => a == b,
      (Variant::Vec2(a), Variant::Vec2(b)) => a == b,
      (Variant::Vec3(a), Variant::Vec3(b)) => a == b,
      (Variant::Vec4(a), Variant::Vec4(b)) => a == b,
      (Variant::Quat(a), Variant::Quat(b)) => a == b,
      (Variant::Color(a), Variant::Color(b)) => a == b,
      (Variant::Color32(a), Variant::Color32(b)) => a == b,
      (Variant::UserData(a), Variant::UserData(b)) => {
        // pointer comparison
        let ptr_a = &**a as *const dyn Any;
        let ptr_b = &**b as *const dyn Any;

        std::ptr::addr_eq(ptr_a, ptr_b)
      }
      _ => false,
    }
  }
}

impl Clone for Variant {
  fn clone(&self) -> Self {
    match self {
      Variant::Null => Variant::Null,
      Variant::Bool(value) => Variant::Bool(*value),
      Variant::Char(value) => Variant::Char(*value),
      Variant::U8(value) => Variant::U8(*value),
      Variant::U16(value) => Variant::U16(*value),
      Variant::U32(value) => Variant::U32(*value),
      Variant::U64(value) => Variant::U64(*value),
      Variant::I8(value) => Variant::I8(*value),
      Variant::I16(value) => Variant::I16(*value),
      Variant::I32(value) => Variant::I32(*value),
      Variant::I64(value) => Variant::I64(*value),
      Variant::F32(value) => Variant::F32(*value),
      Variant::F64(value) => Variant::F64(*value),
      Variant::String(value) => Variant::String(value.clone()),
      Variant::StringName(value) => Variant::StringName(value.clone()),
      Variant::Vec2(value) => Variant::Vec2(value.clone()),
      Variant::Vec3(value) => Variant::Vec3(value.clone()),
      Variant::Vec4(value) => Variant::Vec4(value.clone()),
      Variant::Quat(value) => Variant::Quat(value.clone()),
      Variant::Color(value) => Variant::Color(value.clone()),
      Variant::Color32(value) => Variant::Color32(value.clone()),
      Variant::UserData(value) => Variant::UserData(value.clone()),
    }
  }
}

impl Variant {
  /// Determines the kind of value that this variant holds.
  pub fn kind(&self) -> VariantKind {
    match self {
      Variant::Null => VariantKind::Null,
      Variant::Bool(_) => VariantKind::Bool,
      Variant::Char(_) => VariantKind::Char,
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
      Variant::StringName(_) => VariantKind::StringName,
      Variant::Vec2(_) => VariantKind::Vec2,
      Variant::Vec3(_) => VariantKind::Vec3,
      Variant::Vec4(_) => VariantKind::Vec4,
      Variant::Quat(_) => VariantKind::Quat,
      Variant::Color(_) => VariantKind::Color,
      Variant::Color32(_) => VariantKind::Color32,
      Variant::UserData(_) => VariantKind::UserData,
    }
  }

  /// Determines if the variant is a scalar value.
  pub fn is_scalar(&self) -> bool {
    matches!(
      self.kind(),
      VariantKind::U8
        | VariantKind::U16
        | VariantKind::U32
        | VariantKind::U64
        | VariantKind::I8
        | VariantKind::I16
        | VariantKind::I32
        | VariantKind::I64
        | VariantKind::F32
        | VariantKind::F64
    )
  }

  /// Determines if the variant is a vector value.
  pub fn is_vector(&self) -> bool {
    matches!(
      self.kind(),
      VariantKind::Vec2
        | VariantKind::Vec3
        | VariantKind::Vec4
        | VariantKind::Quat
        | VariantKind::Color
        | VariantKind::Color32
    )
  }

  /// Determines if the variant is a string value.
  pub fn is_string(&self) -> bool {
    matches!(self.kind(), VariantKind::String | VariantKind::StringName)
  }
}

impl std::ops::Neg for Variant {
  type Output = Result<Self, VariantError>;

  fn neg(self) -> Self::Output {
    match self {
      Variant::Bool(value) => Ok(Variant::Bool(!value)),
      Variant::I8(value) => Ok(Variant::I8(value.neg())),
      Variant::I16(value) => Ok(Variant::I16(value.neg())),
      Variant::I32(value) => Ok(Variant::I32(value.neg())),
      Variant::I64(value) => Ok(Variant::I64(value.neg())),
      Variant::Vec2(value) => Ok(Variant::Vec2(value.neg())),
      Variant::Vec3(value) => Ok(Variant::Vec3(value.neg())),
      Variant::Vec4(value) => Ok(Variant::Vec4(value.neg())),
      Variant::Quat(value) => Ok(Variant::Quat(value.neg())),
      _ => Err(VariantError::InvalidNegation),
    }
  }
}

impl std::ops::Add for Variant {
  type Output = Result<Self, VariantError>;

  fn add(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Variant::U8(a), Variant::U8(b)) => Ok(Variant::U8(a + b)),
      (Variant::U16(a), Variant::U16(b)) => Ok(Variant::U16(a + b)),
      (Variant::U32(a), Variant::U32(b)) => Ok(Variant::U32(a + b)),
      (Variant::U64(a), Variant::U64(b)) => Ok(Variant::U64(a + b)),
      (Variant::I8(a), Variant::I8(b)) => Ok(Variant::I8(a + b)),
      (Variant::I16(a), Variant::I16(b)) => Ok(Variant::I16(a + b)),
      (Variant::I32(a), Variant::I32(b)) => Ok(Variant::I32(a + b)),
      (Variant::I64(a), Variant::I64(b)) => Ok(Variant::I64(a + b)),
      (Variant::F32(a), Variant::F32(b)) => Ok(Variant::F32(a + b)),
      (Variant::F64(a), Variant::F64(b)) => Ok(Variant::F64(a + b)),
      (Variant::Vec2(a), Variant::Vec2(b)) => Ok(Variant::Vec2(a + b)),
      (Variant::Vec3(a), Variant::Vec3(b)) => Ok(Variant::Vec3(a + b)),
      (Variant::Vec4(a), Variant::Vec4(b)) => Ok(Variant::Vec4(a + b)),
      (Variant::Quat(a), Variant::Quat(b)) => Ok(Variant::Quat(a + b)),
      (Variant::Color(a), Variant::Color(b)) => Ok(Variant::Color(a + b)),
      (Variant::Color32(a), Variant::Color32(b)) => Ok(Variant::Color32(a + b)),
      _ => Err(VariantError::NonArithmetic),
    }
  }
}

impl std::ops::Sub for Variant {
  type Output = Result<Self, VariantError>;

  fn sub(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Variant::U8(a), Variant::U8(b)) => Ok(Variant::U8(a - b)),
      (Variant::U16(a), Variant::U16(b)) => Ok(Variant::U16(a - b)),
      (Variant::U32(a), Variant::U32(b)) => Ok(Variant::U32(a - b)),
      (Variant::U64(a), Variant::U64(b)) => Ok(Variant::U64(a - b)),
      (Variant::I8(a), Variant::I8(b)) => Ok(Variant::I8(a - b)),
      (Variant::I16(a), Variant::I16(b)) => Ok(Variant::I16(a - b)),
      (Variant::I32(a), Variant::I32(b)) => Ok(Variant::I32(a - b)),
      (Variant::I64(a), Variant::I64(b)) => Ok(Variant::I64(a - b)),
      (Variant::F32(a), Variant::F32(b)) => Ok(Variant::F32(a - b)),
      (Variant::F64(a), Variant::F64(b)) => Ok(Variant::F64(a - b)),
      (Variant::Vec2(a), Variant::Vec2(b)) => Ok(Variant::Vec2(a - b)),
      (Variant::Vec3(a), Variant::Vec3(b)) => Ok(Variant::Vec3(a - b)),
      (Variant::Vec4(a), Variant::Vec4(b)) => Ok(Variant::Vec4(a - b)),
      (Variant::Quat(a), Variant::Quat(b)) => Ok(Variant::Quat(a - b)),
      (Variant::Color(a), Variant::Color(b)) => Ok(Variant::Color(a - b)),
      (Variant::Color32(a), Variant::Color32(b)) => Ok(Variant::Color32(a - b)),
      _ => Err(VariantError::NonArithmetic),
    }
  }
}

impl std::ops::Mul for Variant {
  type Output = Result<Self, VariantError>;

  fn mul(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Variant::U8(a), Variant::U8(b)) => Ok(Variant::U8(a * b)),
      (Variant::U16(a), Variant::U16(b)) => Ok(Variant::U16(a * b)),
      (Variant::U32(a), Variant::U32(b)) => Ok(Variant::U32(a * b)),
      (Variant::U64(a), Variant::U64(b)) => Ok(Variant::U64(a * b)),
      (Variant::I8(a), Variant::I8(b)) => Ok(Variant::I8(a * b)),
      (Variant::I16(a), Variant::I16(b)) => Ok(Variant::I16(a * b)),
      (Variant::I32(a), Variant::I32(b)) => Ok(Variant::I32(a * b)),
      (Variant::I64(a), Variant::I64(b)) => Ok(Variant::I64(a * b)),
      (Variant::F32(a), Variant::F32(b)) => Ok(Variant::F32(a * b)),
      (Variant::F64(a), Variant::F64(b)) => Ok(Variant::F64(a * b)),
      (Variant::Vec2(a), Variant::Vec2(b)) => Ok(Variant::Vec2(a * b)),
      (Variant::Vec3(a), Variant::Vec3(b)) => Ok(Variant::Vec3(a * b)),
      (Variant::Vec4(a), Variant::Vec4(b)) => Ok(Variant::Vec4(a * b)),
      (Variant::Quat(a), Variant::Quat(b)) => Ok(Variant::Quat(a * b)),
      (Variant::Color(a), Variant::Color(b)) => Ok(Variant::Color(a * b)),
      (Variant::Color32(a), Variant::Color32(b)) => Ok(Variant::Color32(a * b)),
      _ => Err(VariantError::NonArithmetic),
    }
  }
}

impl std::ops::Div for Variant {
  type Output = Result<Self, VariantError>;

  fn div(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Variant::U8(a), Variant::U8(b)) => Ok(Variant::U8(a / b)),
      (Variant::U16(a), Variant::U16(b)) => Ok(Variant::U16(a / b)),
      (Variant::U32(a), Variant::U32(b)) => Ok(Variant::U32(a / b)),
      (Variant::U64(a), Variant::U64(b)) => Ok(Variant::U64(a / b)),
      (Variant::I8(a), Variant::I8(b)) => Ok(Variant::I8(a / b)),
      (Variant::I16(a), Variant::I16(b)) => Ok(Variant::I16(a / b)),
      (Variant::I32(a), Variant::I32(b)) => Ok(Variant::I32(a / b)),
      (Variant::I64(a), Variant::I64(b)) => Ok(Variant::I64(a / b)),
      (Variant::F32(a), Variant::F32(b)) => Ok(Variant::F32(a / b)),
      (Variant::F64(a), Variant::F64(b)) => Ok(Variant::F64(a / b)),
      _ => Err(VariantError::NonArithmetic),
    }
  }
}

impl std::ops::Rem for Variant {
  type Output = Result<Self, VariantError>;

  fn rem(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (Variant::U8(a), Variant::U8(b)) => Ok(Variant::U8(a % b)),
      (Variant::U16(a), Variant::U16(b)) => Ok(Variant::U16(a % b)),
      (Variant::U32(a), Variant::U32(b)) => Ok(Variant::U32(a % b)),
      (Variant::U64(a), Variant::U64(b)) => Ok(Variant::U64(a % b)),
      (Variant::I8(a), Variant::I8(b)) => Ok(Variant::I8(a % b)),
      (Variant::I16(a), Variant::I16(b)) => Ok(Variant::I16(a % b)),
      (Variant::I32(a), Variant::I32(b)) => Ok(Variant::I32(a % b)),
      (Variant::I64(a), Variant::I64(b)) => Ok(Variant::I64(a % b)),
      _ => Err(VariantError::NonArithmetic),
    }
  }
}

/// Specialized ordering for variant types.
impl PartialOrd for Variant {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match (self, other) {
      (Variant::U8(a), Variant::U8(b)) => a.partial_cmp(b),
      (Variant::U16(a), Variant::U16(b)) => a.partial_cmp(b),
      (Variant::U32(a), Variant::U32(b)) => a.partial_cmp(b),
      (Variant::U64(a), Variant::U64(b)) => a.partial_cmp(b),
      (Variant::I8(a), Variant::I8(b)) => a.partial_cmp(b),
      (Variant::I16(a), Variant::I16(b)) => a.partial_cmp(b),
      (Variant::I32(a), Variant::I32(b)) => a.partial_cmp(b),
      (Variant::I64(a), Variant::I64(b)) => a.partial_cmp(b),
      (Variant::F32(a), Variant::F32(b)) => a.partial_cmp(b),
      (Variant::F64(a), Variant::F64(b)) => a.partial_cmp(b),
      _ => None,
    }
  }
}

/// Allows conversion to/from a `Variant` for a given type.
macro_rules! impl_variant {
  ((), $kind:ident) => {
    impl ToVariant for () {
      #[inline]
      fn to_variant(&self) -> Variant {
        Variant::$kind
      }
    }

    impl FromVariant for () {
      #[inline]
      fn from_variant(_: Variant) -> Result<Self, VariantError> {
        Ok(()) // no-op
      }
    }
  };

  ($type:ty, $kind:ident) => {
    impl ToVariant for $type {
      #[inline]
      fn to_variant(&self) -> Variant {
        Variant::$kind(self.clone())
      }
    }

    impl FromVariant for $type {
      #[inline]
      fn from_variant(value: Variant) -> Result<Self, VariantError> {
        match value {
          Variant::$kind(value) => Ok(value),
          _ => Err(VariantError::InvalidConversion),
        }
      }
    }
  };

  ($type:ty, $kind:ident, $($kinds:ident),*) => {
    impl ToVariant for $type {
      #[inline]
      fn to_variant(&self) -> Variant {
        Variant::$kind(self.clone())
      }
    }

    impl FromVariant for $type {
      #[inline]
      fn from_variant(value: Variant) -> Result<Self, VariantError> {
        match value {
          Variant::$kind(value) => Ok(value),
          $(Variant::$kinds(value) => Ok(value as $type),)*
          _ => Err(VariantError::InvalidConversion),
        }
      }
    }
  }
}

impl_variant!((), Null);
impl_variant!(bool, Bool);
impl_variant!(char, Char);
impl_variant!(u8, U8, I8, U16, I16, U32, I32, U64, I64, F32, F64);
impl_variant!(i8, I8, U8, I16, U16, U32, I32, U64, I64, F32, F64);
impl_variant!(u16, U16, I8, U8, I16, U32, I32, U64, I64, F32, F64);
impl_variant!(i16, I16, U8, I8, U16, U32, I32, U64, I64, F32, F64);
impl_variant!(u32, U32, I8, I16, U8, U16, I32, I64, U64, F32, F64);
impl_variant!(i32, I32, U8, I8, U16, I16, U32, I64, U64, F32, F64);
impl_variant!(u64, U64, I8, I16, I32, U8, U16, U32, F32, F64);
impl_variant!(i64, I64, U8, I8, I16, U16, U32, U64, F32, F64);
impl_variant!(f32, F32, I8, I16, I32, I64, U8, U16, U32, U64, F64);
impl_variant!(f64, F64, U8, U16, U32, U64, I8, I16, I32, I64, F32);
impl_variant!(String, String);
impl_variant!(StringName, StringName);
impl_variant!(Vec2, Vec2);
impl_variant!(Vec3, Vec3);
impl_variant!(Vec4, Vec4);
impl_variant!(Quat, Quat);
impl_variant!(Color, Color);
impl_variant!(Color32, Color32);

impl<T: Any> ToVariant for Pointer<T> {
  fn to_variant(&self) -> Variant {
    Variant::UserData(self.clone())
  }
}

impl<T: Any> FromVariant for Pointer<T> {
  fn from_variant(variant: Variant) -> Result<Self, VariantError> {
    match variant {
      Variant::UserData(value) => Ok(unsafe { value.cast_unchecked::<T>() }),
      _ => Err(VariantError::InvalidConversion),
    }
  }
}

#[macro_export]
macro_rules! impl_variant_enum {
  ($type:ty, u32) => {
    /// Allow conversion of enum to/from a variant.
    impl $crate::ToVariant for $type {
      fn to_variant(&self) -> $crate::Variant {
        $crate::Variant::U32(*self as u32)
      }
    }

    /// Allow conversion of enum to/from a variant.
    impl $crate::FromVariant for $type {
      fn from_variant(variant: $crate::Variant) -> Result<Self, $crate::VariantError> {
        Ok(unsafe { std::mem::transmute(u32::from_variant(variant)?) })
      }
    }
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_variant_size_is_ok() {
    println!("Variant size is {}", size_of::<Variant>());

    assert!(size_of::<Variant>() <= 32);
  }

  #[test]
  fn test_variant_conversion() {
    assert_eq!(true.to_variant(), Variant::Bool(true));
    assert_eq!(u8::from_variant(Variant::U8(10)).unwrap(), 10);
  }

  #[test]
  fn test_variant_coercion() {
    assert_eq!(u16::from_variant(Variant::U8(10)).unwrap(), 10);
    assert_eq!(u32::from_variant(Variant::U8(10)).unwrap(), 10);
    assert_eq!(f64::from_variant(Variant::U8(10)).unwrap(), 10.0f64);
  }

  #[test]
  fn test_variant_arithmetic() {
    let a = Variant::U32(30);
    let b = Variant::U32(20);

    assert_eq!((a.clone() + b.clone()).unwrap(), Variant::U32(50));
    assert_eq!((a.clone() - b.clone()).unwrap(), Variant::U32(10));
    assert_eq!((a.clone() * b.clone()).unwrap(), Variant::U32(600));
    assert_eq!((a.clone() / b.clone()).unwrap(), Variant::U32(1));
    assert_eq!((a.clone() % b.clone()).unwrap(), Variant::U32(10));
  }

  #[test]
  fn test_variant_user_data_equality() {
    let value = Pointer::new("Hello, World!");

    let a = Variant::UserData(value.clone());
    let b = Variant::UserData(value.clone());

    assert_eq!(a, b);

    value.delete();
  }
}
