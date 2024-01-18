use super::*;

/// A field in 3-space.
#[derive(Clone, Debug)]
pub struct Field<V: Vector> {
  pub size: Vec3,
  pub data: Vec<V>,
}
