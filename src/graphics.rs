//! A lightweight and fast cross-platform graphics engine.

pub use colors::*;
pub use images::*;
pub use shaders::*;
pub use sprites::*;

use crate::RID;

mod colors;
mod images;
mod shaders;
mod sprites;

// TODO: support resources (that can be serialized to disk)?
// TODO: take inspiration from other engines, perhaps
// TODO: build this on top of WGPU?

pub trait GraphicsServer {
  // frame buffers
  fn clear_active_framebuffer(&mut self, color: Color);

  // texture management
  fn create_texture(&mut self) -> Result<RID, GraphicsError>;
  fn create_texture_from_image<P>(&mut self, image: &Image<P>) -> Result<RID, GraphicsError>;
  fn upload_texture_data<P>(&mut self, id: RID, image: &Image<P>) -> Result<(), GraphicsError>;
}

#[repr(u8)]
#[derive(BitFlags, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextureFlags {
  Clamp = 1 << 0,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GraphicsError {
  NotEnoughMemory,
  InvalidTextureFormat,
  InvalidShaderProgram,
}

impl From<GraphicsError> for crate::Error {
  fn from(_: GraphicsError) -> Self {
    crate::Error::GraphicsFailure
  }
}
