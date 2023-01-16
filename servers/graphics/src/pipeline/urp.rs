//! Universal rendering pipeline for scalable games and graphics.
//!
//! The `urp` pipeline is a simple rendering pipeline that is designed to
//! be fast and efficient. It is designed to be used for 2D games, low-poly 3D
//! games and other applications that do not require advanced rendering
//! techniques.

use surreal::graphics::{Color, TextureFormat};

use super::RenderPass;
use crate::{Command, CommandBuffer, CullingResult, GraphicsServer, RenderCamera, RenderManager, RenderPipeline, RenderTargetId};

/// A [`RenderPipeline`] that targets most platforms in a scalable manner.
pub struct UniversalPipeline {
  color_target: RenderTargetId,
  depth_target: RenderTargetId,
  visible_objects: Box<[CullingResult]>,
}

impl UniversalPipeline {
  /// Creates a new managed [`UniversalPipeline`].
  pub fn new<'a>(graphics: &GraphicsServer) -> surreal::Result<RenderManager<'a, UniversalPipeline>> {
    // create initial render targets
    let color_target = graphics.render_target_create(Some("Color Target"), (1920, 1080), TextureFormat::RGBA8)?;
    let depth_target = graphics.render_target_create(Some("Depth Target"), (1920, 1080), TextureFormat::R32)?;

    let manager = RenderManager::new(
      "Universal Render Pipeline",
      graphics,
      Self {
        color_target,
        depth_target,
        visible_objects: Box::new([]),
      },
      vec![
        Box::new(DepthPass::default()),
        Box::new(OpaquePass::default()),
        Box::new(TransparentPass::default()),
        Box::new(PostEffectPass::default()),
        Box::new(ToneMapperPass::default()),
      ],
    );

    Ok(manager)
  }
}

impl RenderPipeline for UniversalPipeline {
  fn begin_camera(&mut self, commands: &mut CommandBuffer, camera: &dyn RenderCamera) {
    commands.enqueue(Command::SetViewMatrix {
      view_matrix: camera.view_matrix().to_cols_array(),
    });

    commands.enqueue(Command::SetProjectionMatrix {
      projection_matrix: camera.projection_matrix().to_cols_array(),
    });

    self.visible_objects = camera.cull_visible_objects();
  }
}

/// Calculates the depth buffer.
#[derive(Default)]
pub struct DepthPass {}

impl RenderPass<UniversalPipeline> for DepthPass {
  fn begin_camera(&mut self, commands: &mut CommandBuffer, _camera: &dyn RenderCamera, pipeline: &mut UniversalPipeline) {
    commands.enqueue(Command::SetRenderTarget {
      render_target_id: Some(pipeline.depth_target),
      clear_color: None,
      depth_value: Some(0.),
    });

    for _result in pipeline.visible_objects.iter() {
      // TODO: render the culled objects?
    }
  }
}

/// Renders opaque geometry.
#[derive(Default)]
pub struct OpaquePass {}

impl RenderPass<UniversalPipeline> for OpaquePass {
  fn begin_camera(&mut self, commands: &mut CommandBuffer, _camera: &dyn RenderCamera, pipeline: &mut UniversalPipeline) {
    commands.enqueue(Command::SetRenderTarget {
      render_target_id: Some(pipeline.color_target),
      clear_color: Some(Color::BLACK),
      depth_value: None,
    });
  }
}

/// Renders transparent geometry.
#[derive(Default)]
pub struct TransparentPass {}

impl RenderPass<UniversalPipeline> for TransparentPass {
  fn begin_camera(&mut self, commands: &mut CommandBuffer, _camera: &dyn RenderCamera, pipeline: &mut UniversalPipeline) {
    commands.enqueue(Command::SetRenderTarget {
      render_target_id: Some(pipeline.color_target),
      clear_color: Some(Color::BLACK),
      depth_value: None,
    });
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
