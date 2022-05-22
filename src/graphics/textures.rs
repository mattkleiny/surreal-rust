use std::mem::size_of;
use std::slice;

use crate::assets::{AssetLoadContext, AssetLoader, AssetResult};
use crate::graphics::{Color, GraphicsContext, GraphicsHandle, Image};

/// Different supported texture formats.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextureFormat {
  RGBA
}

/// Texture wrapping modes modes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextureWrap {
  Clamp,
  Mirror,
}

/// Texture filter modes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextureFilter {
  Nearest,
  Linear,
}

/// A texture is a set of pixel data that has been uploaded to the GPU.
pub struct Texture {
  handle: GraphicsHandle,
  context: GraphicsContext,
  format: TextureFormat,
}

impl Texture {
  /// Creates a new blank texture on the GPU.
  pub fn new(context: &GraphicsContext, format: TextureFormat, filter_mode: TextureFilter, wrap_mode: TextureWrap) -> Self {
    Self {
      handle: unsafe { context.create_texture(filter_mode, wrap_mode) },
      context: context.clone(),
      format,
    }
  }

  /// Creates a new texture from an image.
  pub fn from_image(context: &GraphicsContext, image: &Image, format: TextureFormat, filter: TextureFilter, wrap: TextureWrap) -> Texture {
    let mut texture = Self::new(context, format, filter, wrap);

    texture.write_pixel_data(image.width(), image.height(), &image.as_slice());

    texture
  }

  /// Uploads pixel data to the texture.
  pub fn write_pixel_data(&mut self, width: usize, height: usize, pixels: &[Color]) {
    unsafe {
      let size = pixels.len() * size_of::<Color>();
      let bytes = slice::from_raw_parts(pixels.as_ptr() as *const u8, size);

      self.context.write_texture_data(self.handle, width, height, bytes, self.format, 0);
    }
  }
}

impl Drop for Texture {
  /// Deletes the texture from the GPU.
  fn drop(&mut self) {
    unsafe {
      self.context.delete_texture(self.handle);
    }
  }
}

/// Allows loading `Texture` from the virtual file system.
pub struct TextureLoader {
  context: GraphicsContext,
  default_format: TextureFormat,
  default_filter: TextureFilter,
  default_wrap: TextureWrap,
}

impl TextureLoader {
  pub fn new(context: &GraphicsContext) -> Self {
    Self {
      context: context.clone(),
      default_format: TextureFormat::RGBA,
      default_filter: TextureFilter::Nearest,
      default_wrap: TextureWrap::Clamp,
    }
  }
}

impl AssetLoader for TextureLoader {
  type Asset = Texture;

  fn can_load(&self, context: AssetLoadContext) -> bool {
    context.path.extension() == ".png"
  }

  fn load(&self, context: AssetLoadContext) -> AssetResult<Self::Asset> {
    let image = context.load_asset(context.path)?;
    let texture = Texture::from_image(&self.context, image, self.default_format, self.default_filter, self.default_wrap);

    Ok(texture)
  }
}