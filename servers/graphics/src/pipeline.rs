//! Rendering pipeline abstractions.

use surreal::maths::Mat4;

use crate::{CommandBuffer, GraphicsServer};

#[cfg(feature = "hdrp")]
pub mod hdrp;
#[cfg(feature = "urp")]
pub mod urp;

/// A pipeline for a [`RenderManager`].
#[allow(unused_variables)]
pub trait RenderPipeline {
  fn begin_frame(&mut self, commands: &mut CommandBuffer) {}
  fn begin_camera(&mut self, commands: &mut CommandBuffer, camera: &dyn RenderCamera) {}
  fn render_camera(&mut self, commands: &mut CommandBuffer, camera: &dyn RenderCamera) {}
  fn end_camera(&mut self, commands: &mut CommandBuffer, camera: &dyn RenderCamera) {}
  fn end_frame(&mut self, commands: &mut CommandBuffer) {}
}

/// A single pass for the [`RenderManager`].
#[allow(unused_variables)]
pub trait RenderPass<P> {
  fn begin_frame(&mut self, commands: &mut CommandBuffer, pipeline: &mut P) {}
  fn begin_camera(&mut self, commands: &mut CommandBuffer, camera: &dyn RenderCamera, pipeline: &mut P) {}
  fn render_camera(&mut self, commands: &mut CommandBuffer, camera: &dyn RenderCamera, pipeline: &mut P) {}
  fn end_camera(&mut self, commands: &mut CommandBuffer, camera: &dyn RenderCamera, pipeline: &mut P) {}
  fn end_frame(&mut self, commands: &mut CommandBuffer, pipeline: &mut P) {}
}

/// A camera type that can be used in [`RenderManager`]s.
pub trait RenderCamera<'a> {
  /// Retrieves the projection matrix for the camera.
  fn projection_matrix(&self) -> &Mat4;

  /// Retrieves the view matrix for the camera.
  fn view_matrix(&self) -> &Mat4;
}

/// A manager for a [`RenderPipeline`]  that is composed of one or more [`RenderPass`]es.
pub struct RenderManager<'a, P: RenderPipeline> {
  graphics: GraphicsServer,
  command_buffer: CommandBuffer<'a>,
  pipeline: P,
  passes: Vec<Box<dyn RenderPass<P>>>,
}

impl<'a, P: RenderPipeline> RenderManager<'a, P> {
  /// Creates a new [`RenderManager`] on the given [`GraphicsServer`].
  pub fn new(label: &'a str, graphics: &GraphicsServer, pipeline: P, passes: Vec<Box<dyn RenderPass<P>>>) -> Self {
    Self {
      graphics: graphics.clone(),
      command_buffer: CommandBuffer::new(label),
      pipeline,
      passes,
    }
  }

  /// Adds a new [`RenderPass`] to the [`RenderManager`].
  pub fn add_pass(&mut self, pass: Box<dyn RenderPass<P>>) {
    self.passes.push(pass);
  }

  /// Inserts a new [`RenderPass`] into the [`RenderManager`].
  pub fn insert_pass(&mut self, index: usize, pass: Box<dyn RenderPass<P>>) {
    self.passes.insert(index, pass);
  }

  /// Begins rendering a new frame.
  pub fn begin_frame(&mut self) {
    self.pipeline.begin_frame(&mut self.command_buffer);

    for pass in &mut self.passes {
      pass.begin_frame(&mut self.command_buffer, &mut self.pipeline);
    }
  }

  /// Renders the given [`RenderCamera`] using the pipeline.
  pub fn render_camera(&mut self, camera: &dyn RenderCamera) {
    self.pipeline.begin_camera(&mut self.command_buffer, camera);

    for pass in &mut self.passes {
      self.pipeline.render_camera(&mut self.command_buffer, camera);

      pass.begin_camera(&mut self.command_buffer, camera, &mut self.pipeline);
      pass.render_camera(&mut self.command_buffer, camera, &mut self.pipeline);
      pass.end_camera(&mut self.command_buffer, camera, &mut self.pipeline);
    }

    self.pipeline.end_camera(&mut self.command_buffer, camera);
  }

  /// Ends rendering the current frame.
  pub fn end_frame(&mut self) -> surreal::Result<()> {
    for pass in &mut self.passes {
      pass.end_frame(&mut self.command_buffer, &mut self.pipeline);
    }

    self.pipeline.end_frame(&mut self.command_buffer);
    self.graphics.execute_commands(&mut self.command_buffer)
  }
}
