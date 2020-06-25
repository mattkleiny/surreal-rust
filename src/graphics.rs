//! A lightweight and fast cross-platform graphics engine.
//!
//! This implementation is designed to be portable across platforms through consumption of a
//! standard graphics API across all of those platforms, as opposed to offering different APIs
//! for different platforms (ala gfx-hal).
//!
//! Whilst more directly coupled than other providers, this implementation is simple, direct and
//! fast. It is designed to account for the majority use case as opposed to all possibilities and
//! to do it well, as opposed to solving the general case and doing it poorly.
//!
//! This implementation, whilst not strictly precluding 3d development, is primarily focused towards
//! 2d development, and a lot of the auxiliary libraries and utilities are designed to be fast in 2d.

pub use colors::*;
pub use images::*;
pub use sprites::*;

use crate::RID;

mod colors;
mod images;
mod sprites;

// TODO: implement a visual server like Godot?
// TODO: support resources (that can be serialized to disk)?
// TODO: take inspiration from other engines, perhaps

pub trait GraphicsServer {
  fn clear(&mut self, color: Color);
  fn create_texture(&mut self) -> Result<RID, TextureError>;
  fn create_texture_from_image<P>(&mut self, image: &Image<P>) -> Result<RID, TextureError>;
  fn upload_texture_data<P>(&mut self, id: RID, image: &Image<P>) -> Result<(), TextureError>;
}

#[repr(u8)]
#[derive(BitFlags, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextureFlags {
  Clamp = 1 << 0,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TextureError {
  NotEnoughMemory,
  NotEnoughTextureUnits,
}

impl From<TextureError> for crate::Error {
  fn from(_: TextureError) -> Self {
    crate::Error::GraphicsFailure
  }
}
