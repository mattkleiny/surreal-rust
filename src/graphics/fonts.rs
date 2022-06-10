//! Font loading, management and rendering.
//!
//! We currently support bitmap fonts, with a planned future change to support TrueType fonts.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use ab_glyph::{Font as AbFont, FontVec};

use crate::assets::{Asset, AssetContext, AssetLoader, Handle};
use crate::graphics::{Texture, TextureRegion};
use crate::maths::{vec2, Vector2};

use super::{Color32, GraphicsServer, TextureAtlasBuilder};

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
  pub glyph_width: u16,
  pub glyph_height: u16,
  pub glyph_padding: u16,
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

    let x = (character as u16 % metrics.columns) * (metrics.glyph_width + metrics.glyph_padding);
    let y = (character as u16 / metrics.columns) * (metrics.glyph_height + metrics.glyph_padding);

    let offset = vec2(x as u32, y as u32);
    let size = vec2(metrics.glyph_width as u32, metrics.glyph_height as u32);

    Some(TextureRegion {
      texture: &self.texture,
      offset,
      size,
    })
  }
}

/// An `AssetLoader` for `BitmapFont`s.
pub struct BitmapFontLoader {}

impl Asset for BitmapFont {
  type Loader = BitmapFontLoader;
}

impl AssetLoader<BitmapFont> for BitmapFontLoader {
  fn load(&self, context: &AssetContext) -> crate::Result<BitmapFont> {
    let metrics: BitmapFontMetrics = context.path.deserialize_json()?;
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
  pub offset: Vector2<u32>,
  pub size: Vector2<u32>,
}

impl Font for VectorFont {
  fn measure_size(&self, text: &str) -> (f32, f32) {
    let state = self.state.borrow();
    let mut size = (0., 0.);

    for character in text.chars() {
      let glyph = state.font.glyph_id(character).with_scale(state.font_size);
      let bounds = state.font.glyph_bounds(&glyph);

      size.0 += bounds.max.x - bounds.min.x;
      size.1 += bounds.max.y - bounds.min.y;
    }

    size
  }

  fn get_glyph(&self, character: char) -> Option<TextureRegion> {
    // check if we've already built this glyph before
    let state = self.state.borrow();

    if let Some(glyph_info) = state.glyphs.get(&character) {
      return Some(TextureRegion {
        texture: &self.texture,
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
          let position = (x as usize, y as usize);

          cell.pixels.set(position, color);
        });

        GlyphInfo {
          offset: cell.offset,
          size: cell.size,
        }
      };

      state.atlas.write_to(&self.texture);

      // insert into the cache
      let region = TextureRegion {
        texture: &self.texture,
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

/// An `AssetLoader` for `TrueTypeFont`s.
pub struct VectorFontLoader {
  pub graphics: GraphicsServer,
  pub font_size: f32,
  pub atlas_stride: usize,
  pub atlas_size: Vector2<u32>,
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
        atlas: TextureAtlasBuilder::new(self.atlas_stride, self.atlas_size),
        glyphs: HashMap::new(),
      })),
    };

    Ok(font)
  }
}
