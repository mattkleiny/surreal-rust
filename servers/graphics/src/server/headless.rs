//! Headless graphics support for the engine.

use winit::dpi::PhysicalSize;

use super::*;

/// A headless, no-op [`GraphicsServerBackend`].
#[derive(Default)]
pub struct HeadlessBackend {}

#[allow(unused_variables)]
impl GraphicsServerBackend for HeadlessBackend {
  fn execute_commands(&self, commands: &mut CommandBuffer) -> surreal::Result<()> {
    Ok(()) // no-op
  }

  fn resize_viewport(&self, new_size: PhysicalSize<u32>) -> surreal::Result<()> {
    Ok(()) // no-op
  }
}
