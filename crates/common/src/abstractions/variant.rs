use crate::{Color, Color32, Quat, StringName, Vec2, Vec3, Vec4};

/// Allows for a type to be converted to a [`Variant`].
pub trait ToVariant {
  /// Converts the type into a [`Variant`].
  fn to_variant(&self) -> Variant;
}

/// Allows for a type to be converted from a [`Variant`].
pub trait FromVariant {
  /// Converts a [`Variant`] into the type.
  fn from_variant(variant: Variant) -> Self;
}

/// An error that can occur when working with variants.
#[derive(Debug)]
pub enum VariantError {
  InvalidNegation,
  NonArithmetic,
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
}

/// A type that can hold varying different values.
///
/// This is an abstraction over the different primitive types that are often
/// shuffled around in the engine. It allows for a more generic API that can
/// handle any type of value.
#[derive(Default, Clone, Debug, PartialEq)]
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

  /// Negates the value of the variant, if possible.
  pub fn negate(&self) -> Result<Self, VariantError> {
    match self {
      Variant::Bool(value) => Ok(Variant::Bool(!value)),
      Variant::U8(value) => Ok(Variant::U8(!value)),
      Variant::U16(value) => Ok(Variant::U16(!value)),
      Variant::U32(value) => Ok(Variant::U32(!value)),
      Variant::U64(value) => Ok(Variant::U64(!value)),
      Variant::I8(value) => Ok(Variant::I8(!value)),
      Variant::I16(value) => Ok(Variant::I16(!value)),
      Variant::I32(value) => Ok(Variant::I32(!value)),
      Variant::I64(value) => Ok(Variant::I64(!value)),
      _ => Err(VariantError::InvalidNegation),
    }
  }

  /// Attempts to add two variants together.
  pub fn add(&self, rhs: &Self) -> Result<Self, VariantError> {
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
      (Variant::Vec2(a), Variant::Vec2(b)) => Ok(Variant::Vec2(*a + *b)),
      (Variant::Vec3(a), Variant::Vec3(b)) => Ok(Variant::Vec3(*a + *b)),
      (Variant::Vec4(a), Variant::Vec4(b)) => Ok(Variant::Vec4(*a + *b)),
      (Variant::Quat(a), Variant::Quat(b)) => Ok(Variant::Quat(*a + *b)),
      (Variant::Color(a), Variant::Color(b)) => Ok(Variant::Color(*a + *b)),
      (Variant::Color32(a), Variant::Color32(b)) => Ok(Variant::Color32(*a + *b)),
      _ => Err(VariantError::NonArithmetic),
    }
  }

  /// Attempts to subtract two variants.
  pub fn sub(&self, rhs: &Self) -> Result<Self, VariantError> {
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
      (Variant::Vec2(a), Variant::Vec2(b)) => Ok(Variant::Vec2(*a - *b)),
      (Variant::Vec3(a), Variant::Vec3(b)) => Ok(Variant::Vec3(*a - *b)),
      (Variant::Vec4(a), Variant::Vec4(b)) => Ok(Variant::Vec4(*a - *b)),
      (Variant::Quat(a), Variant::Quat(b)) => Ok(Variant::Quat(*a - *b)),
      (Variant::Color(a), Variant::Color(b)) => Ok(Variant::Color(*a - *b)),
      (Variant::Color32(a), Variant::Color32(b)) => Ok(Variant::Color32(*a - *b)),
      _ => Err(VariantError::NonArithmetic),
    }
  }

  /// Attempts to multiply two variants.
  pub fn mul(&self, rhs: &Self) -> Result<Self, VariantError> {
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
      (Variant::Vec2(a), Variant::Vec2(b)) => Ok(Variant::Vec2(*a * *b)),
      (Variant::Vec3(a), Variant::Vec3(b)) => Ok(Variant::Vec3(*a * *b)),
      (Variant::Vec4(a), Variant::Vec4(b)) => Ok(Variant::Vec4(*a * *b)),
      (Variant::Quat(a), Variant::Quat(b)) => Ok(Variant::Quat(*a * *b)),
      (Variant::Color(a), Variant::Color(b)) => Ok(Variant::Color(*a * *b)),
      (Variant::Color32(a), Variant::Color32(b)) => Ok(Variant::Color32(*a * *b)),
      _ => Err(VariantError::NonArithmetic),
    }
  }

  /// Attempts to divide two variants.
  pub fn div(&self, rhs: &Self) -> Result<Self, VariantError> {
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
      (Variant::Vec2(a), Variant::Vec2(b)) => Ok(Variant::Vec2(*a / *b)),
      (Variant::Vec3(a), Variant::Vec3(b)) => Ok(Variant::Vec3(*a / *b)),
      (Variant::Vec4(a), Variant::Vec4(b)) => Ok(Variant::Vec4(*a / *b)),
      (Variant::Color(a), Variant::Color(b)) => Ok(Variant::Color(*a / *b)),
      (Variant::Color32(a), Variant::Color32(b)) => Ok(Variant::Color32(*a / *b)),
      _ => Err(VariantError::NonArithmetic),
    }
  }
}

/// Allows for a type to be converted to a [`Variant`].
impl<T: Into<Variant> + Clone> ToVariant for T {
  #[inline]
  fn to_variant(&self) -> Variant {
    self.clone().into()
  }
}

/// Allows for a type to be converted from a [`Variant`].
impl<T: From<Variant>> FromVariant for T {
  #[inline]
  fn from_variant(variant: Variant) -> Self {
    Self::from(variant)
  }
}

/// Allows conversion to/from a `Variant` for a given type.
macro_rules! impl_variant {
  ((), $kind:ident) => {
    impl From<()> for Variant {
      #[inline]
      fn from(_: ()) -> Self {
        Variant::$kind
      }
    }

    impl From<Variant> for () {
      #[inline]
      fn from(_: Variant) -> Self {
        // no-op
      }
    }
  };

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

  ($type:ty, $kind:ident, $($kinds:ident),*) => {
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
          $(Variant::$kinds(value) => value as $type,)*
          _ => panic!("Invalid type conversion"),
        }
      }
    }
  }
}

impl_variant!((), Null);
impl_variant!(bool, Bool);
impl_variant!(char, Char);
impl_variant!(u8, U8);
impl_variant!(u16, U16, U8, I8);
impl_variant!(u32, U32, U8, U16, I8, I16, F32);
impl_variant!(u64, U64, U8, U16, U32, I8, I16, I32, F32, F64);
impl_variant!(i8, I8);
impl_variant!(i16, I16, I8);
impl_variant!(i32, I32, I8, I16, U8, U16, U32, F32);
impl_variant!(i64, I64, I8, I16, I32, U8, U16, U32, F32, F64);
impl_variant!(f32, F32, I8, I16, I32, I64, U8, U16, U32, U64);
impl_variant!(f64, F64, I8, I16, I32, I64, U8, U16, U32, U64);
impl_variant!(String, String);
impl_variant!(StringName, StringName);
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
  fn test_variant_conversion() {
    assert_eq!(true.to_variant(), Variant::Bool(true));
    assert_eq!(u8::from_variant(Variant::U8(10)), 10);
  }

  #[test]
  fn test_variant_coercion() {
    assert_eq!(u16::from_variant(Variant::U8(10)), 10);
    assert_eq!(u32::from_variant(Variant::U8(10)), 10);
    assert_eq!(f64::from_variant(Variant::U8(10)), 10.0f64);
  }

  #[test]
  fn test_variant_arithmetic() {
    let a = Variant::U32(30);
    let b = Variant::U32(20);

    assert_eq!(a.add(&b).unwrap(), Variant::U32(50));
    assert_eq!(a.sub(&b).unwrap(), Variant::U32(10));
    assert_eq!(a.mul(&b).unwrap(), Variant::U32(600));
    assert_eq!(a.div(&b).unwrap(), Variant::U32(1));
  }
}
