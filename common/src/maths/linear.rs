//! Mathematical utilities for linear algebra.

pub use aabb::*;
pub use glam::*;
pub use planes::*;
pub use rays::*;
pub use sdf::*;

use super::*;

mod aabb;
mod planes;
mod rays;
mod sdf;

/// A frustum in 3-space.
#[derive(Clone, Debug)]
pub struct Frustum {
  pub near: Plane,
  pub far: Plane,
  pub left: Plane,
  pub right: Plane,
  pub top: Plane,
  pub bottom: Plane,
}

impl Frustum {
  /// Determines whether the given point is contained within this frustum.
  pub fn contains_point(&self, point: Vec3) -> bool {
    self.near.distance_to_point(point) >= 0.0
      && self.far.distance_to_point(point) >= 0.0
      && self.left.distance_to_point(point) >= 0.0
      && self.right.distance_to_point(point) >= 0.0
      && self.top.distance_to_point(point) >= 0.0
      && self.bottom.distance_to_point(point) >= 0.0
  }

  /// Determines whether the given sphere is contained within this frustum.
  pub fn contains_sphere(&self, sphere: Sphere) -> bool {
    self.contains_point(sphere.center + vec3(sphere.radius, 0.0, 0.0))
      && self.contains_point(sphere.center + vec3(-sphere.radius, 0.0, 0.0))
      && self.contains_point(sphere.center + vec3(0.0, sphere.radius, 0.0))
      && self.contains_point(sphere.center + vec3(0.0, -sphere.radius, 0.0))
      && self.contains_point(sphere.center + vec3(0.0, 0.0, sphere.radius))
      && self.contains_point(sphere.center + vec3(0.0, 0.0, -sphere.radius))
  }

  /// Determines whether the given AABB is contained within this frustum.
  pub fn contains_aabb(&self, aabb: AABB) -> bool {
    self.contains_point(aabb.min)
      && self.contains_point(aabb.max)
      && self.contains_point(vec3(aabb.min.x, aabb.min.y, aabb.max.z))
      && self.contains_point(vec3(aabb.min.x, aabb.max.y, aabb.min.z))
      && self.contains_point(vec3(aabb.min.x, aabb.max.y, aabb.max.z))
      && self.contains_point(vec3(aabb.max.x, aabb.min.y, aabb.min.z))
      && self.contains_point(vec3(aabb.max.x, aabb.min.y, aabb.max.z))
      && self.contains_point(vec3(aabb.max.x, aabb.max.y, aabb.min.z))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_contains_point() {
    let frustum = Frustum {
      near: Plane::new(Vec3::new(0.0, 0.0, 1.0), 0.0),
      far: Plane::new(Vec3::new(0.0, 0.0, -1.0), 0.0),
      left: Plane::new(Vec3::new(1.0, 0.0, 0.0), 0.0),
      right: Plane::new(Vec3::new(-1.0, 0.0, 0.0), 0.0),
      top: Plane::new(Vec3::new(0.0, -1.0, 0.0), 0.0),
      bottom: Plane::new(Vec3::new(0.0, 1.0, 0.0), 0.0),
    };

    assert!(frustum.contains_point(Vec3::new(0.0, 0.0, 0.0)));
    assert!(frustum.contains_point(Vec3::new(1.0, 0.0, 0.0)));
    assert!(frustum.contains_point(Vec3::new(0.0, 1.0, 0.0)));
    assert!(frustum.contains_point(Vec3::new(0.0, 0.0, 1.0)));
    assert!(!frustum.contains_point(Vec3::new(2.0, 0.0, 0.0)));
    assert!(!frustum.contains_point(Vec3::new(0.0, 2.0, 0.0)));
    assert!(!frustum.contains_point(Vec3::new(0.0, 0.0, -1.0)));
  }

  #[test]
  fn test_contains_sphere() {
    let frustum = Frustum {
      near: Plane::new(Vec3::new(0.0, 0.0, 1.0), 0.0),
      far: Plane::new(Vec3::new(0.0, 0.0, -1.0), 0.0),
      left: Plane::new(Vec3::new(1.0, 0.0, 0.0), 0.0),
      right: Plane::new(Vec3::new(-1.0, 0.0, 0.0), 0.0),
      top: Plane::new(Vec3::new(0.0, -1.0, 0.0), 0.0),
      bottom: Plane::new(Vec3::new(0.0, 1.0, 0.0), 0.0),
    };

    let sphere_inside = Sphere {
      center: Vec3::new(0.0, 0.0, 0.0),
      radius: 1.0,
    };

    let sphere_outside = Sphere {
      center: Vec3::new(2.0, 0.0, 0.0),
      radius: 1.0,
    };

    assert!(frustum.contains_sphere(sphere_inside));
    assert!(!frustum.contains_sphere(sphere_outside));
  }

  #[test]
  fn test_contains_aabb() {
    let frustum = Frustum {
      near: Plane::new(Vec3::new(0.0, 0.0, 1.0), 0.0),
      far: Plane::new(Vec3::new(0.0, 0.0, -1.0), 0.0),
      left: Plane::new(Vec3::new(1.0, 0.0, 0.0), 0.0),
      right: Plane::new(Vec3::new(-1.0, 0.0, 0.0), 0.0),
      top: Plane::new(Vec3::new(0.0, -1.0, 0.0), 0.0),
      bottom: Plane::new(Vec3::new(0.0, 1.0, 0.0), 0.0),
    };

    let aabb_inside = AABB {
      min: Vec3::new(-0.5, -0.5, -0.5),
      max: Vec3::new(0.5, 0.5, 0.5),
    };

    let aabb_outside = AABB {
      min: Vec3::new(1.0, 1.0, 1.0),
      max: Vec3::new(2.0, 2.0, 2.0),
    };

    assert!(frustum.contains_aabb(aabb_inside));
    assert!(!frustum.contains_aabb(aabb_outside));
  }
}
