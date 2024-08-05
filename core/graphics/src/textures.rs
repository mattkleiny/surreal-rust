//! Texture management and loading.

use common::{uvec2, Color, Color32, Pixel, Rectangle, ToVirtualPath, UVec2};

use super::*;

/// Different supported texture formats.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextureFormat {
  R8,
  RG8,
  RGB8,
  RGBA8,
  R32,
  RG32,
  RGB32,
  RGBA32,
  A8,
  A32,
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

/// Options for configuring a [`Texture`].
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TextureOptions {
  pub format: TextureFormat,
  pub sampler: TextureSampler,
}

impl Default for TextureOptions {
  fn default() -> Self {
    Self {
      format: TextureFormat::RGBA8,
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
  state: internal::GraphicsCell<TextureState>,
}

struct TextureState {
  id: TextureId,
  options: TextureOptions,
  width: u32,
  height: u32,
}

impl Texture {
  /// Creates a new blank texture on the GPU with default options.
  pub fn new(width: u32, height: u32, options: &TextureOptions) -> Result<Self, TextureError> {
    let texture = Self {
      state: internal::GraphicsCell::new(TextureState {
        id: graphics().texture_create(&options.sampler)?,
        options: options.clone(),
        width,
        height,
      }),
    };

    texture.initialize(width, height, options.format);

    Ok(texture)
  }

  /// Loads a texture from the given path.
  pub fn from_path(path: &impl ToVirtualPath) -> Result<Self, TextureError> {
    let image = Image::<Color32>::from_path(path).map_err(TextureError::InvalidImage)?;

    Self::from_image(&image)
  }

  /// Loads a texture from the given image.
  pub fn from_image<T: Pixel + Texel>(image: &Image<T>) -> Result<Self, TextureError> {
    let texture = Self::new(image.width(), image.height(), &TextureOptions::default())?;

    texture.write_pixels(image.width(), image.height(), image.as_slice());

    Ok(texture)
  }

  /// Builds a new colored texture of the given size.
  pub fn from_color<T: Texel>(width: u32, height: u32, color: T) -> Result<Self, TextureError> {
    let texture = Self::new(width, height, &TextureOptions {
      format: T::FORMAT,
      ..TextureOptions::default()
    })?;

    let colors = vec![color; width as usize * height as usize];

    texture.write_pixels(width, height, &colors);

    Ok(texture)
  }

  /// Returns the [`TextureId`] of the underlying texture.
  pub fn id(&self) -> TextureId {
    self.state.read().id
  }

  /// Returns the width of the texture.
  pub fn width(&self) -> u32 {
    self.state.read().width
  }

  /// Returns the width of the texture.
  pub fn height(&self) -> u32 {
    self.state.read().height
  }

  /// Returns the texture's format.
  pub fn format(&self) -> TextureFormat {
    self.state.read().options.format
  }

  /// Sets the texture's options on the GPU.
  pub fn set_options(&mut self, options: TextureOptions) {
    let mut state = self.state.write();

    state.options = options;

    graphics()
      .texture_set_options(state.id, &state.options.sampler)
      .expect("Failed to set texture options");
  }

  /// Initializes the texture with the given width and height.
  ///
  /// This is only necessary if the texture requires sizing information prior to
  /// access from the GPU.
  fn initialize(&self, width: u32, height: u32, format: TextureFormat) {
    let mut state = self.state.write();

    state.width = width;
    state.height = height;

    graphics()
      .texture_initialize(state.id, width, height, format)
      .expect("Failed to initialize texture");
  }

  /// Returns a [`TextureRegion`] that represents the entire texture.
  pub fn to_region(&self) -> TextureRegion {
    TextureRegion::new(self)
  }

  /// Resizes the texture in-place.
  ///
  /// Note that this will discard the contents of the texture and fill it with
  /// the default value.
  pub fn resize(&mut self, width: u32, height: u32) {
    self.initialize(width, height, self.format());
  }

  /// Downloads pixel data from the texture.
  #[allow(clippy::uninit_vec)]
  pub fn read_pixels<T: Texel>(&self) -> Vec<T> {
    let state = self.state.read();

    let size = state.width as usize * state.height as usize;
    let mut buffer = Vec::<T>::with_capacity(size);

    unsafe {
      buffer.set_len(size);

      graphics()
        .texture_read_data(
          state.id,
          size * size_of::<T>(),
          T::FORMAT,
          buffer.as_mut_ptr() as *mut u8,
          0, // mip level
        )
        .expect("Failed to read texture data");
    }

    buffer
  }

  /// Uploads pixel data to the texture.
  pub fn write_pixels<T: Texel>(&self, width: u32, height: u32, pixels: &[T]) {
    let mut state = self.state.write();

    state.width = width;
    state.height = height;

    graphics()
      .texture_write_data(
        state.id,
        width,
        height,
        match pixels.len() {
          0 => std::ptr::null(),
          _ => pixels.as_ptr() as *const u8,
        },
        state.options.format,
        T::FORMAT,
        0, // mip level
      )
      .expect("Failed to write texture data");
  }

  /// Uploads a subsection of pixel data to the texture.
  pub fn write_sub_pixels<T: Texel>(&self, region: &Rectangle, pixels: &[T]) {
    let state = self.state.read();

    graphics()
      .texture_write_sub_data(
        state.id,
        region,
        pixels.as_ptr() as *const u8,
        T::FORMAT,
        0, // mip level
      )
      .expect("Failed to write texture data");
  }
}

impl Drop for TextureState {
  fn drop(&mut self) {
    graphics().texture_delete(self.id).expect("Failed to delete texture");
  }
}

/// Represents a sub-region of a [`Texture`].
#[derive(Clone)]
pub struct TextureRegion {
  pub texture: Texture,
  pub offset: UVec2,
  pub size: UVec2,
}

impl TextureRegion {
  pub fn new(texture: &Texture) -> Self {
    Self {
      texture: texture.clone(),
      offset: uvec2(0, 0),
      size: uvec2(texture.width(), texture.height()),
    }
  }

  /// Sets the offset of the texture region.
  pub fn with_offset(mut self, offset: UVec2) -> Self {
    self.offset = offset;
    self
  }

  /// Sets the size of the texture region.
  pub fn with_size(mut self, size: UVec2) -> Self {
    self.size = size;
    self
  }

  /// Calculates the UV rectangle for the given texture region.
  pub fn calculate_uv(&self) -> Rectangle {
    let left = self.offset.x as f32 / self.texture.width() as f32;
    let top = self.offset.y as f32 / self.texture.height() as f32;
    let right = (self.offset.x + self.size.x) as f32 / self.texture.width() as f32;
    let bottom = (self.offset.y + self.size.y) as f32 / self.texture.height() as f32;

    Rectangle::from_corner_points(left, top, right, bottom)
  }

  /// Slices the texture region into a smaller region.
  pub fn slice(&self, x: u32, y: u32, width: u32, height: u32) -> Self {
    Self {
      texture: self.texture.clone(),
      offset: uvec2(self.offset.x + x, self.offset.y + y),
      size: uvec2(width, height),
    }
  }
}

/// Indicates a kind of pixel that can be used in a texture.
pub trait Texel: Clone + Copy + Sized {
  const FORMAT: TextureFormat;
}

/// Implements texel representations for common pixel types.
macro_rules! impl_texel {
  ($type:ty, $value:ident) => {
    impl Texel for $type {
      const FORMAT: TextureFormat = TextureFormat::$value;
    }
  };
}

impl_texel!(Color32, RGBA8);
impl_texel!(u8, R8);
impl_texel!([u8; 1], R8);
impl_texel!([u8; 2], RG8);
impl_texel!([u8; 3], RGB8);
impl_texel!([u8; 4], RGBA8);
impl_texel!((u8,), R8);
impl_texel!((u8, u8), RG8);
impl_texel!((u8, u8, u8), RGB8);
impl_texel!((u8, u8, u8, u8), RGBA8);

impl_texel!(Color, RGBA32);
impl_texel!(f32, R32);
impl_texel!([f32; 1], R32);
impl_texel!([f32; 2], RG32);
impl_texel!([f32; 3], RGB32);
impl_texel!([f32; 4], RGBA32);
impl_texel!((f32,), R32);
impl_texel!((f32, f32), RG32);
impl_texel!((f32, f32, f32), RGB32);
impl_texel!((f32, f32, f32, f32), RGBA32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_texture_from_image() {
    let image = Image::<Color32>::new(128, 128);
    let texture = Texture::from_image(&image).unwrap();

    assert_eq!(texture.width(), 128);
    assert_eq!(texture.height(), 128);
  }

  #[test]
  fn test_conversion_to_texture_region() {
    let texture = Texture::new(16, 16, &TextureOptions::default()).unwrap();
    let region = texture.to_region();

    assert_eq!(region.offset, uvec2(0, 0));
    assert_eq!(region.size, uvec2(16, 16));
  }

  #[test]
  fn test_texture_region_uv_calculation() {
    let texture = Texture::new(16, 16, &TextureOptions::default()).unwrap();
    let region = texture.to_region().with_offset(uvec2(4, 4)).with_size(uvec2(8, 8));
    let uv = region.calculate_uv();

    assert_eq!(uv.left(), 0.25);
    assert_eq!(uv.top(), 0.25);
    assert_eq!(uv.right(), 0.75);
    assert_eq!(uv.bottom(), 0.75);
  }
}
