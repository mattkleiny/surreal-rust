use super::*;

/// Allows interpolation of arbitrary values.
pub trait Lerp {
  fn lerp(a: Self, b: Self, t: f32) -> Self;
}

impl<T> Lerp for T where T: Numeric {
  fn lerp(a: Self, b: Self, t: f32) -> T {
    let a = a.to_f32();
    let b = b.to_f32();

    T::from_f32(a + t * (b - a))
  }
}
