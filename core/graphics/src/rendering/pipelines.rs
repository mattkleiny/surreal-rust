//! Render pipeline abstractions.

use common::{profile_frame_end, profile_frame_start, Camera, StackAllocator};
use macros::profiling;

use super::*;

const FRAME_STACK_SIZE: usize = 1024 * 1024;

/// A frame of rendering.
pub struct RenderFrame<'a> {
  pub delta_time: f32,
  pub queue: &'a mut RenderQueue,
  pub allocator: StackAllocator<FRAME_STACK_SIZE>,
}

/// A scene that can be rendered.
pub trait RenderScene {
  /// The camera type used by this scene.
  type Camera: ?Sized + Camera = dyn Camera;

  /// Gets the cameras in the scene.
  fn cameras(&self) -> Vec<&Self::Camera>;
}

/// Represents a pipeline capable of rendering a scene.
///
/// A pipeline is a collection of passes that are executed in order to render a
/// scene. Each pass is responsible for rendering a specific set of objects.
pub trait RenderPipeline<S: RenderScene> {
  fn render(&mut self, scene: &S, delta_time: f32);
}

/// A single pass of a [`MultiPassPipeline`].
#[allow(unused_variables)]
pub trait RenderPass<S: RenderScene> {
  fn begin_frame(&mut self, scene: &S, frame: &mut RenderFrame<'_>) {}
  fn begin_camera(&mut self, scene: &S, camera: &S::Camera, frame: &mut RenderFrame<'_>) {}
  fn render_camera(&mut self, scene: &S, camera: &S::Camera, frame: &mut RenderFrame<'_>) {}
  fn end_camera(&mut self, scene: &S, camera: &S::Camera, frame: &mut RenderFrame<'_>) {}
  fn end_frame(&mut self, scene: &S, frame: &mut RenderFrame<'_>) {}
}

/// A [`RenderPipeline`] that executes many [`RenderPass`]es in order.
pub struct MultiPassPipeline<S> {
  queue: RenderQueue,
  passes: Vec<Box<dyn RenderPass<S>>>,
}

impl<S: RenderScene> MultiPassPipeline<S> {
  /// Creates a new [`MultiPassPipeline`] with the given passes.
  pub fn new() -> Self {
    Self {
      queue: RenderQueue::default(),
      passes: Vec::default(),
    }
  }

  /// Adds a pass to the pipeline.
  pub fn with_pass(mut self, pass: impl RenderPass<S> + 'static) -> Self {
    self.passes.push(Box::new(pass));
    self
  }
}

impl<S: RenderScene> RenderPipeline<S> for MultiPassPipeline<S> {
  #[profiling]
  fn render(&mut self, scene: &S, delta_time: f32) {
    profile_frame_start!();

    let mut frame = RenderFrame {
      delta_time,
      queue: &mut self.queue,
      allocator: StackAllocator::new(),
    };

    // begin the frame
    for pass in &mut self.passes {
      pass.begin_frame(scene, &mut frame);
    }

    // render each camera
    for camera in scene.cameras() {
      for pass in &mut self.passes {
        pass.begin_camera(scene, camera, &mut frame);
      }

      for pass in &mut self.passes {
        pass.render_camera(scene, camera, &mut frame);
      }

      for pass in &mut self.passes {
        pass.end_camera(scene, camera, &mut frame);
      }
    }

    // finalize the frame
    for pass in &mut self.passes {
      pass.end_frame(scene, &mut frame);
    }

    frame.queue.flush().unwrap();

    profile_frame_end!();
  }
}
