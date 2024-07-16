use common::{vec3, Camera, Color, Vec2, Vec3, AABB};

use crate::{
  CullableObject, ObjectBounds, RenderFrame, RenderPass, RenderScene, RenderTarget, RenderTargetDescriptor,
  RenderTextureDescriptor, TargetError, TextureFormat, TextureOptions,
};

/// A light in 2 space.
pub enum Light {
  PointLight {
    position: Vec2,
    color: Color,
    intensity: f32,
    radius: f32,
  },
}

/// A render pass that renders all lights in the scene.
pub struct LightPass {
  cascade_1: RenderTarget,
  cascade_2: RenderTarget,
  cascade_3: RenderTarget,
  cascade_4: RenderTarget,
}

impl LightPass {
  /// Creates a new light pass.
  pub fn new() -> Result<Self, TargetError> {
    let base_descriptor = RenderTargetDescriptor {
      color_attachment: RenderTextureDescriptor {
        width: 1920,
        height: 1080,
        options: TextureOptions {
          format: TextureFormat::RGBA8,
          ..Default::default()
        },
      },
      depth_attachment: None,
      stencil_attachment: None,
    };

    Ok(Self {
      cascade_1: RenderTarget::new(&base_descriptor.with_size(1024, 1024))?,
      cascade_2: RenderTarget::new(&base_descriptor.with_size(512, 512))?,
      cascade_3: RenderTarget::new(&base_descriptor.with_size(256, 256))?,
      cascade_4: RenderTarget::new(&base_descriptor.with_size(128, 128))?,
    })
  }
}

impl RenderPass for LightPass {
  fn render_camera(&self, _scene: &dyn RenderScene, camera: &dyn Camera, _frame: &mut RenderFrame<'_>) {
    let _frustum = camera.frustum();

    // TODO: render lights
  }
}

impl CullableObject for Light {
  fn compute_bounds(&self) -> ObjectBounds {
    match self {
      Light::PointLight { position, radius, .. } => {
        let position = vec3(position.x, 0.0, position.y);

        let min = position - Vec3::splat(*radius);
        let max = position + Vec3::splat(*radius);

        ObjectBounds::AABB(AABB::from_min_max(min, max))
      }
    }
  }
}
