use std::cell::RefCell;
use std::rc::Rc;

use crate::maths::Rectangle;

use super::*;

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

/// A sampler describes how a texture should be read from a shader program.
///
/// Sampler allow re-configuring wrap and filter modes on a per-material basis.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TextureSampler {
  pub wrap_mode: TextureWrap,
  pub minify_filter: TextureFilter,
  pub magnify_filter: TextureFilter,
}

/// Options for configuring a `Texture`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TextureOptions {
  pub format: TextureFormat,
  pub sampler: TextureSampler,
}

impl Default for TextureOptions {
  fn default() -> Self {
    Self {
      format: TextureFormat::RGBA,
      sampler: TextureSampler {
        wrap_mode: TextureWrap::Clamp,
        minify_filter: TextureFilter::Nearest,
        magnify_filter: TextureFilter::Nearest,
      },
    }
  }
}

/// A texture is a set of pixel data that has been uploaded to the GPU.
#[derive(Clone)]
pub struct Texture {
  state: Rc<RefCell<TextureState>>,
}

/// The inner state of a texture.
struct TextureState {
  server: GraphicsServer,
  handle: GraphicsHandle,
  options: TextureOptions,
}

impl HasGraphicsHandle for Texture {
  /// Returns the underlying graphics handle of the texture.
  fn handle(&self) -> GraphicsHandle {
    let state = self.state.borrow();

    state.handle
  }
}

impl Texture {
  /// Creates a new blank texture on the GPU with default options.
  pub fn new(server: &GraphicsServer) -> Self {
    Self::with_options(server, TextureOptions::default())
  }

  /// Creates a new blank texture on the GPU with the given [`TextureOptions`].
  pub fn with_options(server: &GraphicsServer, options: TextureOptions) -> Self {
    Self {
      state: Rc::new(RefCell::new(TextureState {
        server: server.clone(),
        handle: server.create_texture(&options.sampler),
        options,
      }))
    }
  }

  /// Sets the the texture's options on the GPU.
  pub fn set_options(&mut self, options: TextureOptions) {
    let mut state = self.state.borrow_mut();

    state.options = options;
    state.server.set_texture_options(state.handle, &options.sampler);
  }

  /// Downloads pixel data from the texture.
  pub fn read_pixels<P>(&self) -> Vec<P> where P: Pixel {
    todo!()
  }

  /// Uploads pixel data to the texture.
  pub fn write_pixels<P>(&mut self, width: usize, height: usize, pixels: &[P]) where P: Pixel {
    let state = self.state.borrow();

    state.server.write_texture_data(
      state.handle,
      width,
      height,
      pixels.as_ptr() as *const u8,
      state.options.format,
      0, // mip-level
    );
  }

  /// Uploads a sub-section of pixel data to the texture.
  pub fn write_sub_pixels<P>(&mut self, _region: &Rectangle<usize>, _pixels: &[P]) where P: Pixel {
    todo!()
  }

  /// Uploads pixel data to the texture from the given image.
  pub fn write_image(&mut self, image: &Image) {
    self.write_pixels(image.width(), image.height(), &image.as_slice());
  }
}

impl Drop for TextureState {
  /// Deletes the texture from the GPU.
  fn drop(&mut self) {
    self.server.delete_texture(self.handle);
  }
}
