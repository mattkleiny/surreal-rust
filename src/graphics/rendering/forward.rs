//! A standard-purpose forward rendering pipeline.

use crate::{
  maths::Matrix4x4,
  prelude::{SpriteBatchContext, SpriteBatchDescriptor},
};

use super::*;

/// Builds a forward `RenderPipeline`.
pub struct ForwardPipelineBuilder {
  pub graphics: GraphicsServer,
  pub size: (u32, u32),
}

impl ForwardPipelineBuilder {
  pub fn build(&self) -> RenderPipeline {
    let mut pipeline = RenderPipeline::new(&self.graphics);

    pipeline.configure(SpriteBatchDescriptor {
      projection_view: Matrix4x4::orthographic(self.size.0 as f32, self.size.1 as f32, 0., 100.),
      ..Default::default()
    });

    pipeline.add_pass(OpaquePass {});
    pipeline.add_pass(TransparentPass {});
    pipeline.add_pass(ScreenGrabPass {
      grab_target: RenderTarget::new(
        &self.graphics,
        &RenderTargetDescriptor {
          color_attachment: RenderTextureDescriptor {
            width: self.size.0,
            height: self.size.1,
            options: TextureOptions {
              format: TextureFormat::RGBA8,
              sampler: TextureSampler {
                wrap_mode: TextureWrap::Clamp,
                minify_filter: TextureFilter::Nearest,
                magnify_filter: TextureFilter::Nearest,
              },
            },
          },
          depth_attachment: None,
          stencil_attachment: None,
        },
      ),
    });
    pipeline.add_pass(PostEffectPass {});
    pipeline.add_pass(CompositePass {});

    pipeline
  }
}

/// Adds an opaque pass to the rendering pipeline.
struct OpaquePass {}

impl RenderPass for OpaquePass {
  fn render_frame(&mut self, frame: &mut RenderFrame) {
    for _visible_object in frame
      .visible_objects
      .iter()
      .filter(|it| it.material_key.flags.contains(MaterialFlags::OPAQUE))
    {
      frame.manager.with(|context: &mut SpriteBatchContext| {
        context.material.set_blend_state(BlendState::Disabled);

        todo!();
      });
    }
  }
}

/// Adds a transparent pass to the rendering pipeline.
struct TransparentPass {}

impl RenderPass for TransparentPass {
  fn render_frame(&mut self, frame: &mut RenderFrame) {
    for _visible_object in frame
      .visible_objects
      .iter()
      .filter(|it| it.material_key.flags.contains(MaterialFlags::TRANSPARENT))
    {
      frame.manager.with(|context: &mut SpriteBatchContext| {
        context.material.set_blend_state(BlendState::Enabled {
          source: BlendFactor::SrcAlpha,
          destination: BlendFactor::OneMinusSrcAlpha,
        });

        todo!();
      });
    }
  }
}

/// Adds a screen-aware forward pass to the rendering pipeline.
struct ScreenGrabPass {
  grab_target: RenderTarget,
}

impl RenderPass for ScreenGrabPass {
  fn begin_frame(&mut self, _context: &mut RenderFrame) {
    self.grab_target.activate();
  }

  fn render_frame(&mut self, frame: &mut RenderFrame) {
    self.grab_target.deactivate();

    for _visible_object in frame
      .visible_objects
      .iter()
      .filter(|it| it.material_key.flags.contains(MaterialFlags::GRAB_PASS))
    {
      frame.manager.with(|_context: &mut SpriteBatchContext| {
        todo!();
      });
    }
  }
}

/// Adds an post-processing pass to the rendering pipeline.
struct PostEffectPass {}

impl RenderPass for PostEffectPass {
  fn render_frame(&mut self, _frame: &mut RenderFrame) {}
}

/// Adds a compositing pass to the rendering pipeline.
struct CompositePass {}

impl RenderPass for CompositePass {
  fn render_frame(&mut self, _frame: &mut RenderFrame) {}
}

#[cfg(test)]
mod tests {
  use crate::scenes::Scene;

  use super::*;

  #[derive(Default)]
  pub struct TestCamera {
    position: Vector3<f32>,
  }

  impl RenderCamera for TestCamera {
    fn compute_frustum(&self) -> CameraFrustum {
      CameraFrustum {
        position: self.position,
        planes: [Plane::default(); 6],
      }
    }
  }

  #[test]
  fn forward_pipeline_should_build_and_render() {
    let graphics = GraphicsServer::new(Box::new(HeadlessGraphicsBackend::new()));

    let mut pipeline = ForwardPipelineBuilder {
      graphics: graphics.clone(),
      size: (256, 144),
    }
    .build();

    let scene = Scene::default();
    let camera = TestCamera::default();

    pipeline.render(&scene, &camera);
  }
}
