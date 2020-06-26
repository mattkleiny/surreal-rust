use crate::graphics::*;
use crate::RID;

use super::DesktopPlatform;

impl GraphicsServer for DesktopPlatform {
  fn clear_active_framebuffer(&mut self, color: Color) {
    let color: (u8, u8, u8, u8) = color.into();

    self.canvas.set_draw_color(color);
    self.canvas.clear();
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
  use crate::graphics::shady::AST;

  use super::*;

  impl crate::graphics::shady::Compiler for DesktopPlatform {
    type Error = GraphicsError;

    fn compile(&mut self, ast: &AST) -> Result<Shader, Self::Error> {
      unimplemented!()
    }
  }
}