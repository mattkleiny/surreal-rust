use crate::graphics::*;

use super::DesktopPlatform;

// TODO: add helper macro for OpenGL errors.

impl Graphics for DesktopPlatform {
  fn clear_active_framebuffer(&mut self, color: Color) {
    unsafe {
      gl::ClearColor(
        color.r as f32 / 255.0,
        color.g as f32 / 255.0,
        color.b as f32 / 255.0,
        color.a as f32 / 255.0,
      );
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }
  }

  fn set_viewport(&mut self, viewport: Viewport) {
    unsafe {
      gl::Viewport(0, 0, viewport.width as i32, viewport.height as i32);
    }
  }
}
