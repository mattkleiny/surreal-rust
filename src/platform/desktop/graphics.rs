use std::ffi::c_void;

use raw_gl_context::{GlConfig, GlContext};

use crate::graphics::{BufferKind, BufferUsage, GraphicsServer, TextureFilter, TextureFormat, TextureWrap};

use super::*;

/// The graphics server for the desktop platform.
pub struct DesktopGraphicsServer {
  context: GlContext,
}

impl DesktopGraphicsServer {
  pub fn new(window: &Window) -> Self {
    // prepare and load opengl functionality
    let config = GlConfig {
      vsync: true,
      ..Default::default()
    };

    let context = GlContext::create(window, config).unwrap();
    context.make_current();
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    Self { context }
  }
}

unsafe impl GraphicsServer for DesktopGraphicsServer {
  unsafe fn begin_frame(&self) {
    self.context.make_current();
  }

  unsafe fn end_frame(&self) {
    self.context.swap_buffers();
    self.context.make_not_current();
  }

  unsafe fn set_viewport_size(&self, viewport: Viewport) {
    gl::Viewport(0, 0, viewport.width as i32, viewport.height as i32);
  }

  unsafe fn clear_color_buffer(&self, color: Color) {
    gl::ClearColor(color.r, color.g, color.b, color.a);
    gl::Clear(gl::COLOR_BUFFER_BIT);
  }

  unsafe fn clear_depth_buffer(&self) {
    gl::Clear(gl::DEPTH_BUFFER_BIT);
  }

  unsafe fn flush_commands(&self) {
    gl::Flush();
  }

  unsafe fn create_buffer(&self) -> GraphicsHandle {
    let mut id: u32 = 0;
    gl::GenBuffers(1, &mut id);
    GraphicsHandle { id }
  }

  unsafe fn read_buffer_data(&self, buffer: GraphicsHandle, kind: BufferKind, offset: usize, length: usize) -> Vec<u8> {
    let kind = match kind {
      BufferKind::Element => gl::ARRAY_BUFFER,
      BufferKind::Index => gl::ELEMENT_ARRAY_BUFFER,
      BufferKind::Uniform => gl::UNIFORM_BUFFER,
    };

    let mut data = Vec::with_capacity(length);
    let pointer = data.as_mut_ptr() as *mut c_void;

    gl::BindBuffer(kind, buffer.id);
    gl::BufferSubData(kind, offset as isize, length as isize, pointer);

    data
  }

  unsafe fn write_buffer_data(&self, buffer: GraphicsHandle, usage: BufferUsage, kind: BufferKind, data: &[u8]) {
    let kind = match kind {
      BufferKind::Element => gl::ARRAY_BUFFER,
      BufferKind::Index => gl::ELEMENT_ARRAY_BUFFER,
      BufferKind::Uniform => gl::UNIFORM_BUFFER,
    };

    let usage = match usage {
      BufferUsage::Static => gl::STATIC_DRAW,
      BufferUsage::Dynamic => gl::DYNAMIC_DRAW,
    };

    let size = data.len() as isize;
    let pointer = data.as_ptr() as *const c_void;

    gl::BindBuffer(kind, buffer.id);
    gl::BufferData(kind, size, pointer, usage);
  }

  unsafe fn delete_buffer(&self, buffer: GraphicsHandle) {
    gl::DeleteBuffers(1, &buffer.id);
  }

  unsafe fn create_texture(&self, filter_mode: TextureFilter, wrap_mode: TextureWrap) -> GraphicsHandle {
    let mut id: u32 = 0;
    let target = gl::TEXTURE_2D;

    gl::GenTextures(1, &mut id);
    gl::BindTexture(target, id);

    let filter_mode = match filter_mode {
      TextureFilter::Nearest => gl::NEAREST,
      TextureFilter::Linear => gl::LINEAR,
    };

    let wrap_mode = match wrap_mode {
      TextureWrap::Clamp => gl::CLAMP_TO_EDGE,
      TextureWrap::Mirror => gl::MIRRORED_REPEAT,
    };

    gl::TexParameteri(target, gl::TEXTURE_MIN_FILTER, filter_mode as i32);
    gl::TexParameteri(target, gl::TEXTURE_MAG_FILTER, filter_mode as i32);
    gl::TexParameteri(target, gl::TEXTURE_WRAP_S, wrap_mode as i32);
    gl::TexParameteri(target, gl::TEXTURE_WRAP_T, wrap_mode as i32);

    GraphicsHandle { id }
  }

  unsafe fn write_texture_data(&self, texture: GraphicsHandle, width: usize, height: usize, pixels: &[u8], format: TextureFormat, mip_level: usize) {
    let target = gl::TEXTURE_2D;
    let internal_format = match format {
      TextureFormat::RGBA => gl::RGBA32F
    };

    let pointer = pixels.as_ptr() as *const c_void;

    gl::BindTexture(target, texture.id);
    gl::TexImage2D(target, 0, internal_format as i32, width as i32, height as i32, 0, gl::RGBA, gl::FLOAT, pointer);
  }

  unsafe fn delete_texture(&self, texture: GraphicsHandle) {
    gl::DeleteTextures(1, &texture.id);
  }

  unsafe fn create_shader(&self) -> GraphicsHandle {
    todo!()
  }

  unsafe fn delete_shader(&self, shader: GraphicsHandle) {
    todo!()
  }

  unsafe fn create_mesh(&self) -> GraphicsHandle {
    let mut id: u32 = 0;
    gl::GenVertexArrays(1, &mut id);
    GraphicsHandle { id }
  }

  unsafe fn delete_mesh(&self, mesh: GraphicsHandle) {
    gl::DeleteVertexArrays(1, &mesh.id);
  }
}
