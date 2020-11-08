/// Allows interpolation of arbitrary values.
pub trait Lerp {
  fn lerp(a: &Self, b: &Self, t: f32) -> Self;
}

impl Lerp for f32 {
  #[inline]
  fn lerp(a: &Self, b: &Self, t: f32) -> Self {
    a + t * (b - a)
  }
}

impl Lerp for f64 {
  #[inline]
  fn lerp(a: &Self, b: &Self, t: f32) -> Self {
    let a = *a as f32;
    let b = *b as f32;

    (a + t * (b - a)) as f64
  }
}

impl Lerp for u8 {
  #[inline]
  fn lerp(a: &Self, b: &Self, t: f32) -> Self {
    let a = *a as f32;
    let b = *b as f32;

    (a + t * (b - a)) as u8
  }
}

impl Lerp for u32 {
  #[inline]
  fn lerp(a: &Self, b: &Self, t: f32) -> Self {
    let a = *a as f32;
    let b = *b as f32;

    (a + t * (b - a)) as u32
  }
}

impl Lerp for i32 {
  #[inline]
  fn lerp(a: &Self, b: &Self, t: f32) -> Self {
    let a = *a as f32;
    let b = *b as f32;

    (a + t * (b - a)) as i32
  }
}

impl Lerp for u64 {
  #[inline]
  fn lerp(a: &Self, b: &Self, t: f32) -> Self {
    let a = *a as f32;
    let b = *b as f32;

    (a + t * (b - a)) as u64
  }
}

impl Lerp for i64 {
  #[inline]
  fn lerp(a: &Self, b: &Self, t: f32) -> Self {
    let a = *a as f32;
    let b = *b as f32;

    (a + t * (b - a)) as i64
  }
}

// Allows in-place blending between values
pub trait Blend {
  fn blend(&mut self, to: &Self, amount: f32);
}

impl<T: Lerp> Blend for T {
  fn blend(&mut self, to: &Self, amount: f32) {
    *self = Self::lerp(self, to, amount);
  }
}
