//! Render pipeline abstractions.

use common::Camera;

use super::*;

/// A frame of rendering.
pub struct RenderFrame<'a> {
  pub delta_time: f32,
  pub queue: &'a mut RenderQueue,
  pub renderer: &'a mut Renderer,
}

/// Represents a scene that can be rendered by a [`RenderPipeline`].
pub trait RenderScene {
  /// Gets the cameras that should be used to render this scene.
  fn cameras(&self) -> Vec<&dyn Camera>;

  /// Gets the object with the given identifier.
  fn get_object(&self, identifier: u64) -> Option<&dyn RenderObject>;

  /// Gets the objects that should be rendered by the given camera.
  fn cull_visible_objects(&self, camera: &dyn Camera) -> VisibleObjectSet<u64>;
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
      let visible_object_set = scene.cull_visible_objects(camera);

      frame.queue.clear_color_buffer(Color::BLACK);

      for _object in visible_object_set.objects {}
    }
  }

  /// A [`RenderPass`] that renders all objects in the scene to a color target.
  struct ColorPass {}

  impl RenderPass for ColorPass {
    fn render_camera(&self, scene: &dyn RenderScene, camera: &dyn Camera, frame: &mut RenderFrame<'_>) {
      let visible_object_set = scene.cull_visible_objects(camera);

      frame.queue.clear_color_buffer(Color::BLACK);

      for (material, group) in visible_object_set.group_by_material() {
        frame.queue.set_material(material);

        for object in group {
          let object = scene.get_object(object.identifier).unwrap();

          object.render(frame);
        }
      }
    }

    fn end_frame(&self, _scene: &dyn RenderScene, _frame: &mut RenderFrame<'_>) {
      // TODO: blit the color target to the screen.
    }
  }

  impl MultiPassPipeline {
    /// Builds a new [`MultiPassPipeline`] for forward rendering.
    pub fn new_forward_pipeline(graphics: &GraphicsEngine) -> Self {
      MultiPassPipeline {
        renderer: Renderer::new(graphics),
        queue: RenderQueue::default(),
        passes: vec![Box::new(DepthPass {}), Box::new(ColorPass {})],
      }
    }
  }
}
