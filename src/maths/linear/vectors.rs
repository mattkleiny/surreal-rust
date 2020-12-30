#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vector2<T> {
  pub x: T,
  pub y: T,
}

impl<T> Vector2<T> {
  #[inline(always)]
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vector3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T> Vector3<T> {
  #[inline(always)]
  pub const fn new(x: T, y: T, z: T) -> Self {
    Self { x, y, z }
  }
}

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vector4<T> {
  pub x: T,
  pub y: T,
  pub z: T,
  pub w: T,
}

impl<T> Vector4<T> {
  #[inline(always)]
  pub const fn new(x: T, y: T, z: T, w: T) -> Self {
    Self { x, y, z, w }
  }
}