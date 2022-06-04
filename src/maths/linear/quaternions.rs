use super::*;

/// A standard purpose quaternion.
#[derive(Copy, Clone, Default, Debug)]
pub struct Quaternion<T> {
  pub x: T,
  pub y: T,
  pub z: T,
  pub w: T,
}

impl<T> Quaternion<T> {
  /// Constructs a new quaternion.
  pub const fn new(x: T, y: T, z: T, w: T) -> Self {
    Self { x, y, z, w }
  }
}

impl<T> Lerp for Quaternion<T>
where T: Numeric
{
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    Self::new(
      T::lerp(a.x, b.x, t),
      T::lerp(a.y, b.y, t),
      T::lerp(a.z, b.z, t),
      T::lerp(a.w, b.w, t),
    )
  }
}
