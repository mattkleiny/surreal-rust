use crate::assets::{Asset, AssetContext, LoadableAsset, AssetResult};
use crate::graphics::Image;
use crate::maths::{vec2, Vector2};
use crate::io::Path;

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

  pub fn upload(&mut self, image: &Image) {
    unsafe {
      let data = image.as_slice().as_ptr() as *const std::os::raw::c_void;

      gl::BindTexture(gl::TEXTURE_2D, self.handle.0);
      gl::TexImage2D(gl::TEXTURE_2D, 0, 0, self.width() as i32, self.height() as i32, 0, 0, 0, data);
    }
  }
}

/// Represents a sub-region of a `Texture`.
pub struct TextureRegion {
  pub texture: Asset<Texture>,
  pub offset: Vector2<f32>,
  pub size: Vector2<usize>,
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
    unsafe {
      let mut id: u32 = -1;
      gl::GenTextures(1, &mut id);

      if id == -1 {
        panic!("Failed to allocate GPU texture!");
      }

      Self(id as u32)
    }
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
  fn load(path: Path, context: &mut impl AssetContext) -> AssetResult<Self> {
    let image = Image::load(path, context)?;
    let mut texture = Texture {
      handle: TextureHandle::new(),
      width: image.width(),
      height: image.height(),
      flags: TextureFlags::Clamp,
    };

    texture.upload(&image);

    Ok(texture)
  }
}
