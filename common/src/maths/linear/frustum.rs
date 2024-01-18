use super::*;

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
  /// Creates a new frustum from an array of planes.
  pub fn from_array(planes: [Plane; 6]) -> Self {
    Self {
      near: planes[0],
      far: planes[1],
      left: planes[2],
      right: planes[3],
      top: planes[4],
      bottom: planes[5],
    }
  }

  /// Creates a new frustum from a slice of planes.
  pub fn from_slice(planes: &[Plane; 6]) -> Self {
    Self {
      near: planes[0],
      far: planes[1],
      left: planes[2],
      right: planes[3],
      top: planes[4],
      bottom: planes[5],
    }
  }

  /// Creates a new frustum from a projection-view matrix.
  pub fn from_projection_view(projection_view: Mat4) -> Self {
    Frustum {
      near: Plane::from_vector4(projection_view.row(2) + projection_view.row(3)),
      far: Plane::from_vector4(projection_view.row(3) - projection_view.row(2)),
      left: Plane::from_vector4(projection_view.row(3) + projection_view.row(0)),
      right: Plane::from_vector4(projection_view.row(3) - projection_view.row(0)),
      top: Plane::from_vector4(projection_view.row(3) - projection_view.row(1)),
      bottom: Plane::from_vector4(projection_view.row(3) + projection_view.row(1)),
    }
  }

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

  /// Converts this frustum to an array of planes.
  pub fn into_array(self) -> [Plane; 6] {
    [self.near, self.far, self.left, self.right, self.top, self.bottom]
  }

  /// Converts this frustum to an array of planes.
  pub fn to_array(&self) -> [Plane; 6] {
    [self.near, self.far, self.left, self.right, self.top, self.bottom]
  }

  /// Converts this frustum to a slice of planes.
  pub fn as_slice(&self) -> &[Plane; 6] {
    unsafe { &*(self as *const Self as *const [Plane; 6]) }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_contains_point() {
    let frustum = Frustum {
      near: Plane::new(vec3(0.0, 0.0, 1.0), 0.0),
      far: Plane::new(vec3(0.0, 0.0, -1.0), 0.0),
      left: Plane::new(vec3(1.0, 0.0, 0.0), 0.0),
      right: Plane::new(vec3(-1.0, 0.0, 0.0), 0.0),
      top: Plane::new(vec3(0.0, -1.0, 0.0), 0.0),
      bottom: Plane::new(vec3(0.0, 1.0, 0.0), 0.0),
    };

    assert!(frustum.contains_point(vec3(0.0, 0.0, 0.0)));
    assert!(frustum.contains_point(vec3(0.0, 0.0, 0.1)));
    assert!(frustum.contains_point(vec3(0.0, 0.1, 0.0)));
    assert!(frustum.contains_point(vec3(0.1, 0.0, 0.0)));
    assert!(!frustum.contains_point(vec3(2.0, 0.0, 0.0)));
    assert!(!frustum.contains_point(vec3(0.0, 2.0, 0.0)));
    assert!(!frustum.contains_point(vec3(0.0, 0.0, -1.0)));
  }

  #[test]
  fn test_contains_sphere() {
    let frustum = Frustum {
      near: Plane::new(vec3(0.0, 0.0, 1.0), 0.0),
      far: Plane::new(vec3(0.0, 0.0, -1.0), 0.0),
      left: Plane::new(vec3(1.0, 0.0, 0.0), 0.0),
      right: Plane::new(vec3(-1.0, 0.0, 0.0), 0.0),
      top: Plane::new(vec3(0.0, -1.0, 0.0), 0.0),
      bottom: Plane::new(vec3(0.0, 1.0, 0.0), 0.0),
    };

    let sphere_inside = Sphere {
      center: vec3(0.0, 0.0, 0.0),
      radius: 1.0,
    };

    let sphere_outside = Sphere {
      center: vec3(2.0, 0.0, 0.0),
      radius: 1.0,
    };

    assert!(frustum.contains_sphere(sphere_inside));
    assert!(!frustum.contains_sphere(sphere_outside));
  }

  #[test]
  fn test_contains_aabb() {
    let frustum = Frustum {
      near: Plane::new(vec3(0.0, 0.0, 1.0), 0.0),
      far: Plane::new(vec3(0.0, 0.0, -1.0), 0.0),
      left: Plane::new(vec3(1.0, 0.0, 0.0), 0.0),
      right: Plane::new(vec3(-1.0, 0.0, 0.0), 0.0),
      top: Plane::new(vec3(0.0, -1.0, 0.0), 0.0),
      bottom: Plane::new(vec3(0.0, 1.0, 0.0), 0.0),
    };

    let aabb_inside = AABB {
      min: vec3(-0.5, -0.5, -0.5),
      max: vec3(0.5, 0.5, 0.5),
    };

    let aabb_outside = AABB {
      min: vec3(1.0, 1.0, 1.0),
      max: vec3(2.0, 2.0, 2.0),
    };

    assert!(frustum.contains_aabb(aabb_inside));
    assert!(!frustum.contains_aabb(aabb_outside));
  }
}
