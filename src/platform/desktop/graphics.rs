use raw_gl_context::{GlConfig, GlContext};

use crate::graphics::GraphicsServer;

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
    gl::ClearColor(
      color.r as f32 / 255.0,
      color.g as f32 / 255.0,
      color.b as f32 / 255.0,
      color.a as f32 / 255.0,
    );
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
    GraphicsHandle { id: id as usize }
  }

  unsafe fn read_buffer_data(&self, buffer: GraphicsHandle) -> Vec<u8> {
    todo!()
  }

  unsafe fn write_buffer_data(&self, buffer: GraphicsHandle, data: &[u8]) {
    todo!()
  }

  unsafe fn delete_buffer(&self, buffer: GraphicsHandle) {
    gl::DeleteBuffers(1, &(buffer.id as u32));
  }

  unsafe fn create_texture(&self) -> GraphicsHandle {
    let mut id: u32 = 0;
    gl::GenTextures(1, &mut id);
    GraphicsHandle { id: id as usize }
  }

  unsafe fn write_texture_data(&self, texture: GraphicsHandle, data: &[u8]) {
    todo!()
  }

  unsafe fn delete_texture(&self, texture: GraphicsHandle) {
    gl::DeleteTextures(1, &(texture.id as u32));
  }

  unsafe fn create_shader(&self) -> GraphicsHandle {
    todo!()
  }

  unsafe fn delete_shader(&self, shader: GraphicsHandle) {
    todo!()
  }

  unsafe fn create_mesh(&self) -> GraphicsHandle {
    todo!()
  }
}
