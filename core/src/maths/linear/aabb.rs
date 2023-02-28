use super::*;

// TODO: add some tests for this guy

/// An axially-aligned bounding box.
#[derive(Clone, Default, Debug)]
pub struct AABB {
  pub position: Vec3,
  pub size: f32,
}

impl AABB {
  /// Creates a new AABB from the given min and max points.
  pub fn from_min_max(min: Vec3, max: Vec3) -> Self {
    let position = (min + max) * 0.5;
    let size = (max - min).length() * 0.5;

    Self { position, size }
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

  /// Returns the minimum position of the AABB.
  pub fn min(&self) -> Vec3 {
    self.position - Vec3::splat(self.size)
  }

  /// Returns the maximum position of the AABB.
  pub fn max(&self) -> Vec3 {
    self.position + Vec3::splat(self.size)
  }

  /// Retrieves the nth corner of the AABB.
  ///
  /// This method will panic if the index is out of bounds.
  pub fn corner(&self, index: usize) -> Vec3 {
    debug_assert!(index < 8)

    let min = self.min();
    let max = self.max();

    match index {
      0 => Vec3::new(min.x, min.y, min.z),
      1 => Vec3::new(min.x, min.y, max.z),
      2 => Vec3::new(min.x, max.y, min.z),
      3 => Vec3::new(min.x, max.y, max.z),
      4 => Vec3::new(max.x, min.y, min.z),
      5 => Vec3::new(max.x, min.y, max.z),
      6 => Vec3::new(max.x, max.y, min.z),
      7 => Vec3::new(max.x, max.y, max.z),
      _ => panic!("Invalid corner index"),
    }
  }

  /// Determines if the AABB contains the given point.
  pub fn contains(&self, point: Vec3) -> bool {
    let min = self.min();
    let max = self.max();

    point.x >= min.x
      && point.x <= max.x
      && point.y >= min.y
      && point.y <= max.y
      && point.z >= min.z
      && point.z <= max.z
  }

  /// Determines if the AABB intersects the given other [`AABB`].
  pub fn intersects(&self, other: &Self) -> bool {
    let min = self.min();
    let max = self.max();

    let other_min = other.min();
    let other_max = other.max();

    other_min.x <= max.x
      && other_max.x >= min.x
      && other_min.y <= max.y
      && other_max.y >= min.y
      && other_min.z <= max.z
      && other_max.z >= min.z
  }

  /// Builds a new AABB that is union with the given other [`AABB`].
  pub fn union(&self, other: &Self) -> Self {
    let min = self.min().min(other.min());
    let max = self.max().max(other.max());

    Self::from_min_max(min, max)
  }

  /// Transforms all points in the [`AABB`] by the given matrix.
  pub fn transform(&self, transform: &Mat4) -> Self {
    let min = self.min();
    let max = self.max();

    let corners = [
      Vec3::new(min.x, min.y, min.z),
      Vec3::new(min.x, min.y, max.z),
      Vec3::new(min.x, max.y, min.z),
      Vec3::new(min.x, max.y, max.z),
      Vec3::new(max.x, min.y, min.z),
      Vec3::new(max.x, min.y, max.z),
      Vec3::new(max.x, max.y, min.z),
      Vec3::new(max.x, max.y, max.z),
    ];

    let mut new_min = Vec3::splat(f32::MAX);
    let mut new_max = Vec3::splat(f32::MIN);

    for corner in &corners {
      let transformed = transform.transform_point3(*corner);

      new_min = new_min.min(transformed);
      new_max = new_max.max(transformed);
    }

    Self::from_min_max(new_min, new_max)
  }
}
