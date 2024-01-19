//! A headless graphics backend for testing and etc.

use std::sync::atomic::{AtomicU32, Ordering};

use common::maths::{Rectangle, UVec2};

use super::*;

/// A headless [`GraphicsBackend`] implementation.
///
/// This backend does nothing (no-ops) and can be used for testing/etc.
pub struct HeadlessGraphicsBackend {
  next_buffer_id: AtomicU32,
  next_texture_id: AtomicU32,
  next_shader_id: AtomicU32,
  next_mesh_id: AtomicU32,
  next_target_id: AtomicU32,
}

impl Default for HeadlessGraphicsBackend {
  fn default() -> Self {
    Self {
      next_buffer_id: AtomicU32::new(1),
      next_texture_id: AtomicU32::new(1),
      next_shader_id: AtomicU32::new(1),
      next_mesh_id: AtomicU32::new(1),
      next_target_id: AtomicU32::new(1),
    }
  }
}

#[allow(unused_variables)]
impl GraphicsBackend for HeadlessGraphicsBackend {
  fn begin_frame(&self) {
    // no-op
  }

  fn end_frame(&self) {
    // no-op
  }

  fn clear_color_buffer(&self, color: Color) {
    // no-op
  }

  fn clear_depth_buffer(&self) {
    // no-op
  }

  fn viewport_size(&self) -> (usize, usize) {
    (1920, 1080)
  }

  fn set_viewport_size(&self, size: UVec2) {
    // no-op
  }

  fn set_blend_state(&self, blend_state: BlendState) {
    // no-op
  }

  fn set_culling_mode(&self, culling_mode: CullingMode) {
    // no-op
  }

  fn set_scissor_mode(&self, scissor_mode: ScissorMode) {
    // no-op
  }

  fn buffer_create(&self) -> Result<BufferId, BufferError> {
    Ok(BufferId::from(self.next_buffer_id.fetch_add(1, Ordering::Relaxed)))
  }

  fn buffer_read_data(
    &self,
    buffer: BufferId,
    offset: usize,
    length: usize,
    pointer: *mut u8,
  ) -> Result<(), BufferError> {
    Ok(())
  }

  fn buffer_write_data(
    &self,
    buffer: BufferId,
    usage: BufferUsage,
    kind: BufferKind,
    length: usize,
    pointer: *const u8,
  ) -> Result<(), BufferError> {
    Ok(())
  }

  fn buffer_delete(&self, buffer: BufferId) -> Result<(), BufferError> {
    Ok(())
  }

  fn texture_create(&self, sampler: &TextureSampler) -> Result<TextureId, TextureError> {
    Ok(TextureId::from(self.next_texture_id.fetch_add(1, Ordering::Relaxed)))
  }

  fn texture_set_options(&self, texture: TextureId, sampler: &TextureSampler) -> Result<(), TextureError> {
    Ok(())
  }

  fn texture_initialize(
    &self,
    texture: TextureId,
    width: u32,
    height: u32,
    format: TextureFormat,
  ) -> Result<(), TextureError> {
    Ok(())
  }

  fn texture_read_data(
    &self,
    texture: TextureId,
    length: usize,
    pixel_format: TextureFormat,
    pixels: *mut u8,
    mip_level: usize,
  ) -> Result<(), TextureError> {
    Ok(())
  }

  fn texture_write_data(
    &self,
    texture: TextureId,
    width: u32,
    height: u32,
    pixels: *const u8,
    internal_format: TextureFormat,
    pixel_format: TextureFormat,
    mip_level: usize,
  ) -> Result<(), TextureError> {
    Ok(())
  }

  fn texture_write_sub_data(
    &self,
    texture: TextureId,
    region: &Rectangle,
    pixels: *const u8,
    pixel_format: TextureFormat,
    mip_level: usize,
  ) -> Result<(), TextureError> {
    Ok(())
  }

  fn texture_delete(&self, texture: TextureId) -> Result<(), TextureError> {
    Ok(())
  }

  fn shader_create(&self) -> Result<ShaderId, ShaderError> {
    Ok(ShaderId::from(self.next_shader_id.fetch_add(1, Ordering::Relaxed)))
  }

  fn shader_link(&self, shader: ShaderId, kernels: &[ShaderKernel]) -> Result<(), ShaderError> {
    Ok(())
  }

  fn shader_uniform_location(&self, shader: ShaderId, name: &str) -> Option<usize> {
    None
  }

  fn shader_set_uniform(&self, shader: ShaderId, location: usize, value: &ShaderUniform) -> Result<(), ShaderError> {
    Ok(())
  }

  fn shader_activate(&self, shader: ShaderId) -> Result<(), ShaderError> {
    Ok(())
  }

  fn shader_delete(&self, shader: ShaderId) -> Result<(), ShaderError> {
    Ok(())
  }

  fn mesh_create(
    &self,
    vertices: BufferId,
    indices: BufferId,
    descriptors: &[VertexDescriptor],
  ) -> Result<MeshId, MeshError> {
    Ok(MeshId::from(self.next_mesh_id.fetch_add(1, Ordering::Relaxed)))
  }

  fn mesh_draw(
    &self,
    mesh: MeshId,
    topology: PrimitiveTopology,
    vertex_count: usize,
    index_count: usize,
  ) -> Result<(), MeshError> {
    Ok(())
  }

  fn mesh_delete(&self, mesh: MeshId) -> Result<(), MeshError> {
    Ok(())
  }

  fn target_create(
    &self,
    color_attachment: TextureId,
    depth_attachment: Option<TextureId>,
    stencil_attachment: Option<TextureId>,
  ) -> Result<TargetId, TargetError> {
    Ok(TargetId::from(self.next_target_id.fetch_add(1, Ordering::Relaxed)))
  }

  fn target_activate(&self, target: TargetId) -> Result<(), TargetError> {
    Ok(())
  }

  fn target_set_default(&self) -> Result<(), TargetError> {
    Ok(())
  }

  fn target_blit(
    &self,
    from: TargetId,
    to: TargetId,
    source_rect: &Rectangle,
    dest_rect: &Rectangle,
    filter: TextureFilter,
  ) -> Result<(), TargetError> {
    Ok(())
  }

  fn target_blit_to_display(
    &self,
    target: TargetId,
    source_rect: &Rectangle,
    dest_rect: &Rectangle,
    filter: TextureFilter,
  ) -> Result<(), TargetError> {
    Ok(())
  }

  fn target_delete(&self, target: TargetId) -> Result<(), TargetError> {
    Ok(())
  }
}
