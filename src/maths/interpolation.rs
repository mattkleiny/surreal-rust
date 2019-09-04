use num_traits::{AsPrimitive, FromPrimitive};

/// Allows interpolation of arbitrary values.
pub trait Lerp {
  fn lerp(from: Self, to: Self, amount: f32) -> Self;
}

/// Generic implementation of interpolation for all the primitive types.
impl<T> Lerp for T where T: AsPrimitive<f32> + FromPrimitive {
  #[inline]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    let a: f32 = a.as_();
    let b: f32 = b.as_();

    Self::from_f32(a + t * (b - a)).unwrap()
  }
}