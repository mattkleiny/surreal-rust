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
  /// The type of vector used to represent points in space.
  type Vector: Vector;

  /// Computes the distance to the shape at the given point.
  fn distance_to(&self, point: Vec3) -> <Self::Vector as Vector>::Scalar;

  /// Converts this signed distance field into an evaluated field structure.
  fn to_field(&self) -> Field<Self::Vector> {
    todo!()
  }
}

impl SDF for Sphere {
  type Vector = Vec3;

  fn distance_to(&self, point: Vec3) -> f32 {
    (point - self.center).length() - self.radius
  }
}

impl SDF for Cube {
  type Vector = Vec3;

  fn distance_to(&self, point: Vec3) -> f32 {
    let half_size = self.size / 2.0;
    let delta = (point - self.center).abs() - half_size;

    delta.max(Vec3::ZERO).length() + delta.min(Vec3::ZERO).max_element()
  }
}

impl SDF for Cylinder {
  type Vector = Vec3;

  fn distance_to(&self, point: Vec3) -> f32 {
    let delta = (point - self.center).abs() - Vec3::new(self.radius, self.height / 2.0, self.radius);

    delta.max(Vec3::ZERO).length() + delta.min(Vec3::ZERO).max_element()
  }
}

// TODO: implement more types of shapes and add tests
