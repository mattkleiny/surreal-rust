use super::*;

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vector2<T> {
  pub x: T,
  pub y: T,
}

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vector3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vector4<T> {
  pub x: T,
  pub y: T,
  pub z: T,
  pub w: T,
}