//! Headless graphics support for the engine.

use winit::dpi::PhysicalSize;

use super::*;

/// A headless, no-op [`GraphicsBackend`].
#[derive(Default)]
pub struct HeadlessBackend {}

#[allow(unused_variables)]
impl GraphicsBackend for HeadlessBackend {
  fn execute_commands(&self, commands: &mut CommandBuffer) -> surreal::Result<()> {
    Ok(()) // no-op
  }

  fn resize_viewport(&self, new_size: PhysicalSize<u32>) -> surreal::Result<()> {
    Ok(()) // no-op
  }

  fn texture_create_1d(&self, label: Option<&str>, size: u32, format: TextureFormat) -> surreal::Result<TextureId> {
    todo!()
  }

  fn texture_create_2d(&self, label: Option<&str>, size: UVec2, format: TextureFormat) -> surreal::Result<TextureId> {
    todo!()
  }

  fn texture_create_3d(&self, label: Option<&str>, size: UVec3, format: TextureFormat) -> surreal::Result<TextureId> {
    todo!()
  }

  fn texture_write(&self, texture_id: TextureId, pixels: &[u8]) -> surreal::Result<()> {
    todo!()
  }

  fn texture_read(&self, texture_id: TextureId) -> surreal::Result<Vec<u8>> {
    todo!()
  }

  fn texture_delete(&self, texture_id: TextureId) -> surreal::Result<()> {
    todo!()
  }

  fn target_create(&self, label: Option<&str>, size: UVec2, format: TextureFormat) -> surreal::Result<RenderTargetId> {
    todo!()
  }

  fn target_delete(&self, render_target_id: RenderTargetId) -> surreal::Result<()> {
    todo!()
  }
}
