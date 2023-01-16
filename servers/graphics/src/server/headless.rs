//! Headless graphics support for the engine.

use surreal::graphics::TextureFormat;
use winit::dpi::PhysicalSize;

use super::*;

/// A headless, no-op [`GraphicsBackend`].
#[derive(Default)]
pub struct HeadlessGraphicsBackend {}

#[allow(unused_variables)]
impl GraphicsBackend for HeadlessGraphicsBackend {
  fn execute_commands(&self, commands: &mut CommandBuffer) -> surreal::Result<()> {
    Ok(()) // no-op
  }

  fn resize_viewport(&self, new_size: PhysicalSize<u32>) -> surreal::Result<()> {
    Ok(()) // no-op
  }

  fn shader_create(&self, descriptor: &ShaderDescriptor) -> surreal::Result<ShaderId> {
    todo!()
  }

  fn shader_delete(&self, shader_id: ShaderId) -> surreal::Result<()> {
    todo!()
  }

  fn material_create(&self, descriptor: &MaterialDescriptor) -> surreal::Result<MaterialId> {
    todo!()
  }

  fn material_delete(&self, material_id: MaterialId) -> surreal::Result<()> {
    todo!()
  }

  fn texture_create(&self, descriptor: &TextureDescriptor) -> surreal::Result<TextureId> {
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
