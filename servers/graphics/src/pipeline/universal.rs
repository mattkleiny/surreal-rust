//! Universal rendering pipeline for scalable games and graphics.
//!
//! The `universal` pipeline is a simple rendering pipeline that is designed to
//! be fast and efficient. It is designed to be used for 2D games, low-poly 3D
//! games and other applications that do not require advanced rendering techniques.

use super::*;

/// A [`RenderPipeline`] for simple 'universal' rendering.
///
/// Not too dissimilar from the Unity `URP` scriptable render pipeline.
///
/// Simple 2D graphics, low-poly 3D graphics, basic environment, sky,
/// tone-mapping and post-processing effects.
pub struct UniversalRenderPipeline {
  effects: Vec<Box<dyn PostEffect>>,
  context: UniversalContext,
}

/// The per-frame rendering context for a [`UniversalRenderPipeline`].
pub struct UniversalContext {}

impl UniversalRenderPipeline {
  pub fn new() -> Self {
    Self {
      effects: vec![],
      context: UniversalContext {},
    }
  }
}

impl RenderPipeline for UniversalRenderPipeline {
  fn begin_frame(&self, frame: &RenderFrame) {
    // rebuild the render frame with context for the pipeline.
    let frame = RenderFrame {
      camera: frame.camera,
      context: &self.context,
    };

    for effect in &self.effects {
      effect.begin_frame(self, &frame);
    }
  }

  fn end_frame(&self, frame: &RenderFrame) {
    // rebuild the render frame with context for the pipeline
    let frame = RenderFrame {
      camera: frame.camera,
      context: &self.context,
    };

    for effect in &self.effects {
      effect.end_frame(self, &frame);
    }
  }
}

/// Allows a type to acts as a post-processing effect in the [`UniversalRenderPipeline`].
pub trait PostEffect {
  fn begin_frame(&self, pipeline: &UniversalRenderPipeline, frame: &RenderFrame<UniversalContext>);
  fn end_frame(&self, pipeline: &UniversalRenderPipeline, frame: &RenderFrame<UniversalContext>);
}

/// The tone-mapper to use in the [`UniversalRenderPipeline`].
#[derive(Default)]
pub enum ToneMapper {
  #[default]
  ACES,
}
