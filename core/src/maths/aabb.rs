use crate::maths::Vector3;

/// An axially-aligned bounding box.
#[derive(Clone)]
pub struct AABB<T> {
  pub position: Vector3<T>,
  pub size: T,
}
