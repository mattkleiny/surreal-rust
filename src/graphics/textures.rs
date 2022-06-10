//! Texture management and loading.

use std::cell::RefCell;
use std::rc::Rc;

use crate::assets::{Asset, AssetContext, AssetLoader};
use crate::collections::Grid;
use crate::maths::{vec2, Rectangle, Vector2};

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
  graphics: GraphicsServer,
  handle: GraphicsHandle,
  options: TextureOptions,
  width: u32,
  height: u32,
}

impl Texture {
  /// Creates a new blank texture on the GPU with default options.
  pub fn new(graphics: &GraphicsServer) -> Self {
    Self::with_options(graphics, &TextureOptions::default())
  }

  /// Builds a new colored texture of the given size.
  pub fn create_colored<T: Texel + Clone>(graphics: &GraphicsServer, width: usize, height: usize, color: T) -> Self {
    let mut texture = Self::new(graphics);
    let colors = vec![color; width * height];

    texture.write_pixels(width, height, &colors);

    texture
  }

  /// Creates a new blank texture on the GPU with the given options.
  pub fn with_options(graphics: &GraphicsServer, options: &TextureOptions) -> Self {
    Self {
      state: Rc::new(RefCell::new(TextureState {
        graphics: graphics.clone(),
        handle: graphics.create_texture(&options.sampler),
        options: options.clone(),
        width: 0,
        height: 0,
      })),
    }
  }

  /// Returns the width of the texture .
  pub fn width(&self) -> u32 {
    self.state.borrow().width
  }

  /// Returns the width of the texture .
  pub fn height(&self) -> u32 {
    self.state.borrow().height
  }

  /// Sets the the texture's options on the GPU.
  pub fn set_options(&mut self, options: TextureOptions) {
    let mut state = self.state.borrow_mut();

    state.options = options;

    let graphics = &state.graphics;

    graphics.set_texture_options(state.handle, &state.options.sampler);
  }

  /// Initializes the texture with the given width and height.
  ///
  /// This is only necessary if the texture requires sizing information prior to access from the GPU.
  pub fn initialize(&mut self, width: u32, height: u32, format: TextureFormat) {
    let mut state = self.state.borrow_mut();

    state.width = width;
    state.height = height;

    let graphics = &state.graphics;

    graphics.initialize_texture(state.handle, width, height, format);
  }

  /// Downloads pixel data from the texture.
  #[profiling::function]
  pub fn read_pixels<T>(&self) -> Vec<T>
  where T: Texel {
    let state = self.state.borrow();
    let graphics = &state.graphics;
    let size = state.width as usize * state.height as usize;

    let mut buffer = Vec::<T>::with_capacity(size);

    unsafe {
      buffer.set_len(size);

      graphics.read_texture_data(
        state.handle,
        size * std::mem::size_of::<T>(),
        T::FORMAT,
        buffer.as_mut_ptr() as *mut u8,
        0, // mip level
      );
    }

    buffer
  }

  /// Uploads pixel data to the texture.
  #[profiling::function]
  pub fn write_pixels<T>(&mut self, width: usize, height: usize, pixels: &[T])
  where T: Texel {
    let mut state = self.state.borrow_mut();

    state.width = width as u32;
    state.height = height as u32;

    let graphics = &state.graphics;

    graphics.write_texture_data(
      state.handle,
      width as u32,
      height as u32,
      match pixels.len() {
        0 => std::ptr::null(),
        _ => pixels.as_ptr() as *const u8,
      },
      state.options.format,
      T::FORMAT,
      0, // mip level
    );
  }

  /// Uploads a sub-section of pixel data to the texture.
  #[profiling::function]
  pub fn write_sub_pixels<T>(&self, region: &Rectangle<usize>, pixels: &[T])
  where T: Texel {
    let state = self.state.borrow();
    let graphics = &state.graphics;

    graphics.write_texture_sub_data(
      state.handle,
      region,
      pixels.as_ptr() as *const u8,
      T::FORMAT,
      0, // mip level
    );
  }

  /// Uploads pixel data to the texture from the given [`Image`].
  pub fn write_image(&mut self, image: &Image) {
    self.write_pixels(image.width(), image.height(), image.as_slice());
  }
}

impl GraphicsResource for Texture {
  /// Returns the underlying graphics handle of the texture.
  fn handle(&self) -> GraphicsHandle {
    self.state.borrow().handle
  }
}

impl Drop for TextureState {
  /// Deletes the texture from the GPU.
  fn drop(&mut self) {
    self.graphics.delete_texture(self.handle);
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
    let image = context.load_asset(context.path)?;
    let mut texture = Texture::new(&self.graphics);

    texture.write_image(&image);

    Ok(texture)
  }
}

/// Represents a sub-region of a `Texture`.
#[derive(Clone)]
pub struct TextureRegion<'a> {
  pub texture: &'a Texture,
  pub offset: Vector2<u32>,
  pub size: Vector2<u32>,
}

impl<'a> TextureRegion<'a> {
  /// Calculates the UV rectangle for the given texture region.
  pub fn calculate_uv(&self) -> Rectangle<f32> {
    let left = self.offset.x as f32 / self.texture.width() as f32;
    let top = self.offset.y as f32 / self.texture.height() as f32;
    let right = (self.offset.x + self.size.x) as f32 / self.texture.width() as f32;
    let bottom = (self.offset.y + self.size.y) as f32 / self.texture.height() as f32;

    Rectangle::from_corner_points(left, top, right, bottom)
  }
}

impl<'a, R: AsRef<Texture>> From<&'a R> for TextureRegion<'a> {
  fn from(texture: &'a R) -> Self {
    let texture = texture.as_ref();

    TextureRegion {
      texture,
      offset: Vector2::ZERO,
      size: vec2(texture.width(), texture.height()),
    }
  }
}

/// An atlas of textures, which is a subdivison of a texture into a smaller grid of [`TextureRegion`]s.
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
      texture: &self.texture,
      offset: vec2(x * self.width, y * self.height),
      size: vec2(self.width, self.height),
    }
  }
}

/// A utility for building texture atlases procedurally.
#[derive(Default)]
pub struct TextureAtlasBuilder<T> {
  cells: Vec<TextureAtlasCell<T>>,
  cell_size: Vector2<u32>,
  next_offset: Vector2<u32>,
  stride: usize,
}

/// Represents a discrete cell in a `TextureAtlasBuilder`.
pub struct TextureAtlasCell<T> {
  pub pixels: Grid<T>,
  pub offset: Vector2<u32>,
  pub size: Vector2<u32>,
}

impl<P: Texel + Clone + Default> TextureAtlasBuilder<P> {
  /// Creates a new texture atlas builder.
  pub fn new(stride: usize, cell_size: Vector2<u32>) -> Self {
    Self {
      cells: Vec::new(),
      cell_size,
      next_offset: vec2(0, 0),
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
    self.next_offset += self.cell_size;

    if self.next_offset.x as usize >= self.stride {
      self.next_offset.x = 0;
      self.next_offset.y += self.cell_size.y;
    }

    self.cells.last_mut().unwrap()
  }

  /// Writes this builder's contents to the given texture.
  pub fn write_to(&self, texture: &mut Texture) {
    let cells_x = self.cells.len() % self.stride;
    let cells_y = self.cells.len() / self.stride;

    let pixels_x = cells_x * self.cell_size.x as usize;
    let pixels_y = cells_y * self.cell_size.y as usize;

    let mut advance_x = 0;
    let mut advance_y = 0;

    let mut pixels = vec![P::default(); pixels_x * pixels_y];

    for cell in &self.cells {
      for y in 0..cell.size.y {
        for x in 0..cell.size.x {
          let dest_index = advance_x + advance_y * pixels_x;

          pixels[dest_index] = cell.pixels.get((x, y)).clone();

          advance_x += 1;

          if advance_x >= pixels_x {
            advance_x = 0;
            advance_y += 1;
          }
        }

        advance_y += 1;
      }
    }

    texture.write_pixels(pixels_x, pixels_y, &pixels);
  }
}

/// Indicates a kind of pixel that can be used in a texture.
pub trait Texel {
  const FORMAT: TextureFormat;
}

/// Implements texel representation common pixel types..
macro_rules! implement_texel {
  ($type:ty, $value:ident) => {
    impl Texel for $type {
      const FORMAT: TextureFormat = TextureFormat::$value;
    }
  };
}

implement_texel!(Color, RGBA32);
implement_texel!(Color32, RGBA8);
implement_texel!([u8; 1], R8);
implement_texel!([u8; 2], RG8);
implement_texel!([u8; 3], RGB8);
implement_texel!([u8; 4], RGBA8);
implement_texel!((u8,), R8);
implement_texel!((u8, u8), RG8);
implement_texel!((u8, u8, u8), RGB8);
implement_texel!((u8, u8, u8, u8), RGBA8);
implement_texel!([f32; 1], R8);
implement_texel!([f32; 2], RG8);
implement_texel!([f32; 3], RGB8);
implement_texel!([f32; 4], RGBA8);
implement_texel!((f32,), R32);
implement_texel!((f32, f32), RG32);
implement_texel!((f32, f32, f32), RGB32);
implement_texel!((f32, f32, f32, f32), RGBA32);
