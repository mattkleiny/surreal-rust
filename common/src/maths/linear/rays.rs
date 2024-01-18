use super::*;

macro_rules! impl_ray {
  ($type:ident, $vec:ty, $scalar:ty, $helper:ident) => {
    /// Shorthand to construct a [`$type`]
    #[inline(always)]
    pub const fn $helper(origin: $vec, direction: $vec) -> $type {
      <$type>::new(origin, direction)
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ray2() {
    let origin = Vec2::new(0.0, 0.0);
    let direction = Vec2::new(1.0, 1.0);
    let ray = Ray2::new(origin, direction);

    // Test point_at
    let point = ray.point_at(2.0);
    assert_eq!(point, Vec2::new(2.0, 2.0));

    // Test distance_to
    let distance = ray.distance_to(Vec2::new(3.0, 3.0));
    assert_eq!(distance, 3.0_f32.sqrt());

    // Test distance_along
    let distance_along = ray.distance_along(Vec2::new(4.0, 4.0));
    assert_eq!(distance_along, 4.0_f32.sqrt());

    // Test point_along
    let point_along = ray.point_along(5.0);
    assert_eq!(point_along, Vec2::new(5.0, 5.0));
  }

  #[test]
  fn test_dray2() {
    let origin = DVec2::new(0.0, 0.0);
    let direction = DVec2::new(1.0, 1.0);
    let ray = DRay2::new(origin, direction);

    // Test point_at
    let point = ray.point_at(2.0);
    assert_eq!(point, DVec2::new(2.0, 2.0));

    // Test distance_to
    let distance = ray.distance_to(DVec2::new(3.0, 3.0));
    assert_eq!(distance, 3.0_f64.sqrt());

    // Test distance_along
    let distance_along = ray.distance_along(DVec2::new(4.0, 4.0));
    assert_eq!(distance_along, 4.0_f64.sqrt());

    // Test point_along
    let point_along = ray.point_along(5.0);
    assert_eq!(point_along, DVec2::new(5.0, 5.0));
  }

  #[test]
  fn test_ray3() {
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let direction = Vec3::new(1.0, 1.0, 1.0);
    let ray = Ray3::new(origin, direction);

    // Test point_at
    let point = ray.point_at(2.0);
    assert_eq!(point, Vec3::new(2.0, 2.0, 2.0));

    // Test distance_to
    let distance = ray.distance_to(Vec3::new(3.0, 3.0, 3.0));
    assert_eq!(distance, 3.0_f32.sqrt());

    // Test distance_along
    let distance_along = ray.distance_along(Vec3::new(4.0, 4.0, 4.0));
    assert_eq!(distance_along, 4.0_f32.sqrt());

    // Test point_along
    let point_along = ray.point_along(5.0);
    assert_eq!(point_along, Vec3::new(5.0, 5.0, 5.0));
  }

  #[test]
  fn test_dray3() {
    let origin = DVec3::new(0.0, 0.0, 0.0);
    let direction = DVec3::new(1.0, 1.0, 1.0);
    let ray = DRay3::new(origin, direction);

    // Test point_at
    let point = ray.point_at(2.0);
    assert_eq!(point, DVec3::new(2.0, 2.0, 2.0));

    // Test distance_to
    let distance = ray.distance_to(DVec3::new(3.0, 3.0, 3.0));
    assert_eq!(distance, 3.0_f64.sqrt());

    // Test distance_along
    let distance_along = ray.distance_along(DVec3::new(4.0, 4.0, 4.0));
    assert_eq!(distance_along, 4.0_f64.sqrt());

    // Test point_along
    let point_along = ray.point_along(5.0);
    assert_eq!(point_along, DVec3::new(5.0, 5.0, 5.0));
  }
}
