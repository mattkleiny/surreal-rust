use crate::maths::{DVec2, DVec3, Polygon, Vec2, Vec3};

/// Computes the convex hull of a set of points using the Graham scan algorithm.
pub trait ConvexHull {
  type Vector;

  /// Computes the convex hull of the given set of points.
  fn convex_hull(&self) -> Vec<Polygon<Self::Vector>>;
}

macro_rules! impl_convex_hull {
  ($type:ty) => {
    impl ConvexHull for [$type] {
      type Vector = $type;

      fn convex_hull(&self) -> Vec<Polygon<Self::Vector>> {
        todo!()
      }
    }
  };
}

impl_convex_hull!(Vec2);
impl_convex_hull!(DVec2);
impl_convex_hull!(Vec3);
impl_convex_hull!(DVec3);
