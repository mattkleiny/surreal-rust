use crate::maths::{DVec2, DVec3, Triangle, Vec2, Vec3, Vector};

/// Delaunay triangulation algorithm.
pub trait Delaunay {
  type Vector: Vector;

  /// Triangulates the given set of points.
  fn triangulate(&self) -> Vec<Triangle<Self::Vector>>;
}

/// Implements the Delaunay triangulation algorithm for the given vector type.
macro_rules! impl_delaunay {
  ($type:ty) => {
    impl Delaunay for [$type] {
      type Vector = $type;

      fn triangulate(&self) -> Vec<Triangle<Self::Vector>> {
        todo!()
      }
    }
  };
}

impl_delaunay!(Vec2);
impl_delaunay!(DVec2);
impl_delaunay!(Vec3);
impl_delaunay!(DVec3);
