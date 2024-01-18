use super::*;

/// Represents a plane in 3-space.
#[repr(C)]
#[derive(Serialize, Deserialize, Default, Copy, Clone, Debug, PartialEq)]
pub struct Plane {
  pub normal: Vec3,
  pub distance: f32,
}

/// Represents a half-space in 3d; usually results from a plane split of the
/// space.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum HalfSpace {
  Behind,
  Inline,
  Front,
}

impl Plane {
  pub const ZERO: Self = Self::new(Vec3::ZERO, 0.0);

  /// Creates a new plane from a normal and a distance.
  pub const fn new(normal: Vec3, distance: f32) -> Self {
    Self { normal, distance }
  }

  /// Creates a new plane from a normal and a point on the plane.
  pub fn from_point(normal: Vec3, point: Vec3) -> Self {
    Self {
      normal,
      distance: -normal.dot(point),
    }
  }

  /// Creates a new plane from three points on the plane.
  pub fn from_points(a: Vec3, b: Vec3, c: Vec3) -> Self {
    let normal = (b - a).cross(c - a).normalize();

    Self::from_point(normal, a)
  }

  /// Creates a new plane from a vector4.
  pub fn from_vector4(vector: Vec4) -> Self {
    Self {
      normal: Vec3::new(vector.x, vector.y, vector.z),
      distance: vector.w,
    }
  }

  /// Returns the distance from the plane to a point.
  pub fn distance_to_point(&self, point: Vec3) -> f32 {
    let origin = self.normal * self.distance;

    self.normal.dot(point - origin)
  }

  /// Returns the half-space that a point is in.
  pub fn half_space(&self, point: Vec3) -> HalfSpace {
    let distance = self.distance_to_point(point);
    if distance < 0.0 {
      HalfSpace::Behind
    } else if distance > 0.0 {
      HalfSpace::Front
    } else {
      HalfSpace::Inline
    }
  }

  /// Returns the half-space that a sphere is in.
  pub fn half_space_sphere(&self, sphere: &Sphere) -> HalfSpace {
    let distance = self.distance_to_point(sphere.center);
    if distance < -sphere.radius {
      HalfSpace::Behind
    } else if distance > sphere.radius {
      HalfSpace::Front
    } else {
      HalfSpace::Inline
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_calculate_distance_to_point() {
    let plane = Plane::new(vec3(0., 1., 0.), 2.);
    let point = vec3(0.0, 3.0, 0.0);

    assert_eq!(plane.distance_to_point(point), 1.0);
  }

  #[test]
  fn test_half_space_should_be_correct() {
    let normal = Vec3::new(0.0, 1.0, 0.0);
    let distance = 2.0;
    let plane = Plane::new(normal, distance);

    let point_behind = Vec3::new(0.0, 1.0, 0.0);
    let point_inline = Vec3::new(0.0, 2.0, 0.0);
    let point_front = Vec3::new(0.0, 3.0, 0.0);

    assert_eq!(plane.half_space(point_front), HalfSpace::Front);
    assert_eq!(plane.half_space(point_inline), HalfSpace::Inline);
    assert_eq!(plane.half_space(point_behind), HalfSpace::Behind);
  }
}
