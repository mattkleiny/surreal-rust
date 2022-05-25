use std::cell::RefCell;
use std::rc::Rc;

use crate::maths::{Rectangle, vec2, Vector2};

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
#[derive(Clone, PartialEq)]
pub struct Texture {
  state: Rc<RefCell<TextureState>>,
}

/// The inner state of a texture.
struct TextureState {
  server: GraphicsServer,
  handle: GraphicsHandle,
  options: TextureOptions,
  width: u32,
  height: u32,
}

impl PartialEq for TextureState {
  fn eq(&self, other: &Self) -> bool {
    self.handle == other.handle
  }
}

/// Represents a sub-region of a `Texture`.
#[derive(Clone)]
pub struct TextureRegion {
  pub texture: Texture,
  pub offset: Vector2<u32>,
  pub size: Vector2<u32>,
}

impl TextureRegion {
  /// Calculates the UV rectangle for the given texture region.
  pub fn calculate_uv(&self) -> Rectangle<f32> {
    let left = self.offset.x as f32 / self.texture.width() as f32;
    let top = self.offset.y as f32 / self.texture.height() as f32;
    let right = (self.offset.x + self.size.x) as f32 / self.texture.width() as f32;
    let bottom = (self.offset.y + self.size.y) as f32 / self.texture.height() as f32;

    Rectangle::from_corner_points(left, top, right, bottom)
  }
}

impl From<&Texture> for TextureRegion {
  fn from(texture: &Texture) -> Self {
    Self {
      texture: texture.clone(),
      offset: Vector2::ZERO,
      size: vec2(texture.width(), texture.height()),
    }
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
        width: 0,
        height: 0,
      }))
    }
  }

  /// Returns the width of the texture.
  pub fn width(&self) -> u32 {
    self.state.borrow().width
  }

  /// Returns the width of the texture.
  pub fn height(&self) -> u32 {
    self.state.borrow().height
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
    let mut state = self.state.borrow_mut();

    state.width = width as u32;
    state.height = height as u32;

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

impl HasGraphicsHandle for Texture {
  /// Returns the underlying graphics handle of the texture.
  fn handle(&self) -> GraphicsHandle {
    self.state.borrow().handle
  }
}

impl Drop for TextureState {
  /// Deletes the texture from the GPU.
  fn drop(&mut self) {
    self.server.delete_texture(self.handle);
  }
}
