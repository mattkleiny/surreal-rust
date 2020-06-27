use crate::graphics::*;
use crate::RID;

use super::DesktopPlatform;

macro_rules! checked {
  ($e:expr) => {
    // TODO: check for OpenGL errors/etc
    unsafe { ($e) }
  };
}

impl GraphicsServer for DesktopPlatform {
  type Buffer = ();
  type Texture = ();
  type Shader = ();

  fn create_framebuffer(&mut self) -> Result<RID, GraphicsError> {
    unimplemented!()
  }

  fn delete_framebuffer(&mut self, buffer_id: RID) -> Result<RID, GraphicsError> {
    unimplemented!()
  }

  fn set_active_framebuffer(&mut self, buffer_id: RID) -> Result<(), GraphicsError> {
    unimplemented!()
  }

  fn clear_active_framebuffer(&mut self, color: Color) {
    unsafe {
      gl::ClearColor(
        color.r as f32 / 255.0,
        color.g as f32 / 255.0,
        color.b as f32 / 255.0,
        color.a as f32 / 255.0,
      );
    }
    checked!(gl::Clear(gl::COLOR_BUFFER_BIT));
  }

  fn create_vertex_buffer(&mut self) -> Result<RID, GraphicsError> {
    unimplemented!()
  }

  fn create_index_buffer(&mut self) -> Result<RID, GraphicsError> {
    unimplemented!()
  }

  fn draw_mesh(&mut self, count: usize, topology: PrimitiveTopology) -> Result<(), GraphicsError> {
    unimplemented!()
  }

  fn draw_mesh_indexed(&mut self, count: usize, topology: PrimitiveTopology) -> Result<(), GraphicsError> {
    unimplemented!()
  }

  fn create_texture(&mut self) -> Result<RID, GraphicsError> {
    unimplemented!()
  }

  fn create_texture_from_image(&mut self, image: &Image) -> Result<RID, GraphicsError> {
    unimplemented!()
  }

  fn upload_texture_data(&mut self, id: RID, image: &Image) -> Result<(), GraphicsError> {
    unimplemented!()
  }

  fn delete_texture(&mut self, texture_id: RID) -> Result<(), GraphicsError> {
    unimplemented!()
  }

  fn create_shader(&mut self, source: &impl ShaderSource) -> Result<RID, GraphicsError> {
    for (kind, raw) in source.get_spirv_binary() {}

    unimplemented!()
  }

  fn delete_shader(&mut self, shader_id: RID) -> Result<(), GraphicsError> {
    Ok(checked!(gl::DeleteShader(shader_id.0)))
  }
}
