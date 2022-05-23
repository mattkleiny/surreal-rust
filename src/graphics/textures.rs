use crate::assets::{AssetLoadContext, AssetLoader, AssetResult};
use crate::graphics::{GraphicsContext, GraphicsHandle, Image, Pixel};

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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TextureSampler {
  pub wrap_mode: TextureWrap,
  pub minify_filter: TextureFilter,
  pub magnify_filter: TextureFilter,
}

/// Options for configuring a `Texture`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
pub struct Texture {
  context: GraphicsContext,
  handle: GraphicsHandle,
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
      context: context.clone(),
      handle: context.create_texture(&options.sampler),
      options,
    }
  }

  /// Returns the underlying GPU texture handle.
  pub fn handle(&self) -> GraphicsHandle {
    self.handle
  }

  /// Returns the texture options.
  pub fn options(&self) -> &TextureOptions {
    &self.options
  }

  /// Sets the the texture's options on the GPU.
  pub fn set_options(&mut self, options: TextureOptions) {
    self.options = options;

    // TODO: configure on the GPU, too
  }

  /// Downloads pixel data from the texture.
  pub fn read_pixels<P>(&self) -> Vec<P> where P: Pixel {
    todo!()
  }

  /// Uploads pixel data to the texture.
  pub fn write_pixels<P>(&mut self, width: usize, height: usize, pixels: &[P]) where P: Pixel {
    self.context.write_texture_data(
      self.handle,
      width,
      height,
      pixels.as_ptr() as *const u8,
      self.options.format,
      0, // mip-level
    );
  }

  /// Uploads pixel data to the texture from the given image.
  pub fn write_image(&mut self, image: &Image) {
    self.write_pixels(image.width(), image.height(), &image.as_slice());
  }
}

impl Drop for Texture {
  /// Deletes the texture from the GPU.
  fn drop(&mut self) {
    self.context.delete_texture(self.handle);
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
    let mut texture = Texture::new_with_options(&self.context, self.options);

    texture.write_image(&image);

    Ok(texture)
  }
}