use super::*;

/// A frustum in 3-space.
#[repr(C)]
#[derive(Default, Clone, Debug)]
pub struct Frustum {
  pub near: Plane,
  pub far: Plane,
  pub left: Plane,
  pub right: Plane,
  pub top: Plane,
  pub bottom: Plane,
}

impl Frustum {
  /// An empty frustum.
  pub const EMPTY: Frustum = Frustum {
    near: Plane::ZERO,
    far: Plane::ZERO,
    left: Plane::ZERO,
    right: Plane::ZERO,
    top: Plane::ZERO,
    bottom: Plane::ZERO,
  };

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
      near: Plane::from_vec4(projection_view.row(2) + projection_view.row(3)),
      far: Plane::from_vec4(projection_view.row(3) - projection_view.row(2)),
      left: Plane::from_vec4(projection_view.row(3) + projection_view.row(0)),
      right: Plane::from_vec4(projection_view.row(3) - projection_view.row(0)),
      top: Plane::from_vec4(projection_view.row(3) - projection_view.row(1)),
      bottom: Plane::from_vec4(projection_view.row(3) + projection_view.row(1)),
    }
  }

  /// Creates a frustum from a set of orthographic planes.
  pub fn from_ortho_planes(center: Vec3, ortho_size: f32, near_plane: f32, far_plane: f32) -> Self {
    let half_width = ortho_size / 2.0;
    let half_height = half_width;

    Self {
      near: Plane::new(Vec3::Z, near_plane),
      far: Plane::new(Vec3::NEG_Z, far_plane),
      left: Plane::new(Vec3::X, center.x - half_width),
      right: Plane::new(Vec3::NEG_X, center.x + half_width),
      top: Plane::new(Vec3::Y, center.y - half_height),
      bottom: Plane::new(Vec3::NEG_Y, center.y + half_height),
    }
  }

  /// Determines whether the given point is contained within this frustum.
  pub fn contains_point(&self, point: Vec3) -> bool {
    self.near.distance_to_point(point) >= 0.0
      && self.far.distance_to_point(point) <= 0.0
      && self.left.distance_to_point(point) >= 0.0
      && self.right.distance_to_point(point) <= 0.0
      && self.top.distance_to_point(point) >= 0.0
      && self.bottom.distance_to_point(point) <= 0.0
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
  pub fn contains_aabb(&self, aabb: &AABB) -> bool {
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
  ///
  /// The order is Near, Far, Left, Right, Top, Bottom.
  pub fn into_array(self) -> [Plane; 6] {
    [self.near, self.far, self.left, self.right, self.top, self.bottom]
  }

  /// Converts this frustum to an array of planes.
  ///
  /// The order is Near, Far, Left, Right, Top, Bottom.
  pub fn to_array(&self) -> [Plane; 6] {
    [self.near, self.far, self.left, self.right, self.top, self.bottom]
  }

  /// Converts this frustum to a slice of planes.
  ///
  /// The order is Near, Far, Left, Right, Top, Bottom.
  pub fn as_slice(&self) -> &[Plane; 6] {
    unsafe { crate::reinterpret_cast(self) }
  }
}
