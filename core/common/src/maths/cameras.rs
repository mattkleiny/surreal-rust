//! Camera types and utilities.

use super::*;

/// Represents a camera.
pub trait Camera {
  /// Gets the position of this camera.
  fn position(&self) -> Vec3;

  /// Computes the projection matrix for this camera.
  fn projection(&self) -> Mat4;

  /// Computes the view matrix for this camera.
  fn view(&self) -> Mat4;

  /// Computes the projection-view matrix for this camera.
  #[inline]
  fn projection_view(&self) -> Mat4 {
    self.projection() * self.view()
  }

  /// Computes the frustum for this camera.
  #[inline]
  fn frustum(&self) -> Frustum {
    Frustum::from_projection_view(self.projection() * self.view())
  }
}

/// An orthographic camera.
#[derive(Clone, Debug)]
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
      ortho_size: 144.0,
    }
  }
}

impl Camera for OrthographicCamera {
  fn position(&self) -> Vec3 {
    self.position
  }

  fn projection(&self) -> Mat4 {
    Mat4::orthographic_rh_gl(
      0.,
      self.ortho_size,
      self.ortho_size,
      0.,
      self.near_plane,
      self.far_plane,
    )
  }

  fn view(&self) -> Mat4 {
    Mat4::look_at_rh(self.position, self.look_at, self.up)
  }
}

/// A perspective camera.
#[derive(Clone, Debug)]
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
  fn position(&self) -> Vec3 {
    self.position
  }
  
  fn projection(&self) -> Mat4 {
    Mat4::perspective_lh(self.fov, self.aspect_ratio, self.near_plane, self.far_plane)
  }

  fn view(&self) -> Mat4 {
    Mat4::look_at_lh(self.position, self.look_at, self.up)
  }
}
