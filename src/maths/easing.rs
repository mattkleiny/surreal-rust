//! Interpolation and easing.

use crate::graphics::Color;

/// Allows interpolation of arbitrary values.
pub trait Lerp<T> {
  fn lerp(from: T, to: T, amount: f64) -> Self;
}

impl<T: Copy + Sized + Into<f64> + From<f64>> Lerp<T> for T {
  #[inline]
  fn lerp(a: T, b: T, t: f64) -> Self {
    let a: f64 = a.into();
    let b: f64 = b.into();

    (a + t * (b - a)).into()
  }
}

impl Lerp<Color> for Color {
  #[inline]
  fn lerp(_a: Color, _b: Color, _t: f64) -> Self {
    unimplemented!()
  }
}
