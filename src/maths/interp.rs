/// Allows interpolation of arbitrary values.
pub trait Lerp {
  fn lerp(a: Self, b: Self, t: f32) -> Self;
}

impl Lerp for f32 {
  #[inline]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    a + t * (b - a)
  }
}

impl Lerp for f64 {
  #[inline]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    let a = a as f32;
    let b = b as f32;

    (a + t * (b - a)) as f64
  }
}

impl Lerp for i32 {
  #[inline]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    let a = a as f32;
    let b = b as f32;

    (a + t * (b - a)) as i32
  }
}

impl Lerp for u32 {
  #[inline]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    let a = a as f32;
    let b = b as f32;

    (a + t * (b - a)) as u32
  }
}

impl Lerp for u8 {
  #[inline]
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    let a = a as f32;
    let b = b as f32;

    (a + t * (b - a)) as u8
  }
}