use num_traits::{FromPrimitive, Num, ToPrimitive};

/// Allows interpolation of arbitrary values.
pub trait Lerp {
  fn lerp(a: Self, b: Self, t: f32) -> Self;
}

impl<T: Copy + Num + FromPrimitive + ToPrimitive> Lerp for T {
  #[inline]
  fn lerp(a: Self, b: Self, t: f32) -> T {
    let a = a.to_f32().unwrap();
    let b = b.to_f32().unwrap();

    let result = a + t * (b - a);

    T::from_f32(result).unwrap()
  }
}
