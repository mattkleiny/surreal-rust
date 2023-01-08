//! A headless graphics backend for testing and etc.

use std::sync::atomic::{AtomicU32, Ordering};

use crate::maths::Rectangle;

use super::*;

/// Creates a [`GraphicsServer`] from the [`HeadlessGraphicsBackend`] for testing purposes.
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

impl GraphicsBackend for HeadlessGraphicsBackend {
  fn begin_frame(&self) {
    // no-op
  }

  fn end_frame(&self) {
    // no-op
  }

  fn get_viewport_size(&self) -> (usize, usize) {
    (1920, 1080)
  }

  fn set_viewport_size(&self, _size: winit::dpi::PhysicalSize<u32>) {
    // no-op
  }

  fn set_blend_state(&self, _blend_state: BlendState) {
    // no-op
  }

  fn set_culling_mode(&self, _culling_mode: CullingMode) {
    // no-op
  }

  fn set_scissor_mode(&self, _scissor_mode: ScissorMode) {
    // no-op
  }

  fn clear_color_buffer(&self, _color: Color) {
    // no-op
  }

  fn clear_depth_buffer(&self) {
    // no-op
  }

  fn create_buffer(&self) -> GraphicsHandle {
    self.next_buffer_id.fetch_add(1, Ordering::Relaxed)
  }

  fn read_buffer_data(&self, _buffer: GraphicsHandle, _offset: usize, _length: usize, _pointer: *mut u8) {
    // no-op
  }

  fn write_buffer_data(&self, _buffer: GraphicsHandle, _usage: BufferUsage, _kind: BufferKind, _length: usize, _pointer: *const u8) {
    // no-op
  }

  fn delete_buffer(&self, _buffer: GraphicsHandle) {
    // no-op
  }

  fn create_texture(&self, _sampler: &TextureSampler) -> GraphicsHandle {
    self.next_texture_id.fetch_add(1, Ordering::Relaxed)
  }

  fn set_texture_options(&self, _texture: GraphicsHandle, _sampler: &TextureSampler) {
    // no-op
  }

  fn initialize_texture(&self, _texture: GraphicsHandle, _width: u32, _height: u32, _format: TextureFormat) {
    // no-op
  }

  fn read_texture_data(&self, _texture: GraphicsHandle, _length: usize, _pixel_format: TextureFormat, _pixels: *mut u8, _mip_level: usize) {
    // no-op
  }

  fn write_texture_data(
    &self,
    _texture: GraphicsHandle,
    _width: u32,
    _height: u32,
    _pixels: *const u8,
    _internal_format: TextureFormat,
    _pixel_format: TextureFormat,
    _mip_level: usize,
  ) {
    // no-op
  }

  fn write_texture_sub_data(
    &self,
    _texture: GraphicsHandle,
    _region: &Rectangle,
    _pixels: *const u8,
    _pixel_format: TextureFormat,
    _mip_level: usize,
  ) {
    // no-op
  }

  fn delete_texture(&self, _texture: GraphicsHandle) {
    // no-op
  }

  fn create_shader(&self) -> GraphicsHandle {
    self.next_shader_id.fetch_add(1, Ordering::Relaxed)
  }

  fn link_shaders(&self, _shader: GraphicsHandle, _shaders: &[Shader]) -> crate::Result<()> {
    Ok(())
  }

  fn get_shader_uniform_location(&self, _shader: GraphicsHandle, _name: &str) -> Option<usize> {
    None
  }

  fn set_shader_uniform(&self, _shader: GraphicsHandle, _location: usize, _value: &ShaderUniform) {
    // no-op
  }

  fn set_active_shader(&self, _shader: GraphicsHandle) {
    // no-op
  }

  fn delete_shader(&self, _shader: GraphicsHandle) {
    // no-op
  }

  fn dispatch_compute(&self, _shader: GraphicsHandle, _x: u32, _y: u32, _z: u32) {
    // no-op
  }

  fn wait_compute_barrier(&self, _barrier: GraphicsBarrier) {
    // no-op
  }

  fn create_mesh(&self, _vertices: GraphicsHandle, _indices: GraphicsHandle, _descriptors: &[VertexDescriptor]) -> GraphicsHandle {
    self.next_mesh_id.fetch_add(1, Ordering::Relaxed)
  }

  fn draw_mesh(&self, _mesh: GraphicsHandle, _topology: PrimitiveTopology, _vertex_count: usize, _index_count: usize) {
    // no-op
  }

  fn delete_mesh(&self, _mesh: GraphicsHandle) {
    // no-op
  }

  fn create_render_target(
    &self,
    _color_attachment: GraphicsHandle,
    _depth_attachment: Option<GraphicsHandle>,
    _stencil_attachment: Option<GraphicsHandle>,
  ) -> GraphicsHandle {
    self.next_render_target_id.fetch_add(1, Ordering::Relaxed)
  }

  fn set_active_render_target(&self, _render_target: GraphicsHandle) {
    // no-op
  }

  fn set_default_render_target(&self) {
    // no-op
  }

  fn blit_render_target(
    &self,
    _from: GraphicsHandle,
    _to: GraphicsHandle,
    _source_rect: &Rectangle,
    _dest_rect: &Rectangle,
    _filter: TextureFilter,
  ) {
    // no-op
  }

  fn blit_render_target_to_display(
    &self,
    _handle: GraphicsHandle,
    _source_rect: &Rectangle,
    _dest_rect: &Rectangle,
    _filter: TextureFilter,
  ) {
    // no-op
  }

  fn delete_render_target(&self, _render_target: GraphicsHandle) {
    // no-op
  }
}
