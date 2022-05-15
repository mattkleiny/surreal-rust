/// A standard purpose quaternion.
#[derive(Copy, Clone, Default, Debug)]
pub struct Quaternion<T> {
  pub x: T,
  pub y: T,
  pub z: T,
  pub w: T,
}