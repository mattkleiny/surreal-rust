use super::*;

/// An axially-aligned bounding box.
#[derive(Clone)]
pub struct AABB<T> {
  pub position: Vec3,
  pub size: T,
}
