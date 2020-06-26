use crate::graphics::*;
use crate::RID;

use super::DesktopPlatform;

impl GraphicsServer for DesktopPlatform {
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

  fn create_texture(&mut self) -> Result<RID, GraphicsError> {
    unimplemented!()
  }

  fn create_texture_from_image<P>(&mut self, image: &Image<P>) -> Result<RID, GraphicsError> {
    unimplemented!()
  }

  fn upload_texture_data<P>(&mut self, id: RID, image: &Image<P>) -> Result<(), GraphicsError> {
    unimplemented!()
  }
}

#[cfg(feature = "shady")]
mod shady {
  use crate::graphics::shady::ShadyProgram;

  use super::*;

  impl crate::graphics::shady::Compiler for DesktopPlatform {
    type Error = GraphicsError;

    fn compile(&mut self, ast: &ShadyProgram) -> Result<ShaderProgram, Self::Error> {
      unimplemented!()
    }
  }
}