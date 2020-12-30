use super::*;

#[derive(Copy, Clone, Debug)]
pub struct Quaternion<T> {
  pub x: T,
  pub y: T,
  pub z: T,
  pub w: T,
}