use super::*;

macro_rules! impl_ray {
  ($type:ty, $vec:ty, $scalar:ty, $helper:ident) => {
    /// Shorthand to construct a [`$type`]
    #[inline(always)]
    pub const fn helper(origin: $vec, direction: $vec) -> $type {
      $type::new(origin, direction)
    }

    /// Represents a ray.
    #[derive(Copy, Clone, Debug, PartialEq)]
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

      /// Calculates the reflection of the ray
      #[inline(always)]
      pub fn reflect(&self, normal: $vec) -> Self {
        Self::new(self.origin, self.direction.reflect(normal))
      }

      /// Calculates the refraction of the ray
      #[inline(always)]
      pub fn refract(&self, normal: $vec, eta: $scalar) -> Self {
        Self::new(self.origin, self.direction.refract(normal, eta))
      }

      /// Calculate the point on the ray at a given distance
      #[inline(always)]
      pub fn point_at(&self, distance: $vec) -> $vec {
        self.origin + self.direction * distance
      }

      /// Calculate the distance from the ray origin to the point on the ray at a given distance
      #[inline(always)]
      pub fn distance_at(&self, distance: $vec) -> $scalar {
        self.direction.magnitude() * distance
      }

      /// Calculate the distance from the ray origin to the given point
      #[inline(always)]
      pub fn distance_to(&self, point: $vec) -> $scalar {
        (point - self.origin).magnitude()
      }

      /// Calculate the distance from the ray origin to the given point along the ray
      #[inline(always)]
      pub fn distance_along(&self, point: $vec) -> $scalar {
        (point - self.origin).dot(self.direction) / self.direction.magnitude()
      }

      /// Calculate the point on the ray at the given distance along the ray
      #[inline(always)]
      pub fn point_along(&self, distance: $vec) -> $vec {
        self.origin + self.direction * (distance / self.direction.magnitude())
      }
    }
  };
}

impl_ray(Ray2, Vec2, f32, ray2);
impl_ray(DRay2, DVec2, f64, dray2);
impl_ray(Ray3, Vec3, f32, ray3);
impl_ray(DRay3, DVec3, f64, dray3);

// TODO: implement tests for these guys
