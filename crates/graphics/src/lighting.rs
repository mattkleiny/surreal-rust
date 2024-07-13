use common::{Camera, Color, Vec2};

use crate::{RenderFrame, RenderPass, RenderScene, RenderTarget};

/// A point light in 2 space.
pub struct PointLight {
  pub position: Vec2,
  pub color: Color,
  pub intensity: f32,
  pub radius: f32,
}

/// A render pass that renders all lights in the scene.
pub struct LightPass {
  pub lights: Vec<PointLight>,

  lighting_cascade_1: RenderTarget,
  lighting_cascade_2: RenderTarget,
  lighting_cascade_3: RenderTarget,
}

impl RenderPass for LightPass {
  fn begin_frame(&self, scene: &dyn RenderScene, frame: &mut RenderFrame<'_>) {
    todo!()
  }

  fn render_camera(&self, scene: &dyn RenderScene, camera: &dyn Camera, frame: &mut RenderFrame<'_>) {
    todo!()
  }
}
