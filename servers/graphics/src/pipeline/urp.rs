//! Universal rendering pipeline for scalable games and graphics.
//!
//! The `urp` pipeline is a simple rendering pipeline that is designed to
//! be fast and efficient. It is designed to be used for 2D games, low-poly 3D
//! games and other applications that do not require advanced rendering techniques.

use surreal::graphics::{Color, TextureFormat};

use crate::{Command, CommandBuffer, GraphicsServer, RenderCamera, RenderManager, RenderPipeline, RenderTargetId};

use super::RenderPass;

/// A [`RenderPipeline`] that targets most platforms in a scalable manner.
pub struct UniversalPipeline {
  color_target: RenderTargetId,
  depth_target: RenderTargetId,
}

impl UniversalPipeline {
  /// Creates a new managed [`UniversalPipeline`].
  pub fn new<'a>(graphics: &GraphicsServer) -> surreal::Result<RenderManager<'a, UniversalPipeline>> {
    Ok(RenderManager::new(
      "Universal Render Pipeline",
      graphics,
      Self {
        color_target: graphics.render_target_create(Some("Color Target"), (1920, 1080), TextureFormat::RGBA8)?,
        depth_target: graphics.render_target_create(Some("Depth Target"), (1920, 1080), TextureFormat::R32)?,
      },
      vec![
        Box::new(DepthPass::default()),
        Box::new(OpaquePass::default()),
        Box::new(TransparentPass::default()),
        Box::new(PostEffectPass::default()),
        Box::new(ToneMapperPass::default()),
      ],
    ))
  }
}

impl RenderPipeline for UniversalPipeline {}

/// Calculates the depth buffer.
#[derive(Default)]
pub struct DepthPass {}

impl RenderPass<UniversalPipeline> for DepthPass {
  fn begin_camera(&mut self, commands: &mut CommandBuffer, _camera: &dyn RenderCamera, pipeline: &mut UniversalPipeline) {
    commands.enqueue(Command::SetRenderTarget {
      render_target_id: Some(pipeline.depth_target),
      clear_color: None,
      depth_value: Some(0.),
    })
  }
}

/// Renders opaque geometry.
#[derive(Default)]
pub struct OpaquePass {}

impl RenderPass<UniversalPipeline> for OpaquePass {
  fn begin_camera(&mut self, commands: &mut CommandBuffer, camera: &dyn RenderCamera, pipeline: &mut UniversalPipeline) {
    commands.enqueue(Command::SetRenderTarget {
      render_target_id: Some(pipeline.color_target),
      clear_color: Some(Color::BLACK),
      depth_value: None,
    });

    commands.enqueue(Command::SetViewMatrix {
      view_matrix: camera.view_matrix().to_cols_array(),
    });

    commands.enqueue(Command::SetProjectionMatrix {
      projection_matrix: camera.projection_matrix().to_cols_array(),
    })
  }
}

/// Renders transparent geometry.
#[derive(Default)]
pub struct TransparentPass {}

impl RenderPass<UniversalPipeline> for TransparentPass {
  fn begin_camera(&mut self, commands: &mut CommandBuffer, camera: &dyn RenderCamera, pipeline: &mut UniversalPipeline) {
    commands.enqueue(Command::SetRenderTarget {
      render_target_id: Some(pipeline.color_target),
      clear_color: Some(Color::BLACK),
      depth_value: None,
    });

    commands.enqueue(Command::SetViewMatrix {
      view_matrix: camera.view_matrix().to_cols_array(),
    });

    commands.enqueue(Command::SetProjectionMatrix {
      projection_matrix: camera.projection_matrix().to_cols_array(),
    })
  }
}

/// Applies post-processing effects.
#[derive(Default)]
pub struct PostEffectPass {}

impl RenderPass<UniversalPipeline> for PostEffectPass {}

/// Applies tone mapping to the final image.
#[derive(Default)]
pub struct ToneMapperPass {}

impl RenderPass<UniversalPipeline> for ToneMapperPass {}
