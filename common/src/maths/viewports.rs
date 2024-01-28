use super::Camera;

/// Manages the viewport of a camera.
pub trait Viewport {
  fn apply_to(&self, camera: &mut dyn Camera);
}

pub struct FillViewport {}
pub struct FitViewport {}
pub struct StretchViewport {}
