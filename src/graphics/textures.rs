use std::rc::Rc;

use crate::assets::Asset;
use crate::graphics::{GraphicsHandle, GraphicsServer, Image};
use crate::maths::Vector2;

/// Flags for texture creation.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextureFlags {
  Clamp = 1 << 0,
}

/// Represents a 2d texture.
pub struct Texture {
  server: Rc<dyn GraphicsServer>,
  handle: GraphicsHandle,
  width: usize,
  height: usize,
  flags: TextureFlags,
}

/// Represents a sub-region of a `Texture`.
pub struct TextureRegion {
  pub offset: Vector2<f32>,
  pub size: Vector2<usize>,
  pub texture: Asset<Texture>,
}

impl Texture {
  pub fn new(server: &Rc<dyn GraphicsServer>, width: usize, height: usize, flags: TextureFlags) -> Self {
    let handle = server.create_texture();

    Self {
      server: server.clone(),
      handle,
      width,
      height,
      flags,
    }
  }

  pub fn width(&self) -> usize { self.width }
  pub fn height(&self) -> usize { self.height }
  pub fn flags(&self) -> TextureFlags { self.flags }

  pub fn upload(&mut self, image: &Image) {
    unimplemented!()
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    self.server.delete_texture(self.handle);
  }
}
