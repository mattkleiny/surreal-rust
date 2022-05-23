use std::sync::atomic::{AtomicU32, Ordering};

use crate::graphics::{BlendState, BufferKind, BufferUsage, Color, GraphicsHandle, GraphicsResult, GraphicsServerImpl, PrimitiveTopology, Shader, ShaderUniform, TextureFormat, TextureSampler, VertexDescriptor};

/// The graphics server for the headless platform.
pub struct HeadlessGraphicsServer {
  next_buffer_id: AtomicU32,
  next_texture_id: AtomicU32,
  next_shader_id: AtomicU32,
  next_mesh_id: AtomicU32,
}

impl HeadlessGraphicsServer {
  pub fn new() -> Self {
    Self {
      next_buffer_id: AtomicU32::new(0),
      next_texture_id: AtomicU32::new(0),
      next_shader_id: AtomicU32::new(0),
      next_mesh_id: AtomicU32::new(0),
    }
  }
}

impl GraphicsServerImpl for HeadlessGraphicsServer {
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

  fn create_buffer(&self) -> GraphicsHandle {
    GraphicsHandle { id: self.next_buffer_id.fetch_add(1, Ordering::Relaxed) }
  }

  fn read_buffer_data(&self, _buffer: GraphicsHandle, _kind: BufferKind, _offset: usize, _length: usize) -> Vec<u8> {
    Vec::new()
  }

  fn write_buffer_data(&self, _buffer: GraphicsHandle, _usage: BufferUsage, _kind: BufferKind, _data: *const u8, _length: usize) {
    // no-op
  }

  fn delete_buffer(&self, _buffer: GraphicsHandle) {
    // no-op
  }

  fn create_texture(&self, _sampler: &TextureSampler) -> GraphicsHandle {
    GraphicsHandle { id: self.next_texture_id.fetch_add(1, Ordering::Relaxed) }
  }

  fn write_texture_data(&self, _texture: GraphicsHandle, _width: usize, _height: usize, _pixels: *const u8, _format: TextureFormat, _mip_level: usize) {
    // no-op
  }

  fn delete_texture(&self, _texture: GraphicsHandle) {
    // no-op
  }

  fn create_shader(&self) -> GraphicsHandle {
    GraphicsHandle { id: self.next_shader_id.fetch_add(1, Ordering::Relaxed) }
  }

  fn link_shaders(&self, _shader: GraphicsHandle, _shaders: Vec<Shader>) -> GraphicsResult<()> {
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

  fn create_mesh(&self, _vertices: GraphicsHandle, _indices: GraphicsHandle, _descriptors: &[VertexDescriptor]) -> GraphicsHandle {
    GraphicsHandle { id: self.next_mesh_id.fetch_add(1, Ordering::Relaxed) }
  }

  fn draw_mesh(&self, _mesh: GraphicsHandle, _topology: PrimitiveTopology, _vertex_count: usize, _index_count: usize) {
    // no-op
  }

  fn delete_mesh(&self, _mesh: GraphicsHandle) {
    // no-op
  }
}