use std::ops::{Add, Div, Mul, Sub};

/// Represents a numeric type that allows standard equality and arithmetic.
pub trait Numeric: Copy + PartialOrd + PartialEq + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> {
  const ZERO: Self;
  const ONE: Self;

  /// Converts a value from a 32-bit floating point number.
  fn from_f32(value: f32) -> Self;

  /// Converts this numeric type to a 32-bit floating point number.
  fn to_f32(self) -> f32;
}

/// Implements the numeric traits for standard purpose a numeric type.
macro implement_numeric($type:ty) {
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
}
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