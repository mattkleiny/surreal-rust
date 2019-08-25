//! Interpolation and easing.

use crate::graphics::Color;

/// Allows interpolation of arbitrary values.
pub trait Lerp<T> {
  fn lerp(from: T, to: T, amount: f64) -> Self;
}

impl Lerp<u8> for u8 {
  #[inline]
  fn lerp(a: Self, b: Self, t: f64) -> Self {
    let a = a as f64;
    let b = b as f64;

    (a + t * (b - a)) as Self
  }
}

impl Lerp<u32> for u32 {
  #[inline]
  fn lerp(a: Self, b: Self, t: f64) -> Self {
    let a = a as f64;
    let b = b as f64;

    (a + t * (b - a)) as Self
  }
}

impl Lerp<u64> for u64 {
  #[inline]
  fn lerp(a: Self, b: Self, t: f64) -> Self {
    let a = a as f64;
    let b = b as f64;

    (a + t * (b - a)) as Self
  }
}

impl Lerp<f32> for f32 {
  #[inline]
  fn lerp(a: Self, b: Self, t: f64) -> Self {
    let a = a as f64;
    let b = b as f64;

    (a + t * (b - a)) as Self
  }
}

impl Lerp<f64> for f64 {
  #[inline]
  fn lerp(a: Self, b: Self, t: f64) -> Self {
    a + t * (b - a)
  }
}

impl Lerp<Color> for Color {
  #[inline]
  fn lerp(a: Color, b: Color, t: f64) -> Self {
    Color::RGBA(
      u8::lerp(a.r, b.r, t),
      u8::lerp(a.g, b.g, t),
      u8::lerp(a.b, b.b, t),
      u8::lerp(a.a, b.a, t),
    )
  }
}
