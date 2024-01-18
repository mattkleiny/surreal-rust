use common::collections::SmallVec;

use super::*;

/// Represents a scene that can be rendered by a [`RenderPipeline`].
pub trait RenderScene {
  /// Gets the cameras that should be used to render this scene.
  fn cameras(&self) -> SmallVec<[&dyn RenderCamera; 4]>;
}

/// Represents a camera that can be used to render a scene.
pub trait RenderCamera {
  /// Finds all visible objects in the scene.
  fn cull_visible_objects(&self) -> Vec<&dyn RenderObject>;
}

/// Represents an object capable of being rendered.
pub trait RenderObject {
  /// Renders the object to the given [`Renderer`].
  fn render(&self, renderer: &mut Renderer);
}

/// Represents a pipeline capable of rendering a scene.
///
/// A pipeline is a collection of passes that are executed in order to render a
/// scene. Each pass is responsible for rendering a specific set of objects.
pub trait RenderPipeline {
  /// Renders the given scene.
  fn render(&mut self, scene: &dyn RenderScene);
}

/// A [`RenderPipeline`] that executes many [`RenderPass`]es in order.
pub struct MultiPassPipeline {
  renderer: Renderer,
  passes: Vec<Box<dyn RenderPass>>,
}

impl RenderPipeline for MultiPassPipeline {
  fn render(&mut self, scene: &dyn RenderScene) {
    // begin the frame
    for pass in &self.passes {
      pass.begin_frame(scene, &mut self.renderer);
    }

    // render each camera
    for camera in scene.cameras() {
      for pass in &self.passes {
        pass.begin_camera(camera, &mut self.renderer);
      }

      for pass in &self.passes {
        pass.render_camera(camera, &mut self.renderer);
      }

      for pass in &self.passes {
        pass.end_camera(camera, &mut self.renderer);
      }
    }

    // finalize the frame
    for pass in &self.passes {
      pass.end_frame(scene, &mut self.renderer);
    }
  }
}

/// A single pass of a [`MultiPassPipeline`].
#[allow(unused_variables)]
pub trait RenderPass {
  fn begin_frame(&self, scene: &dyn RenderScene, renderer: &mut Renderer) {}
  fn begin_camera(&self, camera: &dyn RenderCamera, renderer: &mut Renderer) {}
  fn render_camera(&self, camera: &dyn RenderCamera, renderer: &mut Renderer) {}
  fn end_camera(&self, camera: &dyn RenderCamera, renderer: &mut Renderer) {}
  fn end_frame(&self, scene: &dyn RenderScene, renderer: &mut Renderer) {}
}

pub mod forward {
  //! A forward rendering pipeline.

  use super::*;

  /// A [`RenderPass`] that renders all objects in the scene to a depth target.
  struct DepthPass {
    depth_target: RenderTarget,
  }

  impl RenderPass for DepthPass {
    fn render_camera(&self, camera: &dyn RenderCamera, renderer: &mut Renderer) {
      self.depth_target.activate();

      for object in camera.cull_visible_objects() {
        object.render(renderer);
      }

      self.depth_target.deactivate();
    }
  }

  /// A [`RenderPass`] that renders all objects in the scene to a color target.
  struct ColorPass {
    color_target: RenderTarget,
  }

  impl RenderPass for ColorPass {
    fn render_camera(&self, camera: &dyn RenderCamera, renderer: &mut Renderer) {
      self.color_target.activate();

      for object in camera.cull_visible_objects() {
        object.render(renderer);
      }

      self.color_target.deactivate();
    }

    fn end_frame(&self, scene: &dyn RenderScene, _renderer: &mut Renderer) {
      self.color_target.blit_to_display(TextureFilter::Nearest);
    }
  }

  impl MultiPassPipeline {
    /// Builds a new [`MultiPassPipeline`] for forward rendering.
    pub fn new_forward_pipeline(graphics: &GraphicsEngine) -> Self {
      let renderer = Renderer::new(graphics);

      let depth_target = RenderTarget::new(
        &graphics,
        &RenderTargetDescriptor {
          color_attachment: RenderTextureDescriptor {
            width: 1920,
            height: 1080,
            options: TextureOptions {
              format: TextureFormat::RGBA32,
              ..Default::default()
            },
          },
          depth_attachment: None,
          stencil_attachment: None,
        },
      )
      .expect("Failed to create depth target");

      let color_target = RenderTarget::new(
        &graphics,
        &RenderTargetDescriptor {
          color_attachment: RenderTextureDescriptor {
            width: 1920,
            height: 1080,
            options: TextureOptions {
              format: TextureFormat::RGBA32,
              ..Default::default()
            },
          },
          depth_attachment: None,
          stencil_attachment: None,
        },
      )
      .expect("Failed to create color target");

      MultiPassPipeline {
        renderer,
        passes: vec![
          Box::new(DepthPass { depth_target }),
          Box::new(ColorPass { color_target }),
        ],
      }
    }
  }
}
