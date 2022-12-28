use crate::maths::Numeric;

/// Shorthand to construct a [`Quaternion`].
#[inline(always)]
pub const fn quat<T: Numeric>(x: T, y: T, z: T, w: T) -> Quaternion<T> {
  Quaternion::new(x, y, z, w)
}

/// A quaternion.
#[repr(C)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Quaternion<T: Numeric> {
  pub x: T,
  pub y: T,
  pub z: T,
  pub w: T,
}

impl<T: Numeric> Quaternion<T> {
  pub const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
  pub const IDENTITY: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);

  /// Creates a new quaternion from the given components.
  #[inline(always)]
  pub const fn new(x: T, y: T, z: T, w: T) -> Self {
    Self { x, y, z, w }
  }
}
