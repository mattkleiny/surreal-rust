//! A headless graphics backend for testing and etc.

use std::sync::atomic::{AtomicU32, Ordering};

use super::*;
use crate::maths::Rectangle;

/// Creates a [`GraphicsServer`] from the [`HeadlessGraphicsBackend`] for
/// testing purposes.
pub fn create_test_graphics() -> GraphicsServer {
  GraphicsServer::new(HeadlessGraphicsBackend::new())
}

/// A headless [`GraphicsBackend`] implementation.
///
/// This backend does nothing (no-ops) and can be used for testing/etc.
pub struct HeadlessGraphicsBackend {
  next_buffer_id: AtomicU32,
  next_texture_id: AtomicU32,
  next_shader_id: AtomicU32,
  next_mesh_id: AtomicU32,
  next_render_target_id: AtomicU32,
}

impl HeadlessGraphicsBackend {
  pub fn new() -> Self {
    Self {
      next_buffer_id: AtomicU32::new(1),
      next_texture_id: AtomicU32::new(1),
      next_shader_id: AtomicU32::new(1),
      next_mesh_id: AtomicU32::new(1),
      next_render_target_id: AtomicU32::new(1),
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

  fn set_viewport_size(&self, size: winit::dpi::PhysicalSize<u32>) {
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

  fn buffer_create(&self) -> GraphicsHandle {
    self.next_buffer_id.fetch_add(1, Ordering::Relaxed)
  }

  fn buffer_read_data(&self, buffer: GraphicsHandle, offset: usize, length: usize, pointer: *mut u8) {
    // no-op
  }

  fn buffer_write_data(&self, buffer: GraphicsHandle, usage: BufferUsage, kind: BufferKind, length: usize, pointer: *const u8) {
    // no-op
  }

  fn buffer_delete(&self, buffer: GraphicsHandle) {
    // no-op
  }

  fn texture_create(&self, sampler: &TextureSampler) -> GraphicsHandle {
    self.next_texture_id.fetch_add(1, Ordering::Relaxed)
  }

  fn texture_set_options(&self, texture: GraphicsHandle, sampler: &TextureSampler) {
    // no-op
  }

  fn texture_initialize(&self, texture: GraphicsHandle, width: u32, height: u32, format: TextureFormat) {
    // no-op
  }

  fn texture_read_data(&self, texture: GraphicsHandle, length: usize, pixel_format: TextureFormat, pixels: *mut u8, mip_level: usize) {
    // no-op
  }

  fn texture_write_data(
    &self,
    texture: GraphicsHandle,
    width: u32,
    height: u32,
    pixels: *const u8,
    internal_format: TextureFormat,
    pixel_format: TextureFormat,
    mip_level: usize,
  ) {
    // no-op
  }

  fn texture_write_sub_data(
    &self,
    texture: GraphicsHandle,
    region: &Rectangle,
    pixels: *const u8,
    pixel_format: TextureFormat,
    mip_level: usize,
  ) {
    // no-op
  }

  fn texture_delete(&self, texture: GraphicsHandle) {
    // no-op
  }

  fn shader_create(&self) -> GraphicsHandle {
    self.next_shader_id.fetch_add(1, Ordering::Relaxed)
  }

  fn shader_link(&self, shader: GraphicsHandle, kernels: &[ShaderKernel]) -> crate::Result<()> {
    Ok(())
  }

  fn shader_uniform_location(&self, shader: GraphicsHandle, name: &str) -> Option<usize> {
    None
  }

  fn shader_set_uniform(&self, shader: GraphicsHandle, location: usize, value: &ShaderUniform) {
    // no-op
  }

  fn shader_activate(&self, shader: GraphicsHandle) {
    // no-op
  }

  fn shader_delete(&self, shader: GraphicsHandle) {
    // no-op
  }

  fn mesh_create(&self, vertices: GraphicsHandle, indices: GraphicsHandle, descriptors: &[VertexDescriptor]) -> GraphicsHandle {
    self.next_mesh_id.fetch_add(1, Ordering::Relaxed)
  }

  fn mesh_draw(&self, mesh: GraphicsHandle, topology: PrimitiveTopology, vertex_count: usize, index_count: usize) {
    // no-op
  }

  fn mesh_delete(&self, mesh: GraphicsHandle) {
    // no-op
  }

  fn target_create(
    &self,
    color_attachment: GraphicsHandle,
    depth_attachment: Option<GraphicsHandle>,
    stencil_attachment: Option<GraphicsHandle>,
  ) -> GraphicsHandle {
    self.next_render_target_id.fetch_add(1, Ordering::Relaxed)
  }

  fn target_activate(&self, render_target: GraphicsHandle) {
    // no-op
  }

  fn target_set_default(&self) {
    // no-op
  }

  fn target_blit(&self, from: GraphicsHandle, to: GraphicsHandle, source_rect: &Rectangle, dest_rect: &Rectangle, filter: TextureFilter) {
    // no-op
  }

  fn target_blit_to_display(&self, handle: GraphicsHandle, source_rect: &Rectangle, dest_rect: &Rectangle, filter: TextureFilter) {
    // no-op
  }

  fn target_delete(&self, render_target: GraphicsHandle) {
    // no-op
  }
}
