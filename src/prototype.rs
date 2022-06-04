//! A simple set of tools for rapid prototyping of game ideas and etc.

pub use canvas::*;
pub use tiles::*;

use crate::graphics::*;
use crate::maths::Matrix4x4;

mod canvas;
mod tiles;

// built-in shaders
const SHADER_SPRITE_STANDARD: &str = include_str!("../assets/shaders/sprite-standard.glsl");
const SHADER_SPRITE_PALETTE: &str = include_str!("../assets/shaders/sprite-palette.glsl");
const SHADER_EFFECT_ABERRATION: &str = include_str!("../assets/shaders/effect-aberration.glsl");

// built-in palettes
const PALETTE_AYY_4: &[u8] = include_bytes!("../assets/palettes/ayy-4.pal");
const PALETTE_DEMICHROME_4: &[u8] = include_bytes!("../assets/palettes/demichrome-4.pal");
const PALETTE_HOLLOW_4: &[u8] = include_bytes!("../assets/palettes/hollow-4.pal");
const PALETTE_KULE_16: &[u8] = include_bytes!("../assets/palettes/kule-16.pal");
const PALETTE_LOW_8: &[u8] = include_bytes!("../assets/palettes/low-8.pal");
const PALETTE_SPACE_DUST_9: &[u8] = include_bytes!("../assets/palettes/space-dust-9.pal");

/// Represents one of the built-in shaders.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BuiltInShader {
  SpriteStandard,
  SpritePalette,
  AberrationEffect,
}

/// Represents one of the built-in color palettes.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BuiltInPalette {
  Ayy4,
  Demichrome4,
  Hollow4,
  Kule16,
  Low8,
  SpaceDust9,
}

/// Loads the standard shader program from embedded resources.
pub fn load_built_in_shader(server: &GraphicsServer, shader: BuiltInShader) -> ShaderProgram {
  let shader = match shader {
    BuiltInShader::SpriteStandard => ShaderProgram::from_glsl(server, SHADER_SPRITE_STANDARD),
    BuiltInShader::SpritePalette => ShaderProgram::from_glsl(server, SHADER_SPRITE_PALETTE),
    BuiltInShader::AberrationEffect => ShaderProgram::from_glsl(server, SHADER_EFFECT_ABERRATION),
  };

  shader.expect("Failed to load standard shader")
}

/// Loads the given built-in color palette.
pub fn load_built_in_palette<P: Pixel>(palette: BuiltInPalette) -> ColorPalette<P> {
  let palette = match palette {
    BuiltInPalette::Ayy4 => ColorPalette::from_bytes(PALETTE_AYY_4),
    BuiltInPalette::Demichrome4 => ColorPalette::from_bytes(PALETTE_DEMICHROME_4),
    BuiltInPalette::Hollow4 => ColorPalette::from_bytes(PALETTE_HOLLOW_4),
    BuiltInPalette::Kule16 => ColorPalette::from_bytes(PALETTE_KULE_16),
    BuiltInPalette::Low8 => ColorPalette::from_bytes(PALETTE_LOW_8),
    BuiltInPalette::SpaceDust9 => ColorPalette::from_bytes(PALETTE_SPACE_DUST_9),
  };

  palette.expect("Failed to load standard palette")
}

/// A descriptor for the `SpriteContext`.
pub struct SpriteBatchDescriptor {
  /// A default projection-view matrix to apply.
  pub projection_view: Matrix4x4<f32>,

  /// If a palette is specified, a special shader variant will be loaded that uses the palette.
  pub palette: Option<ColorPalette<Color>>,

  /// The expected number of sprites to use in the batch; used for pre-sizing the batch vertex buffer.
  pub sprite_count: usize,
}

impl Default for SpriteBatchDescriptor {
  fn default() -> Self {
    Self {
      projection_view: Matrix4x4::identity(),
      palette: None,
      sprite_count: 1024,
    }
  }
}

impl RenderContextDescriptor for SpriteBatchDescriptor {
  type Context = SpriteBatchContext;

  fn create(&self, server: &GraphicsServer) -> Self::Context {
    // determine which shader we're using, prepare material
    let shader = match self.palette {
      None => BuiltInShader::SpriteStandard,
      Some(_) => BuiltInShader::SpritePalette,
    };

    let mut material = Material::new(server, &load_built_in_shader(server, shader));
    let batch = SpriteBatch::with_capacity(server, self.sprite_count);

    // prepare the palette texture, if enabled
    if let Some(palette) = &self.palette {
      let mut palette_texture = Texture::new(server);

      palette_texture.write_pixels(palette.len(), 1, palette.as_slice());

      material.set_texture("u_palette", &palette_texture, 1, None);
      material.set_uniform("u_paletteWidth", palette.len() as u32);
    }

    // apply the default projection-view matrix
    material.set_uniform("u_projectionView", &self.projection_view);

    // enable alpha blending
    material.set_blend_state(BlendState::Enabled {
      source: BlendFactor::SrcAlpha,
      destination: BlendFactor::OneMinusSrcAlpha,
    });

    Self::Context { material, batch }
  }
}

/// A simple [`RenderContext`] that allows for sprite rendering using the standard sprite shaders.
pub struct SpriteBatchContext {
  /// A material configured to render sprites.
  pub material: Material,

  /// The sprite batch to use for sprite geometry.
  pub batch: SpriteBatch,
}

impl RenderContext for SpriteBatchContext {
  fn on_before_with(&mut self) {
    self.batch.begin(&self.material);
  }

  fn on_after_with(&mut self) {
    self.batch.flush();
  }
}
