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

  fn material_create(&self, descriptor: &MaterialDescriptor) -> surreal::Result<MaterialId> {
    todo!()
  }

  fn material_set_uniform(&self, material_id: MaterialId, uniform_name: &str, value: &UniformValue) -> surreal::Result<()> {
    todo!()
  }

  fn material_get_uniform(&self, material_id: MaterialId, uniform_name: &str) -> surreal::Result<Option<UniformValue>> {
    todo!()
  }

  fn material_delete(&self, material_id: MaterialId) -> surreal::Result<()> {
    todo!()
  }

  fn texture_create(&self, descriptor: &TextureDescriptor) -> surreal::Result<TextureId> {
    todo!()
  }

  fn texture_read(&self, texture_id: TextureId) -> surreal::Result<Box<[u8]>> {
    todo!()
  }

  fn texture_write(&self, texture_id: TextureId, pixels: &[u8]) -> surreal::Result<()> {
    todo!()
  }

  fn texture_delete(&self, texture_id: TextureId) -> surreal::Result<()> {
    todo!()
  }

  fn render_target_create(&self, label: Option<&str>, size: (u32, u32), format: TextureFormat) -> surreal::Result<RenderTargetId> {
    todo!()
  }

  fn render_target_delete(&self, render_target_id: RenderTargetId) -> surreal::Result<()> {
    todo!()
  }
}
