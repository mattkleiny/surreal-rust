use super::*;

/// An axially-aligned bounding box.
#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct AABB {
  pub min: Vec3,
  pub max: Vec3,
}

impl AABB {
  /// Creates a new AABB from the given min and max points.
  pub fn from_min_max(min: Vec3, max: Vec3) -> Self {
    AABB { min, max }
  }

  /// Creates a new AABB that encapsulates the given set of points.
  pub fn from_points(points: &[Vec3]) -> Self {
    let mut min = Vec3::splat(f32::MAX);
    let mut max = Vec3::splat(f32::MIN);

    for point in points {
      min = min.min(*point);
      max = max.max(*point);
    }

    Self::from_min_max(min, max)
  }

  /// Retrieves the nth corner of the AABB.
  ///
  /// This method will panic if the index is out of bounds.
  pub fn corner(&self, index: usize) -> Vec3 {
    debug_assert!(index < 8);

    let min = self.min;
    let max = self.max;

    match index {
      0 => vec3(min.x, min.y, min.z),
      1 => vec3(min.x, min.y, max.z),
      2 => vec3(min.x, max.y, min.z),
      3 => vec3(min.x, max.y, max.z),
      4 => vec3(max.x, min.y, min.z),
      5 => vec3(max.x, min.y, max.z),
      6 => vec3(max.x, max.y, min.z),
      7 => vec3(max.x, max.y, max.z),
      _ => panic!("Invalid corner index"),
    }
  }

  /// Determines if the AABB contains the given point.
  pub fn contains(&self, point: Vec3) -> bool {
    let min = self.min;
    let max = self.max;

    point.x >= min.x && point.x <= max.x && point.y >= min.y && point.y <= max.y && point.z >= min.z && point.z <= max.z
  }

  /// Determines if the AABB intersects the given other [`AABB`].
  pub fn intersects(&self, other: &Self) -> bool {
    let min = self.min;
    let max = self.max;

    let other_min = other.min;
    let other_max = other.max;

    other_min.x <= max.x
      && other_max.x >= min.x
      && other_min.y <= max.y
      && other_max.y >= min.y
      && other_min.z <= max.z
      && other_max.z >= min.z
  }

  /// Builds a new AABB that is union with the given other [`AABB`].
  pub fn union(&self, other: &Self) -> Self {
    let min = self.min.min(other.min);
    let max = self.max.max(other.max);

    Self::from_min_max(min, max)
  }

  /// Transforms all points in the [`AABB`] by the given matrix.
  pub fn transform(&self, transform: &Mat4) -> Self {
    let min = self.min;
    let max = self.max;

    let corners = [
      vec3(min.x, min.y, min.z),
      vec3(min.x, min.y, max.z),
      vec3(min.x, max.y, min.z),
      vec3(min.x, max.y, max.z),
      vec3(max.x, min.y, min.z),
      vec3(max.x, min.y, max.z),
      vec3(max.x, max.y, min.z),
      vec3(max.x, max.y, max.z),
    ];

    let mut new_min = Vec3::splat(f32::MAX);
    let mut new_max = Vec3::splat(f32::MIN);

    for corner in corners {
      let transformed = transform.transform_point3(corner);

      new_min = new_min.min(transformed);
      new_max = new_max.max(transformed);
    }

    Self::from_min_max(new_min, new_max)
  }

  /// Converts the AABB into a slice of floats.
  pub fn as_slice(&self) -> &[f32; 6] {
    unsafe { std::mem::transmute(self) }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_from_min_max_points() {
    let aabb = AABB::from_min_max(vec3(-1.0, -1.0, -1.0), vec3(1.0, 1.0, 1.0));

    assert_eq!(aabb.min, vec3(-1.0, -1.0, -1.0));
    assert_eq!(aabb.max, vec3(1.0, 1.0, 1.0));
  }

  #[test]
  fn test_return_correct_corner_points() {
    let aabb = AABB::from_min_max(vec3(-1.0, -1.0, -1.0), vec3(1.0, 1.0, 1.0));

    assert_eq!(aabb.corner(0), vec3(-1.0, -1.0, -1.0));
    assert_eq!(aabb.corner(1), vec3(-1.0, -1.0, 1.0));
    assert_eq!(aabb.corner(2), vec3(-1.0, 1.0, -1.0));
    assert_eq!(aabb.corner(3), vec3(-1.0, 1.0, 1.0));
    assert_eq!(aabb.corner(4), vec3(1.0, -1.0, -1.0));
    assert_eq!(aabb.corner(5), vec3(1.0, -1.0, 1.0));
    assert_eq!(aabb.corner(6), vec3(1.0, 1.0, -1.0));
    assert_eq!(aabb.corner(7), vec3(1.0, 1.0, 1.0));
  }

  #[test]
  fn test_contain_point_inside() {
    let aabb = AABB::from_min_max(vec3(-1.0, -1.0, -1.0), vec3(1.0, 1.0, 1.0));

    assert!(aabb.contains(vec3(0.0, 0.0, 0.0)));
    assert!(aabb.contains(vec3(-0.5, -0.5, -0.5)));
    assert!(aabb.contains(vec3(0.5, 0.5, 0.5)));
  }

  #[test]
  fn test_not_contain_point_outside() {
    let aabb = AABB::from_min_max(vec3(-1.0, -1.0, -1.0), vec3(1.0, 1.0, 1.0));

    assert!(!aabb.contains(vec3(2.0, 2.0, 2.0)));
    assert!(!aabb.contains(vec3(-2.0, -2.0, -2.0)));
    assert!(!aabb.contains(vec3(0.0, 0.0, 2.0)));
  }

  #[test]
  fn test_intersect_with_other_aabb() {
    let aabb1 = AABB::from_min_max(vec3(-1.0, -1.0, -1.0), vec3(1.0, 1.0, 1.0));
    let aabb2 = AABB::from_min_max(vec3(0.0, 0.0, 0.0), vec3(2.0, 2.0, 2.0));
    let aabb3 = AABB::from_min_max(vec3(2.0, 2.0, 2.0), vec3(3.0, 3.0, 3.0));

    assert!(aabb1.intersects(&aabb2));
    assert!(aabb2.intersects(&aabb1));
    assert!(!aabb1.intersects(&aabb3));
    assert!(!aabb3.intersects(&aabb1));
  }

  #[test]
  fn test_union_with_other_aabb() {
    let aabb1 = AABB::from_min_max(vec3(-1.0, -1.0, -1.0), vec3(1.0, 1.0, 1.0));
    let aabb2 = AABB::from_min_max(vec3(0.0, 0.0, 0.0), vec3(2.0, 2.0, 2.0));

    let union_aabb = aabb1.union(&aabb2);

    assert_eq!(union_aabb.min, vec3(-1.0, -1.0, -1.0));
    assert_eq!(union_aabb.max, vec3(2.0, 2.0, 2.0));
  }

  #[test]
  fn test_transform_by_matrix() {
    let aabb = AABB::from_min_max(vec3(-1.0, -1.0, -1.0), vec3(1.0, 1.0, 1.0));
    let transform = Mat4::from_translation(vec3(1.0, 1.0, 1.0));

    let transformed_aabb = aabb.transform(&transform);

    assert_eq!(transformed_aabb.min, vec3(0.0, 0.0, 0.0));
    assert_eq!(transformed_aabb.max, vec3(2.0, 2.0, 2.0));
  }
}
