use super::*;

macro_rules! impl_ray {
  ($type:ident, $vec:ty, $scalar:ty, $helper:ident) => {
    /// Shorthand to construct a [`$type`]
    #[inline(always)]
    pub const fn $helper(origin: $vec, direction: $vec) -> $type {
      <$type>::new(origin, direction)
    }

    /// Represents a ray.
    #[repr(C)]
    #[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
    pub struct $type {
      pub origin: $vec,
      pub direction: $vec,
    }

    impl $type {
      /// Creates a new ray with the given origin and direction
      #[inline(always)]
      pub const fn new(origin: $vec, direction: $vec) -> Self {
        Self { origin, direction }
      }

      /// Calculate the point on the ray at a given distance
      #[inline(always)]
      pub fn point_at(&self, distance: $scalar) -> $vec {
        self.origin + self.direction * distance
      }

      /// Calculate the point on the ray at the given distance along the ray
      #[inline(always)]
      pub fn point_along(&self, distance: $scalar) -> $vec {
        self.origin + self.direction * (distance / self.direction.length())
      }

      /// Calculate the distance from the ray origin to the given point
      #[inline(always)]
      pub fn distance_to(&self, point: $vec) -> $scalar {
        (point - self.origin).length()
      }

      /// Calculate the distance from the origin to the given point along the ray
      #[inline(always)]
      pub fn distance_along(&self, point: $vec) -> $scalar {
        (point - self.origin).dot(self.direction) / self.direction.length()
      }
    }
  };
}

impl_ray!(Ray2, Vec2, f32, ray2);
impl_ray!(DRay2, DVec2, f64, dray2);
impl_ray!(Ray3, Vec3, f32, ray3);
impl_ray!(DRay3, DVec3, f64, dray3);
