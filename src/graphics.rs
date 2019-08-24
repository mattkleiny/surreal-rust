//! A lightweight graphics system.

pub use buffers::*;
pub use materials::*;
pub use meshes::*;
pub use shaders::*;
pub use textures::*;
pub use primitives::*;

mod buffers;
mod materials;
mod meshes;
mod shaders;
mod textures;
mod primitives;

/// An abstraction over the graphics device for the system.
pub trait GraphicsDevice {
  fn clear(&mut self, color: Color);
  fn present(&mut self);
}

/// A pipeline of render passes for use in graphics rendering.
pub struct GraphicsPipeline {
  passes: Vec<Box<dyn RenderPass>>,
}

impl GraphicsPipeline {
  pub fn new() -> Self {
    Self {
      passes: Vec::new()
    }
  }

  /// Adds the given render pass to the pipeline.
  pub fn add<P: 'static + RenderPass>(&mut self, pass: P) {
    self.passes.push(Box::new(pass));
  }

  /// Executes the render pipeline.
  pub fn execute(&mut self) {
    for pass in self.passes.iter_mut() {
      pass.execute();
    }
  }
}

/// An individual render pass that constitutes a render pipeline
pub trait RenderPass {
  /// A render pass as composed in a graphics pipeline.
  fn execute(&mut self);
}

/// A queue used for issuing render commands to a pipeline.
pub trait RenderQueue {}

/// Permits rendering of a type by submitting it to a render queue.
pub trait Renderer {
  /// Renders the given element on the given pipeline.
  fn submit(&self, queue: &mut impl RenderQueue);
}

#[cfg(test)]
mod tests {
  use super::*;

  struct TestPass;

  impl RenderPass for TestPass {
    fn execute(&mut self) {}
  }

  #[test]
  fn it_should_execute_basic_render_passes() {
    let mut pipeline = GraphicsPipeline::new();
    pipeline.add(TestPass);
    pipeline.execute();
  }
}