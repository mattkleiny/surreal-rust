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
  fn distance_to(&self, point: Self::Vector) -> <Self::Vector as Vector>::Scalar;

  /// Converts this signed distance field into an evaluated field structure.
  fn to_field(&self) -> Field<Self::Vector> {
    todo!()
  }
}

impl SDF for Circle {
  type Vector = Vec2;

  fn distance_to(&self, point: Vec2) -> f32 {
    (point - self.center).length() - self.radius
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
    let radius = self.radius;
    let delta = (point - self.center).abs() - radius;

    delta.xy().length().max(delta.z) - self.height
  }
}

impl SDF for Trapezoid {
  type Vector = Vec3;

  fn distance_to(&self, point: Vec3) -> f32 {
    let half_size = self.size / 2.0;
    let delta = (point - self.center).abs() - half_size;

    let xy = delta.xy().max(Vec2::ZERO);
    let z = delta.z - half_size.y;

    xy.length().max(z.abs()) + xy.min(Vec2::ZERO).max_element()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_circle_distance() {
    let circle = Circle {
      center: Vec2::new(0.0, 0.0),
      radius: 1.0,
    };

    let point_inside = Vec2::new(0.5, 0.0);
    let point_on = Vec2::new(1.0, 0.0);
    let point_outside = Vec2::new(2.0, 0.0);

    assert_eq!(circle.distance_to(point_inside), -0.5);
    assert_eq!(circle.distance_to(point_on), 0.0);
    assert_eq!(circle.distance_to(point_outside), 1.0);
  }

  #[test]
  fn test_sphere_distance() {
    let sphere = Sphere {
      center: Vec3::new(0.0, 0.0, 0.0),
      radius: 1.0,
    };

    let point_inside = Vec3::new(0.5, 0.0, 0.0);
    let point_outside = Vec3::new(2.0, 0.0, 0.0);

    assert_eq!(sphere.distance_to(point_inside), -0.5);
    assert_eq!(sphere.distance_to(point_outside), 1.0);
  }

  #[test]
  fn test_cube_distance() {
    let cube = Cube {
      center: Vec3::new(0.0, 0.0, 0.0),
      size: Vec3::new(2.0, 2.0, 2.0),
    };

    let point_inside = Vec3::new(0.5, 0.5, 0.5);
    let point_outside = Vec3::new(2.0, 2.0, 2.0);

    assert_eq!(cube.distance_to(point_inside), -0.5);
    assert_eq!(cube.distance_to(point_outside), 1.7320508);
  }

  #[test]
  fn test_cylinder_distance() {
    let cylinder = Cylinder {
      center: Vec3::new(0.0, 0.0, 0.0),
      radius: 1.0,
      height: 2.0,
    };

    let point_inside = Vec3::new(0.5, 0.0, 0.0);
    let point_outside = Vec3::new(2.0, 0.0, 0.0);

    assert_eq!(cylinder.distance_to(point_inside), -0.5);
    assert_eq!(cylinder.distance_to(point_outside), 1.0);
  }

  #[test]
  fn test_trapezoid_distance() {
    let trapezoid = Trapezoid {
      center: Vec3::new(0.0, 0.0, 0.0),
      size: Vec3::new(2.0, 2.0, 2.0),
    };

    let point_inside = Vec3::new(0.5, 0.5, 0.5);
    let point_outside = Vec3::new(2.0, 2.0, 2.0);

    assert_eq!(trapezoid.distance_to(point_inside), 1.5);
    assert_eq!(trapezoid.distance_to(point_outside), 1.4142135);
  }
}
