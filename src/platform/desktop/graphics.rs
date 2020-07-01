use crate::graphics::*;
use crate::RID;

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

  fn create_buffer(&mut self) -> GraphicsResult<RID> {
    let mut id = RID(0);

    unsafe {
      gl::GenBuffers(1, &mut id.0);
    }

    Ok(id)
  }

  fn upload_buffer_data(&mut self, buffer_id: RID, data: &[u8]) -> GraphicsResult<()> {
    unsafe {
      gl::BufferData(
        gl::ARRAY_BUFFER,
        data.len() as isize,
        data.as_ptr() as *mut std::ffi::c_void,
        gl::STATIC_DRAW,
      );
    }

    Ok(())
  }

  fn delete_buffer(&mut self, buffer_id: RID) -> GraphicsResult<()> {
    unsafe { gl::DeleteBuffers(1, &buffer_id.0) }

    Ok(())
  }

  fn create_and_link_shader(&mut self, source: &impl ShaderSource) -> GraphicsResult<RID> {
    let id = RID(0);

    for (kind, raw) in source.get_source() {
      match kind {
        ShaderKind::Vertex => unimplemented!(),
        ShaderKind::Fragment => unimplemented!(),
      }
    }

    Ok(id)
  }

  fn delete_shader(&mut self, shader_id: RID) -> GraphicsResult<()> {
    unsafe { gl::DeleteShader(shader_id.0) }

    Ok(())
  }
}
