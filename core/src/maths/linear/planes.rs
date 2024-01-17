use super::*;

/// Represents a plane in 3-space.
#[derive(Default, Copy, Clone, Debug, PartialEq)]
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
    self.normal.dot(point) + self.distance
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

  /// Returns the half-space that an AABB is in.
  pub fn half_space_aabb(&self, aabb: &AABB) -> HalfSpace {
    let mut behind = 0;
    let mut front = 0;

    for i in 0..8 {
      let distance = self.distance_to_point(aabb.corner(i));
      if distance < 0.0 {
        behind += 1;
      } else if distance > 0.0 {
        front += 1;
      }
    }

    if behind == 8 {
      HalfSpace::Behind
    } else if front == 8 {
      HalfSpace::Front
    } else {
      HalfSpace::Inline
    }
  }
}

// TODO: implement tests for this guy
