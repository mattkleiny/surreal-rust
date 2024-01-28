use super::Scalar;

/// Allows linear interpolation of arbitrary values.
pub trait Lerp {
  fn lerp(a: Self, b: Self, t: f32) -> Self;
}

impl<T: Scalar> Lerp for T {
  #[inline(always)]
  fn lerp(a: Self, b: Self, t: f32) -> T {
    let a = a.to_f32();
    let b = b.to_f32();

    T::from_f32(a + t * (b - a))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_lerp_f32() {
    let a = 0.0;
    let b = 1.0;
    let t = 0.5;

    let result = f32::lerp(a, b, t);

    assert_eq!(result, 0.5);
  }

  #[test]
  fn test_lerp_u32() {
    let a = 0;
    let b = 10;
    let t = 0.3;

    let result = u32::lerp(a, b, t);

    assert_eq!(result, 3);
  }
}
