use crate::{FromVariant, ToVariant, Variant, VariantError};

/// A packed 'enum' value, that can be converted to/from some enum type.
///
/// This is used to store enum values in a generic way, without knowing the
/// specific enum type; it can be converted to/from the enum type when needed.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackedEnum {
  pub kind: u16,
  pub ordinal: u32,
}

impl PackedEnum {
  // the first 2 bits indicate an enum
  // the next 14 bits represent the kind
  // the last 48 bits represent the ordinal
  //
  // bit pattern looks like this:
  // 0b11XXXXXX_XXXXYYYY_YYYYYYYY_YYYYYYYY

  /// Creates a new packed enum value.
  pub fn new(kind: u16, ordinal: u32) -> Self {
    Self { kind, ordinal }
  }

  /// Converts this packed enum value into a 64-bit integer.
  pub fn to_i64(&self) -> i64 {
    let mask = 0b11 << 62;

    let kind = (self.kind as i64) << 48;
    let ordinal = self.ordinal as i64;

    mask | kind | ordinal
  }

  /// Tries to convert a 64-bit integer into a packed enum value.
  pub fn try_from_i64(value: i64) -> Option<Self> {
    let mask = 0b11 << 62;

    if (value & mask) != mask {
      return None;
    }

    let kind = ((value >> 48) & 0x3FFF) as u16;
    let ordinal = (value & 0xFFFFFFFF) as u32;

    Some(Self { kind, ordinal })
  }
}

/// Represents a type that can be packed into a [`PackedEnum`].
pub trait ToPackedEnum: Sized {
  /// Converts this value to a [`PackedEnum`].
  fn to_packed_enum(&self) -> PackedEnum;
}

/// Represents a type that can be unpacked from a [`PackedEnum`].
pub trait FromPackedEnum: Sized {
  /// Converts this value from a [`PackedEnum`].
  fn from_packed_enum(value: PackedEnum) -> Option<Self>;
}

impl<E: ToPackedEnum> ToVariant for E {
  #[inline]
  fn to_variant(&self) -> Variant {
    Variant::Enum(self.to_packed_enum())
  }
}

impl<E: FromPackedEnum> FromVariant for E {
  #[inline]
  fn from_variant(variant: Variant) -> Result<Self, VariantError> {
    match variant {
      Variant::Enum(value) => Self::from_packed_enum(value).ok_or(VariantError::InvalidConversion),
      _ => Err(VariantError::InvalidConversion),
    }
  }
}

/// Represents an enum that can be packed/unpacked by simple transmutation.
pub unsafe trait FastPackedEnum: Sized {
  const KIND: u16;

  /// Converts this enum to u32.
  #[inline(always)]
  fn to_u32(&self) -> u32 {
    unsafe { std::mem::transmute_copy(self) }
  }

  /// Converts this u32 to an enum.
  #[inline(always)]
  fn from_u32(value: u32) -> Self {
    unsafe { std::mem::transmute_copy(&value) }
  }
}

impl<T: FastPackedEnum> ToPackedEnum for T {
  fn to_packed_enum(&self) -> PackedEnum {
    let kind = Self::KIND;
    let ordinal = self.to_u32();

    PackedEnum::new(kind, ordinal)
  }
}

impl<T: FastPackedEnum> FromPackedEnum for T {
  fn from_packed_enum(value: PackedEnum) -> Option<Self> {
    if value.kind != Self::KIND {
      return None;
    }

    Some(Self::from_u32(value.ordinal))
  }
}
