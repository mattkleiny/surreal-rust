//! Camera types and utilities.

use common::maths::{Mat4, Plane, Vec3};

use super::*;

/// The name of the uniform containing the projection-view matrix of the camera.
pub const UNIFORM_PROJECTION_VIEW: UniformKey<Mat4> = UniformKey::new("u_projectionView");

/// Represents a camera.
pub trait Camera {
  /// Computes the projection matrix for this camera.
  fn projection(&self) -> Mat4;

  /// Computes the view matrix for this camera.
  fn view(&self) -> Mat4;

  /// Computes the frustum for this camera.
  fn frustum(&self) -> CameraFrustum {
    // compute the frustum planes from the view-projection matrix
    let vp = self.projection() * self.view();

    let mut planes = CameraFrustum {
      near: Plane::ZERO,
      far: Plane::ZERO,
      left: Plane::ZERO,
      right: Plane::ZERO,
      top: Plane::ZERO,
      bottom: Plane::ZERO,
    };

    planes.near = Plane::from_vector4(vp.row(2) + vp.row(3));
    planes.far = Plane::from_vector4(vp.row(3) - vp.row(2));
    planes.left = Plane::from_vector4(vp.row(3) + vp.row(0));
    planes.right = Plane::from_vector4(vp.row(3) - vp.row(0));
    planes.top = Plane::from_vector4(vp.row(3) - vp.row(1));
    planes.bottom = Plane::from_vector4(vp.row(3) + vp.row(1));

    planes
  }
}

/// A frustum for a camera.
pub struct CameraFrustum {
  pub near: Plane,
  pub far: Plane,
  pub left: Plane,
  pub right: Plane,
  pub top: Plane,
  pub bottom: Plane,
}

/// An orthographic camera.
pub struct OrthographicCamera {
  pub position: Vec3,
  pub look_at: Vec3,
  pub up: Vec3,
  pub near_plane: f32,
  pub far_plane: f32,
  pub ortho_size: f32,
}

impl Default for OrthographicCamera {
  fn default() -> Self {
    Self {
      position: Vec3::ZERO,
      look_at: Vec3::NEG_Z,
      up: Vec3::Y,
      near_plane: 0.1,
      far_plane: 100.0,
      ortho_size: 1.0,
    }
  }
}

impl Camera for OrthographicCamera {
  fn projection(&self) -> Mat4 {
    todo!()
  }

  fn view(&self) -> Mat4 {
    Mat4::look_at_lh(self.position, self.look_at, self.up)
  }
}

/// A perspective camera.
pub struct PerspectiveCamera {
  pub position: Vec3,
  pub look_at: Vec3,
  pub up: Vec3,
  pub near_plane: f32,
  pub far_plane: f32,
  pub fov: f32,
  pub aspect_ratio: f32,
}

impl Default for PerspectiveCamera {
  fn default() -> Self {
    Self {
      position: Vec3::ZERO,
      look_at: Vec3::NEG_Z,
      up: Vec3::Y,
      near_plane: 0.1,
      far_plane: 100.0,
      fov: 60.0,
      aspect_ratio: 1.0,
    }
  }
}

impl Camera for PerspectiveCamera {
  fn projection(&self) -> Mat4 {
    Mat4::perspective_lh(self.fov, self.aspect_ratio, self.near_plane, self.far_plane)
  }

  fn view(&self) -> Mat4 {
    Mat4::look_at_lh(self.position, self.look_at, self.up)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn camera_should_compute_frustum() {
    let camera = PerspectiveCamera::default();
    let frustum = camera.frustum();

    assert_eq!(frustum.near, Plane::ZERO);
  }
}
