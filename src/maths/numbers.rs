use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Represents a numeric type that allows standard equality and arithmetic.
pub trait Numeric:
  Copy
  + Default
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
{
  const ZERO: Self;
  const ONE: Self;

  /// Converts a value from a 32-bit floating point number.
  fn from_f32(value: f32) -> Self;

  /// Converts this numeric type to a 32-bit floating point number.
  fn to_f32(self) -> f32;

  /// Clamps this value in the given range.
  fn clamp(self, lower: Self, upper: Self) -> Self;
}

/// Implements the numeric traits for standard purpose a numeric type.
macro_rules! implement_numeric {
  ($type:ty) => {
    impl Numeric for $type {
      const ZERO: Self = 0 as Self;
      const ONE: Self = 1 as Self;

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

implement_numeric!(u8);
implement_numeric!(u16);
implement_numeric!(u32);
implement_numeric!(u64);
implement_numeric!(u128);
implement_numeric!(usize);

implement_numeric!(i8);
implement_numeric!(i16);
implement_numeric!(i32);
implement_numeric!(i64);
implement_numeric!(i128);
implement_numeric!(isize);

implement_numeric!(f32);
implement_numeric!(f64);
