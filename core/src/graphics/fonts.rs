//! Font loading, management and rendering.
//!
//! We currently support bitmap fonts with basic support for vector fonts at a fixed scale.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use ab_glyph::{Font as AbFont, FontVec};

use crate::assets::{Asset, AssetContext, AssetLoader, Handle};
use crate::graphics::{Texture, TextureRegion};
use crate::io::Deserializable;
use crate::maths::{uvec2, UVec2};

use super::{Color32, GraphicsServer, Texel, TextureAtlasBuilder};

/// Represents different kinds of fonts and permits rendering.
pub trait Font {
  /// Measures the size of the given text in the font.
  fn measure_size(&self, text: &str) -> (f32, f32);

  /// Retrieves a texture region representing the given glyph in the font.
  fn get_glyph(&self, character: char) -> Option<TextureRegion>;
}

/// A font comprised of bitmap images for each glyph.
#[derive(Clone)]
pub struct BitmapFont {
  texture: Handle<Texture>,
  metrics: BitmapFontMetrics,
}

/// Describes the metrics for a bitmap font.
#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct BitmapFontMetrics {
  pub file_path: String,
  pub glyph_width: u32,
  pub glyph_height: u32,
  pub glyph_padding: u32,
  pub columns: u16,
}

impl Font for BitmapFont {
  fn measure_size(&self, text: &str) -> (f32, f32) {
    let mut line_count = 0;
    let mut longest_line = 0;
    let mut current_line = 0;

    for character in text.chars() {
      current_line += 1;

      if current_line >= longest_line {
        longest_line = current_line;
      }

      if character == '\n' {
        line_count += 1;
        current_line = 0;
      }
    }

    let metrics = &self.metrics;

    let width = longest_line * (metrics.glyph_width + metrics.glyph_padding);
    let height = line_count * (metrics.glyph_height + metrics.glyph_padding);

    return (width as f32, height as f32);
  }

  fn get_glyph(&self, character: char) -> Option<TextureRegion> {
    // we only support ascii glyphs at the moment
    if !character.is_ascii() {
      return None;
    }

    let metrics = &self.metrics;
    let character = character as u8;

    let x = (character as u16 % metrics.columns) as u32 * (metrics.glyph_width + metrics.glyph_padding);
    let y = (character as u16 / metrics.columns) as u32 * (metrics.glyph_height + metrics.glyph_padding);

    Some(TextureRegion {
      texture: self.texture.as_ref().clone(),
      offset: uvec2(x, y),
      size: uvec2(metrics.glyph_width, metrics.glyph_height),
    })
  }
}

/// An [`AssetLoader`] for [`BitmapFont`]s.
pub struct BitmapFontLoader {}

impl Asset for BitmapFont {
  type Loader = BitmapFontLoader;
}

impl AssetLoader<BitmapFont> for BitmapFontLoader {
  fn load(&self, context: &AssetContext) -> crate::Result<BitmapFont> {
    let metrics = BitmapFontMetrics::load_from_json(context.path)?;
    let texture: Handle<Texture> = context.load_asset(&metrics.file_path)?;

    let font = BitmapFont {
      texture: texture.clone(),
      metrics,
    };

    Ok(font)
  }
}

/// A vector font that can be rasterized at different font sizes.
#[derive(Clone)]
pub struct VectorFont {
  texture: Texture,
  state: Rc<RefCell<VectorFontState>>,
}

/// Internal state for a `VectorFont`.
struct VectorFontState {
  font: FontVec,
  font_size: f32,
  atlas: TextureAtlasBuilder<Color32>,
  glyphs: HashMap<char, GlyphInfo>,
}

/// Represents position information for a single glyph in a texture.
#[derive(Clone, Debug)]
struct GlyphInfo {
  pub offset: UVec2,
  pub size: UVec2,
}

impl Font for VectorFont {
  fn measure_size(&self, text: &str) -> (f32, f32) {
    let state = self.state.borrow();

    let mut line_count = 0;
    let mut longest_line = 0;
    let mut current_line = 0;

    for character in text.chars() {
      current_line += 1;

      if current_line >= longest_line {
        longest_line = current_line;
      }

      if character == '\n' {
        line_count += 1;
        current_line = 0;
      }
    }

    let width = longest_line as f32 * state.font_size;
    let height = line_count as f32 * state.font_size;

    return (width, height);
  }

  fn get_glyph(&self, character: char) -> Option<TextureRegion> {
    // check if we've already built this glyph before
    let state = self.state.borrow();

    if let Some(glyph_info) = state.glyphs.get(&character) {
      return Some(TextureRegion {
        texture: self.texture.clone(),
        offset: glyph_info.offset,
        size: glyph_info.size,
      });
    }

    // otherwise build a new glyph
    let glyph = state.font.glyph_id(character).with_scale(state.font_size);

    if let Some(outline) = state.font.outline_glyph(glyph) {
      drop(state); // reborrow mutably
      let mut state = self.state.borrow_mut();

      // allocate a new glyph in the texture
      let glyph_info = {
        let cell = state.atlas.allocate();

        // draw this glyph's pixels into our texture atlas
        outline.draw(|x, y, coverage| {
          let color = Color32::rgba(255, 255, 255, (coverage * 255.0) as u8);

          cell.pixels.set(x as i32, y as i32, color);
        });

        GlyphInfo {
          offset: cell.offset,
          size: cell.size,
        }
      };

      state.atlas.write_to(&self.texture);

      // insert into the cache
      let region = TextureRegion {
        texture: self.texture.clone(),
        offset: glyph_info.offset,
        size: glyph_info.size,
      };

      state.glyphs.insert(character, glyph_info);

      Some(region)
    } else {
      None
    }
  }
}

/// An [`AssetLoader`] for [`VectorFont`]s.
pub struct VectorFontLoader {
  pub graphics: GraphicsServer,
  pub font_size: f32,
  pub atlas_stride: usize,
  pub atlas_cell_size: UVec2,
}

impl Asset for VectorFont {
  type Loader = VectorFontLoader;
}

impl AssetLoader<VectorFont> for VectorFontLoader {
  fn load(&self, context: &AssetContext) -> crate::Result<VectorFont> {
    let bytes = context.path.read_all_bytes()?;

    let font = VectorFont {
      texture: Texture::new(&self.graphics),
      state: Rc::new(RefCell::new(VectorFontState {
        font: FontVec::try_from_vec(bytes)?,
        font_size: self.font_size,
        atlas: TextureAtlasBuilder::new(self.atlas_stride, self.atlas_cell_size),
        glyphs: HashMap::new(),
      })),
    };

    Ok(font)
  }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct Coverage(f32);

impl Texel for Coverage {
  const FORMAT: super::TextureFormat = super::TextureFormat::A32;
}
