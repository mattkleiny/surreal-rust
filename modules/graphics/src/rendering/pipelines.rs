//! Render pipeline abstractions.

use super::*;

/// Represents a scene that can be rendered by a [`RenderPipeline`].
pub trait RenderScene {
  /// Gets the cameras that should be used to render this scene.
  fn cameras(&self) -> Vec<&dyn RenderCamera>;
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
  struct DepthPass {}

  impl RenderPass for DepthPass {
    fn render_camera(&self, camera: &dyn RenderCamera, renderer: &mut Renderer) {
      for object in camera.cull_visible_objects() {
        object.render(renderer);
      }
    }
  }

  /// A [`RenderPass`] that renders all objects in the scene to a color target.
  struct ColorPass {}

  impl RenderPass for ColorPass {
    fn render_camera(&self, camera: &dyn RenderCamera, renderer: &mut Renderer) {
      for object in camera.cull_visible_objects() {
        object.render(renderer);
      }
    }

    fn end_frame(&self, _scene: &dyn RenderScene, _renderer: &mut Renderer) {
      // TODO: blit the color target to the screen.
    }
  }

  impl MultiPassPipeline {
    /// Builds a new [`MultiPassPipeline`] for forward rendering.
    pub fn new_forward_pipeline(graphics: &GraphicsEngine) -> Self {
      let renderer = Renderer::new(graphics);

      MultiPassPipeline {
        renderer,
        passes: vec![Box::new(DepthPass {}), Box::new(ColorPass {})],
      }
    }
  }
}
