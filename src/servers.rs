// TODO: implement a visual server like Godot?
// TODO: support resources (that can be serialized to disk)?
// TODO: take inspiration from other engines, perhaps

use crate::graphics::Image;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RID(u32);

impl Drop for RID {
  fn drop(&mut self) {
    unimplemented!()
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TextureError {
  NotEnoughMemory,
  NotEnoughTextureUnits,
}

#[repr(u8)]
#[derive(BitFlags, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextureFlags {
  Clamp = 1 << 0,
}

pub trait GraphicsServer {
  fn create_texture() -> Result<RID, TextureError>;
  fn create_texture_from_image<P>(image: &Image<P>) -> Result<RID, TextureError>;
}
