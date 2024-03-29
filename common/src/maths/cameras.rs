//! Camera types and utilities.

use super::*;

/// Represents a camera.
pub trait Camera {
  /// Computes the projection matrix for this camera.
  fn projection(&self) -> Mat4;

  /// Computes the view matrix for this camera.
  fn view(&self) -> Mat4;

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
      ortho_size: 1.0,
    }
  }
}

impl Camera for OrthographicCamera {
  fn projection(&self) -> Mat4 {
    Mat4::orthographic_lh(
      self.ortho_size / 2.,
      self.ortho_size / 2.,
      self.ortho_size / 2.,
      self.ortho_size / 2.,
      self.near_plane,
      self.far_plane,
    )
  }

  fn view(&self) -> Mat4 {
    Mat4::look_at_lh(self.position, self.look_at, self.up)
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
  fn projection(&self) -> Mat4 {
    Mat4::perspective_lh(self.fov, self.aspect_ratio, self.near_plane, self.far_plane)
  }

  fn view(&self) -> Mat4 {
    Mat4::look_at_lh(self.position, self.look_at, self.up)
  }
}

/// Manages the viewport of a camera.
pub trait Viewport {
  /// Applies the viewport to the given camera.
  fn apply_to(&self, camera: &mut dyn Camera);
}

/// A viewport that fills the entire screen with no stretching.
pub struct FillViewport {}

/// A viewport that fits the entire screen.
pub struct FitViewport {}

/// A viewport that stretches to fill the entire screen.
pub struct StretchViewport {}
