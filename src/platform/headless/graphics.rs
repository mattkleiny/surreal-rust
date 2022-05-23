use std::sync::atomic::{AtomicU32, Ordering};

use crate::graphics::*;

/// The graphics server for the headless platform.
pub struct HeadlessGraphics {
  next_buffer_id: AtomicU32,
  next_texture_id: AtomicU32,
  next_shader_id: AtomicU32,
  next_mesh_id: AtomicU32,
}

impl HeadlessGraphics {
  pub fn new() -> GraphicsServer<Self> {
    GraphicsServer::new(Self {
      next_buffer_id: AtomicU32::new(0),
      next_texture_id: AtomicU32::new(0),
      next_shader_id: AtomicU32::new(0),
      next_mesh_id: AtomicU32::new(0),
    })
  }
}

impl GraphicsImpl for HeadlessGraphics {
  type Handle = u32;

  fn begin_frame(&self) {
    // no-op
  }

  fn end_frame(&self) {
    // no-op
  }

  fn set_viewport_size(&self, _viewport: (usize, usize)) {
    // no-op
  }

  fn set_blend_state(&self, _blend_state: BlendState) {
    // no-op
  }

  fn clear_color_buffer(&self, _color: Color) {
    // no-op
  }

  fn clear_depth_buffer(&self) {
    // no-op
  }

  fn flush_commands(&self) {
    // no-op
  }

  fn create_buffer(&self) -> Self::Handle {
    self.next_buffer_id.fetch_add(1, Ordering::Relaxed)
  }

  fn read_buffer_data(&self, _buffer: Self::Handle, _kind: BufferKind, _offset: usize, _length: usize) -> Vec<u8> {
    Vec::new()
  }

  fn write_buffer_data(&self, _buffer: Self::Handle, _usage: BufferUsage, _kind: BufferKind, _data: *const u8, _length: usize) {
    // no-op
  }

  fn delete_buffer(&self, _buffer: Self::Handle) {
    // no-op
  }

  fn create_texture(&self, _sampler: &TextureSampler) -> Self::Handle {
    self.next_texture_id.fetch_add(1, Ordering::Relaxed)
  }

  fn write_texture_data(&self, _texture: Self::Handle, _width: usize, _height: usize, _pixels: *const u8, _format: TextureFormat, _mip_level: usize) {
    // no-op
  }

  fn delete_texture(&self, _texture: Self::Handle) {
    // no-op
  }

  fn create_shader(&self) -> Self::Handle {
    self.next_shader_id.fetch_add(1, Ordering::Relaxed)
  }

  fn link_shaders(&self, _shader: Self::Handle, _shaders: Vec<Shader>) -> GraphicsResult<()> {
    Ok(())
  }

  fn get_shader_uniform_location(&self, _shader: Self::Handle, _name: &str) -> Option<usize> {
    None
  }

  fn set_shader_uniform(&self, _shader: Self::Handle, _location: usize, _value: &ShaderUniform<Self>) {
    // no-op
  }

  fn set_active_shader(&self, _shader: Self::Handle) {
    // no-op
  }

  fn delete_shader(&self, _shader: Self::Handle) {
    // no-op
  }

  fn create_mesh(&self, _vertices: Self::Handle, _indices: Self::Handle, _descriptors: &[VertexDescriptor]) -> Self::Handle {
    self.next_mesh_id.fetch_add(1, Ordering::Relaxed)
  }

  fn draw_mesh(&self, _mesh: Self::Handle, _topology: PrimitiveTopology, _vertex_count: usize, _index_count: usize) {
    // no-op
  }

  fn delete_mesh(&self, _mesh: Self::Handle) {
    // no-op
  }
}