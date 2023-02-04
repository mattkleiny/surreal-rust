//! Texture management and loading.

use std::{cell::RefCell, rc::Rc};

use super::*;
use crate::{
  assets::{Asset, AssetContext, AssetLoader, AssetManager},
  collections::Grid,
  maths::{uvec2, Rectangle, UVec2},
};

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
  state: Rc<RefCell<TextureState>>,
}

struct TextureState {
  id: TextureId,
  graphics: GraphicsServer,
  options: TextureOptions,
  width: u32,
  height: u32,
}

impl Texture {
  /// Creates a new blank texture on the GPU with default options.
  pub fn new(graphics: &GraphicsServer) -> crate::Result<Self> {
    Self::with_options(graphics, &TextureOptions::default())
  }

  /// Builds a new colored texture of the given size.
  pub fn create_colored<T: Texel + Clone>(graphics: &GraphicsServer, width: usize, height: usize, color: T) -> crate::Result<Self> {
    let texture = Self::new(graphics)?;
    let colors = vec![color; width * height];

    texture.write_pixels(width, height, &colors);

    Ok(texture)
  }

  /// Creates a new blank texture on the GPU with the given options.
  pub fn with_options(graphics: &GraphicsServer, options: &TextureOptions) -> crate::Result<Self> {
    Ok(Self {
      state: Rc::new(RefCell::new(TextureState {
        id: graphics.texture_create(&options.sampler)?,
        graphics: graphics.clone(),
        options: options.clone(),
        width: 0,
        height: 0,
      })),
    })
  }

  /// Creates a new blank texture on the GPU with the given options and initial
  /// size.
  pub fn with_options_and_size(
    graphics: &GraphicsServer,
    options: &TextureOptions,
    width: u32,
    height: u32,
    format: TextureFormat,
  ) -> crate::Result<Self> {
    let texture = Self::with_options(graphics, options)?;
    texture.initialize(width, height, format);
    Ok(texture)
  }

  /// Returns the [`TextureId`] of the underlying texture.
  pub fn id(&self) -> TextureId {
    self.state.borrow().id
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
    state
      .graphics
      .texture_set_options(state.id, &state.options.sampler)
      .expect("Failed to set texture options");
  }

  /// Initializes the texture with the given width and height.
  ///
  /// This is only necessary if the texture requires sizing information prior to
  /// access from the GPU.
  pub fn initialize(&self, width: u32, height: u32, format: TextureFormat) {
    let mut state = self.state.borrow_mut();

    state.width = width;
    state.height = height;

    let graphics = &state.graphics;

    graphics
      .texture_initialize(state.id, width, height, format)
      .expect("Failed to initialize texture");
  }

  /// Resizes the texture in-place.
  ///
  /// Note that this will discard the contents of the texture and fill it with
  /// the default value.
  pub fn resize(&mut self, width: u32, height: u32) {
    let format = self.state.borrow().options.format;

    self.initialize(width, height, format);
  }

  /// Downloads pixel data from the texture.
  #[allow(clippy::uninit_vec)]
  pub fn read_pixels<T: Texel>(&self) -> Vec<T> {
    let state = self.state.borrow();

    let graphics = &state.graphics;
    let size = state.width as usize * state.height as usize;

    let mut buffer = Vec::<T>::with_capacity(size);

    unsafe {
      buffer.set_len(size);

      graphics
        .texture_read_data(
          state.id,
          size * std::mem::size_of::<T>(),
          T::FORMAT,
          buffer.as_mut_ptr() as *mut u8,
          0, // mip level
        )
        .expect("Failed to read texture data");
    }

    buffer
  }

  /// Uploads pixel data to the texture.
  pub fn write_pixels<T: Texel>(&self, width: usize, height: usize, pixels: &[T]) {
    let mut state = self.state.borrow_mut();

    state.width = width as u32;
    state.height = height as u32;

    let graphics = &state.graphics;

    graphics
      .texture_write_data(
        state.id,
        width as u32,
        height as u32,
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

  /// Uploads a sub-section of pixel data to the texture.
  pub fn write_sub_pixels<T: Texel>(&self, region: &Rectangle, pixels: &[T]) {
    let state = self.state.borrow();
    let graphics = &state.graphics;

    graphics
      .texture_write_sub_data(
        state.id,
        region,
        pixels.as_ptr() as *const u8,
        T::FORMAT,
        0, // mip level
      )
      .expect("Failed to write texture data");
  }

  /// Uploads pixel data to the texture from the given [`Image`].
  pub fn write_image(&mut self, image: &Image) {
    self.write_pixels(image.width(), image.height(), image.as_slice());
  }

  /// Blits this texture to the active display via the given material.
  pub fn blit_to_display(&self, material: &mut Material) {
    material.set_texture("u_texture", self, None);
    material.draw_fullscreen_quad();
  }
}

impl Drop for TextureState {
  fn drop(&mut self) {
    self.graphics.texture_delete(self.id).expect("Failed to delete texture");
  }
}

/// An [`AssetLoader`] for textures.
pub struct TextureLoader {
  pub graphics: GraphicsServer,
  pub options: TextureOptions,
}

impl Asset for Texture {
  type Loader = TextureLoader;
}

impl AssetLoader<Texture> for TextureLoader {
  fn load(&self, context: &AssetContext) -> crate::Result<Texture> {
    let image = context.load_asset(&context.path)?;
    let mut texture = Texture::new(&self.graphics)?;

    texture.write_image(&image);

    Ok(texture)
  }
}

/// Represents a sub-region of a [`Texture`]`.
#[derive(Clone)]
pub struct TextureRegion {
  pub texture: Texture,
  pub offset: UVec2,
  pub size: UVec2,
}

impl TextureRegion {
  /// Calculates the UV rectangle for the given texture region.
  pub fn calculate_uv(&self) -> Rectangle {
    let left = self.offset.x as f32 / self.texture.width() as f32;
    let top = self.offset.y as f32 / self.texture.height() as f32;
    let right = (self.offset.x + self.size.x) as f32 / self.texture.width() as f32;
    let bottom = (self.offset.y + self.size.y) as f32 / self.texture.height() as f32;

    Rectangle::from_corner_points(left, top, right, bottom)
  }
}

impl From<Texture> for TextureRegion {
  fn from(texture: Texture) -> Self {
    TextureRegion {
      offset: uvec2(0, 0),
      size: uvec2(texture.width(), texture.height()),
      texture,
    }
  }
}

impl From<&Texture> for TextureRegion {
  fn from(texture: &Texture) -> Self {
    TextureRegion {
      texture: texture.clone(),
      offset: uvec2(0, 0),
      size: uvec2(texture.width(), texture.height()),
    }
  }
}

impl<R: AsRef<Texture>> From<&R> for TextureRegion {
  fn from(texture: &R) -> Self {
    let texture = texture.as_ref();

    TextureRegion {
      texture: texture.clone(),
      offset: uvec2(0, 0),
      size: uvec2(texture.width(), texture.height()),
    }
  }
}

/// An atlas of textures, which is a sub-division of a texture into a smaller
/// grid of [`TextureRegion`]s.
#[derive(Clone)]
pub struct TextureAtlas {
  texture: Texture,
  width: u32,
  height: u32,
}

impl TextureAtlas {
  /// Creates a new texture atlas from the given texture.
  pub fn new(width: u32, height: u32, texture: &Texture) -> Self {
    Self {
      texture: texture.clone(),
      width,
      height,
    }
  }

  /// Loads a texture atlas from the given file.
  pub fn load(assets: &AssetManager, width: u32, height: u32, path: &str) -> crate::Result<TextureAtlas> {
    let texture = Texture::load(assets, path)?;

    Ok(TextureAtlas::new(width, height, texture.as_ref()))
  }

  /// The width of the atlas, in sub-regions.
  pub fn width(&self) -> u32 {
    self.texture.width() / self.width
  }

  /// The height of the atlas, in sub-regions.
  pub fn height(&self) -> u32 {
    self.texture.width() / self.height
  }

  /// Gets a sub-region of the texture atlas at the given position.
  pub fn get_region(&self, x: u32, y: u32) -> TextureRegion {
    TextureRegion {
      texture: self.texture.clone(),
      offset: uvec2(x * self.width, y * self.height),
      size: uvec2(self.width, self.height),
    }
  }
}

/// A utility for building texture atlases procedurally.
#[derive(Default)]
pub struct TextureAtlasBuilder<T> {
  cells: Vec<TextureAtlasCell<T>>,
  cell_size: UVec2,
  next_offset: UVec2,
  stride: usize,
}

/// Represents a discrete cell in a [`TextureAtlasBuilder`].
pub struct TextureAtlasCell<T> {
  pub pixels: Grid<T>,
  pub offset: UVec2,
  pub size: UVec2,
}

impl<P: Texel + Clone + Default> TextureAtlasBuilder<P> {
  /// Creates a new texture atlas builder.
  pub fn new(stride: usize, cell_size: UVec2) -> Self {
    Self {
      cells: Vec::new(),
      cell_size,
      next_offset: uvec2(0, 0),
      stride,
    }
  }

  /// Allocates a new cell in the texture atlas.
  pub fn allocate(&mut self) -> &mut TextureAtlasCell<P> {
    // allocate the cell
    self.cells.push(TextureAtlasCell {
      pixels: Grid::new(self.cell_size.x as usize, self.cell_size.y as usize),
      offset: self.next_offset,
      size: self.cell_size,
    });

    // advance the offset
    self.next_offset.x += self.cell_size.x;

    let max_width = self.cell_size.x as usize;

    if self.next_offset.x as usize > self.stride * max_width {
      self.next_offset.x = 0;
      self.next_offset.y += self.cell_size.y;
    }

    self.cells.last_mut().unwrap()
  }

  /// Writes this builder's contents to the given texture.
  pub fn write_to(&self, texture: &Texture) {
    let cells_x = self.cells.len() % self.stride;
    let cells_y = (self.cells.len() / self.stride).max(1);

    let cell_size_x = self.cell_size.x as usize;
    let cell_size_y = self.cell_size.y as usize;

    let pixels_x = cells_x * self.cell_size.x as usize;
    let pixels_y = cells_y * self.cell_size.y as usize;

    let mut cell_x = 0;
    let mut cell_y = 0;

    let mut texels = vec![P::default(); pixels_x * pixels_y];

    for cell in &self.cells {
      for pixel_y in 0..cell.size.y as usize {
        for pixel_x in 0..cell.size.x as usize {
          let advance_x = cell_x * cell_size_x + pixel_x;
          let advance_y = cell_y * cell_size_y + pixel_y;
          let index = advance_x + advance_y * pixels_x;

          unsafe {
            texels[index] = cell.pixels.get_unchecked(pixel_x as i32, pixel_y as i32).clone();
          }
        }
      }

      cell_x += 1;
      if cell_x > cells_x {
        cell_x = 0;
        cell_y += 1;
      }
    }

    texture.write_pixels(pixels_x, pixels_y, &texels);
  }
}

/// Indicates a kind of pixel that can be used in a texture.
pub trait Texel {
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
