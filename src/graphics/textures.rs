use crate::assets::*;
use crate::graphics::Image;
use crate::maths::{vec2, Vector2};

/// Represents a 2d texture.
#[derive(Debug, Eq, PartialEq)]
pub struct Texture {
  width: usize,
  height: usize,
  flags: TextureFlags,
  handle: TextureHandle,
}

impl Texture {
  pub fn new(width: usize, height: usize, flags: TextureFlags) -> Self {
    Self {
      width,
      height,
      flags,
      handle: TextureHandle::new(),
    }
  }

  pub fn width(&self) -> usize { self.width }
  pub fn height(&self) -> usize { self.height }
  pub fn flags(&self) -> TextureFlags { self.flags }

  pub fn upload(&mut self, image: &Image) {
    unimplemented!()
  }
}

/// Represents a sub-region of a `Texture`.
pub struct TextureRegion {
  pub offset: Vector2<f32>,
  pub size: Vector2<usize>,
  pub texture: Asset<Texture>,
}

impl From<Asset<Texture>> for TextureRegion {
  fn from(texture: Asset<Texture>) -> Self {
    Self {
      offset: vec2(0., 0.),
      size: vec2(texture.width(), texture.height()),
      texture,
    }
  }
}

/// Flags for texture creation.
#[repr(u8)]
#[derive(BitFlags, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextureFlags {
  Clamp = 1 << 0,
}

/// A managed ID for OpenGL textures.
#[derive(Debug, Eq, PartialEq)]
struct TextureHandle(u32);

impl TextureHandle {
  pub fn new() -> Self {
    let mut handle = Self(0);
    unsafe {
      gl::GenTextures(1, &mut handle.0);
    }
    handle
  }
}

impl Drop for TextureHandle {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteTextures(1, &self.0);
    }
  }
}

impl LoadableAsset for Texture {
  fn load(path: impl AsRef<str>, context: &mut impl AssetContext) -> AssetResult<Self> {
    let image = Image::load(path, context)?;

    let mut texture = Texture {
      width: image.width(),
      height: image.height(),
      flags: TextureFlags::Clamp,
      handle: TextureHandle::new(),
    };

    texture.upload(&image);

    Ok(texture)
  }
}
