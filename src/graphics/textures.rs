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

/// Options for configuring a `Texture`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TextureOptions {
  format: TextureFormat,
  filter_mode: TextureFilter,
  wrap_mode: TextureWrap,
}

impl Default for TextureOptions {
  fn default() -> Self {
    Self {
      format: TextureFormat::RGBA,
      filter_mode: TextureFilter::Nearest,
      wrap_mode: TextureWrap::Clamp,
    }
  }
}

/// A texture is a set of pixel data that has been uploaded to the GPU.
pub struct Texture {
  handle: GraphicsHandle,
  context: GraphicsContext,
  options: TextureOptions,
}

impl Texture {
  /// Creates a new blank texture on the GPU with default options.
  pub fn new(context: &GraphicsContext) -> Self {
    Self::new_with_options(context, TextureOptions::default())
  }

  /// Creates a new blank texture on the GPU.
  pub fn new_with_options(context: &GraphicsContext, options: TextureOptions) -> Self {
    Self {
      handle: unsafe { context.create_texture(options.filter_mode, options.wrap_mode) },
      context: context.clone(),
      options,
    }
  }

  /// Creates a new texture from an image.
  pub fn from_image(context: &GraphicsContext, image: &Image, options: TextureOptions) -> Texture {
    let mut texture = Self::new_with_options(context, options);

    texture.write_pixels(image.width(), image.height(), &image.as_slice());

    texture
  }

  /// Downloads pixel data from the texture.
  pub fn read_pixels(&self) -> Vec<Color> {
    todo!()
  }

  /// Uploads pixel data to the texture.
  pub fn write_pixels(&mut self, width: usize, height: usize, pixels: &[Color]) {
    unsafe {
      let size = pixels.len() * size_of::<Color>();
      let bytes = slice::from_raw_parts(pixels.as_ptr() as *const u8, size);

      self.context.write_texture_data(self.handle, width, height, bytes, self.options.format, 0);
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
  options: TextureOptions,
}

impl TextureLoader {
  pub fn new(context: &GraphicsContext) -> Self {
    Self {
      context: context.clone(),
      options: TextureOptions::default(),
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
    let texture = Texture::from_image(&self.context, image, self.options);

    Ok(texture)
  }
}