use std::mem::size_of;
use std::slice;

use crate::graphics::{Color, GraphicsContext, GraphicsHandle};

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

/// A GPU texture that can read and write pixel data.
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