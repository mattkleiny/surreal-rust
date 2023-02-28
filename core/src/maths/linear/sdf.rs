use super::*;

/// Allows computing the signed distance field of a shape.
///
/// The signed distance field is a function that returns the distance from a
/// point in arbitrary space to the shape.
///
/// The distance is negative if the point is inside the shape, and positive if
/// the point is outside the shape, and zero if the point is on the shape.
///
/// The distance is measured in the same units as the shape's dimensions.
pub trait SDF {
  /// Computes the distance to the shape at the given point.
  fn distance_to(&self, point: Vec3) -> f32;
}

impl SDF for Sphere {
  fn distance_to(&self, point: Vec3) -> f32 {
    (point - self.center).length() - self.radius
  }
}

impl SDF for Cube {
  fn distance_to(&self, point: Vec3) -> f32 {
    let half_size = self.size / 2.0;
    let d = (point - self.center).abs() - half_size;

    d.max(Vec3::ZERO).length() + d.min(Vec3::ZERO).max_component()
  }
}

impl SDF for Cylinder {
  fn distance_to(&self, point: Vec3) -> f32 {
    let d = (point - self.center).abs() - Vec3::new(self.radius, self.height / 2.0, self.radius);

    d.max(Vec3::ZERO).length() + d.min(Vec3::ZERO).max_component()
  }
}

impl SDF for Cone {
  fn distance_to(&self, point: Vec3) -> f32 {
    let d = (point - self.center).abs() - Vec3::new(self.radius, self.height / 2.0, self.radius);

    d.max(Vec3::ZERO).length() + d.min(Vec3::ZERO).max_component()
  }
}

// TODO: implement more types of shapes and add tests
