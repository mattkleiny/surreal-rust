use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::Identity;

/// Represents a scalar type that allows standard equality and arithmetic.
pub trait Scalar:
  Copy
  + Default
  + Identity
  + PartialOrd
  + PartialEq
  + Add<Output = Self>
  + AddAssign
  + Sub<Output = Self>
  + SubAssign
  + Mul<Output = Self>
  + MulAssign
  + Div<Output = Self>
  + DivAssign
  + Sized
{
  /// Converts a value from a 32-bit floating point number.
  fn from_f32(value: f32) -> Self;

  /// Converts this numeric type to a 32-bit floating point number.
  fn to_f32(self) -> f32;

  /// Clamps this value in the given range.
  fn clamp(self, lower: Self, upper: Self) -> Self;
}

/// Implements the numeric traits for standard purpose a numeric type.
macro_rules! impl_scalar {
  ($name:ty) => {
    impl Identity for $name {
      const ZERO: Self = 0 as Self;
      const ONE: Self = 1 as Self;
      const MIN: Self = <$name>::MIN;
      const MAX: Self = <$name>::MAX;
    }

    impl Scalar for $name {
      #[inline(always)]
      fn from_f32(value: f32) -> Self {
        value as Self
      }

      #[inline(always)]
      fn to_f32(self) -> f32 {
        self as f32
      }

      fn clamp(self, lower: Self, upper: Self) -> Self {
        match () {
          _ if self > upper => upper,
          _ if self < lower => lower,
          _ => self,
        }
      }
    }
  };
}

impl_scalar!(u8);
impl_scalar!(u16);
impl_scalar!(u32);
impl_scalar!(u64);
impl_scalar!(u128);
impl_scalar!(usize);

impl_scalar!(i8);
impl_scalar!(i16);
impl_scalar!(i32);
impl_scalar!(i64);
impl_scalar!(i128);
impl_scalar!(isize);

impl_scalar!(f32);
impl_scalar!(f64);
