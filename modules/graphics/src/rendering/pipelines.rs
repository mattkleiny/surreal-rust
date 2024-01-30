//! Render pipeline abstractions.

use common::Camera;

use super::*;

/// A frame of rendering.
pub struct RenderFrame<'a> {
  pub delta_time: f32,
  pub queue: &'a mut RenderQueue,
  pub renderer: &'a mut Renderer,
}

impl<'a> RenderFrame<'a> {
  /// Draws the object visible to the given camera.
  pub fn draw_camera(&mut self, scene: &dyn RenderScene, camera: &dyn Camera) {
    let visible_object_set = scene.cull_visible_objects(camera);

    for (material, group) in visible_object_set.group_by_material() {
      self.queue.set_material(material);

      for entry in group {
        entry.object.render(self)
      }
    }
  }
}

/// Represents a scene that can be rendered by a [`RenderPipeline`].
pub trait RenderScene {
  /// Gets the cameras that should be used to render this scene.
  fn cameras(&self) -> Vec<&dyn Camera>;

  /// Gets the objects that should be rendered by the given camera.
  fn cull_visible_objects(&self, camera: &dyn Camera) -> VisibleObjectSet;
}

/// Represents an object capable of being rendered.
pub trait RenderObject {
  /// Renders the object to the given [`Renderer`].
  fn render(&self, frame: &mut RenderFrame<'_>);
}

/// Represents a pipeline capable of rendering a scene.
///
/// A pipeline is a collection of passes that are executed in order to render a
/// scene. Each pass is responsible for rendering a specific set of objects.
pub trait RenderPipeline {
  /// Renders the given scene.
  fn render(&mut self, scene: &dyn RenderScene, delta_time: f32);
}

/// A [`RenderPipeline`] that executes many [`RenderPass`]es in order.
pub struct MultiPassPipeline {
  renderer: Renderer,
  queue: RenderQueue,
  passes: Vec<Box<dyn RenderPass>>,
}

impl RenderPipeline for MultiPassPipeline {
  fn render(&mut self, scene: &dyn RenderScene, delta_time: f32) {
    let mut frame = RenderFrame {
      delta_time,
      queue: &mut self.queue,
      renderer: &mut self.renderer,
    };

    // begin the frame
    for pass in &self.passes {
      pass.begin_frame(scene, &mut frame);
    }

    // render each camera
    for camera in scene.cameras() {
      for pass in &self.passes {
        pass.begin_camera(scene, camera, &mut frame);
      }

      for pass in &self.passes {
        pass.render_camera(scene, camera, &mut frame);
      }

      for pass in &self.passes {
        pass.end_camera(scene, camera, &mut frame);
      }
    }

    // finalize the frame
    for pass in &self.passes {
      pass.end_frame(scene, &mut frame);
    }
  }
}

/// A single pass of a [`MultiPassPipeline`].
#[allow(unused_variables)]
pub trait RenderPass {
  fn begin_frame(&self, scene: &dyn RenderScene, frame: &mut RenderFrame<'_>) {}
  fn begin_camera(&self, scene: &dyn RenderScene, camera: &dyn Camera, frame: &mut RenderFrame<'_>) {}
  fn render_camera(&self, scene: &dyn RenderScene, camera: &dyn Camera, frame: &mut RenderFrame<'_>) {}
  fn end_camera(&self, scene: &dyn RenderScene, camera: &dyn Camera, frame: &mut RenderFrame<'_>) {}
  fn end_frame(&self, scene: &dyn RenderScene, frame: &mut RenderFrame<'_>) {}
}

pub mod forward {
  use super::*;

  /// A [`RenderPass`] that renders all objects in the scene to a depth target.
  struct DepthPass {}

  impl RenderPass for DepthPass {
    fn render_camera(&self, scene: &dyn RenderScene, camera: &dyn Camera, frame: &mut RenderFrame<'_>) {
      frame.queue.clear_color_buffer(Color::BLACK);
      frame.draw_camera(scene, camera);
    }
  }

  /// A [`RenderPass`] that renders all objects in the scene to a color target.
  struct ColorPass {
    color_target: RenderTarget,
  }

  impl RenderPass for ColorPass {
    fn begin_frame(&self, _scene: &dyn RenderScene, frame: &mut RenderFrame<'_>) {
      frame.queue.set_render_target(&self.color_target);
    }

    fn render_camera(&self, scene: &dyn RenderScene, camera: &dyn Camera, frame: &mut RenderFrame<'_>) {
      frame.queue.clear_color_buffer(Color::BLACK);
      frame.draw_camera(scene, camera);
    }

    fn end_frame(&self, _scene: &dyn RenderScene, frame: &mut RenderFrame<'_>) {
      frame.queue.blit_render_target_to_display(&self.color_target, None);
    }
  }

  impl MultiPassPipeline {
    /// Builds a new [`MultiPassPipeline`] for forward rendering.
    pub fn new_forward_pipeline(graphics: &GraphicsEngine) -> Self {
      MultiPassPipeline {
        renderer: Renderer::new(graphics),
        queue: RenderQueue::default(),
        passes: vec![
          Box::new(DepthPass {}),
          Box::new(ColorPass {
            color_target: RenderTarget::new(graphics, &RenderTargetDescriptor {
              color_attachment: RenderTextureDescriptor {
                width: 1920,
                height: 1080,
                options: TextureOptions::default(),
              },
              depth_attachment: None,
              stencil_attachment: None,
            })
            .unwrap(),
          }),
        ],
      }
    }
  }
}
