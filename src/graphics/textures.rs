use std::path::Path;

use crate::assets::{Asset, AssetContext, LoadableAsset};
use crate::graphics::{Color, Image};
use crate::maths::Vector2;

/// A managed ID for OpenGL textures.
#[derive(Debug, Eq, PartialEq)]
struct TextureHandle(u32);

impl TextureHandle {
  pub fn new() -> Self {
    let mut id = 0;
    unsafe {
      gl::GenTextures(1, &mut id)
    }
    Self(id)
  }
}

impl Drop for TextureHandle {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteTextures(1, &self.0);
    }
  }
}

/// Represents a 2d texture.
#[derive(Debug, Eq, PartialEq)]
pub struct Texture {
  handle: TextureHandle,
  width: usize,
  height: usize,
  flags: TextureFlags,
}

impl Texture {
  pub fn new(width: usize, height: usize, flags: TextureFlags) -> Self {
    Self {
      handle: TextureHandle::new(),
      width,
      height,
      flags,
    }
  }

  pub fn width(&self) -> usize { self.width }
  pub fn height(&self) -> usize { self.height }

  /// Accesses the pixels of the `Texture`.
  pub fn pixels(&self) -> &[Color] {
    unimplemented!()
  }

  /// Mutably accesses the pixels of the `Texture`.
  pub fn pixels_mut(&mut self) -> &mut [Color] {
    unimplemented!()
  }
}

/// Represents a sub-region of a `Texture`.
pub struct TextureRegion {
  pub texture: Asset<Texture>,
  pub offset: Vector2<f32>,
  pub size: Vector2<f32>,
}

/// Flags for texture creation.
#[repr(u8)]
#[derive(BitFlags, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextureFlags {
  Clamp = 1 << 0,
}

impl LoadableAsset for Texture {
  fn load(path: &impl AsRef<Path>, context: &mut impl AssetContext) -> Self {
    let image = Image::load(path, context);

    Texture {
      handle: TextureHandle::new(),
      width: image.width(),
      height: image.height(),
      flags: TextureFlags::Clamp,
    }
  }
}
