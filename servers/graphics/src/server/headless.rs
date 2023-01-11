//! Headless graphics support for the engine.

use winit::dpi::PhysicalSize;

use super::*;

/// A headless, no-op [`GraphicsServerBackend`].
#[derive(Default)]
pub struct HeadlessBackend {}

#[allow(unused_variables)]
impl GraphicsServerBackend for HeadlessBackend {
  fn begin_frame(&self, color: Color) -> surreal::Result<()> {
    todo!()
  }

  fn end_frame(&self) -> surreal::Result<()> {
    todo!()
  }

  fn resize_viewport(&self, new_size: PhysicalSize<u32>) -> surreal::Result<()> {
    todo!()
  }

  fn shader_create(&self, name: Option<&str>) -> surreal::Result<ShaderId> {
    todo!()
  }

  fn shader_set_code(&self, shader_id: ShaderId, code: &str) -> surreal::Result<()> {
    todo!()
  }

  fn shader_delete(&self, shader_id: ShaderId) -> surreal::Result<()> {
    todo!()
  }
}
