use super::*;

/// A field in 3-space.
pub struct Field<V: Vector> {
  pub size: Vec3,
  pub data: Vec<V>,
}
