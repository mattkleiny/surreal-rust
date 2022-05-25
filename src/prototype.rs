//! A simple set of tools for rapid prototyping of game ideas and etc.

pub use pixels::*;
pub use tiles::*;

use crate::graphics::*;

mod pixels;
mod tiles;

// built-in shaders
const SHADER_SPRITE_STANDARD: &'static str = include_str!("../assets/shaders/sprite-standard.glsl");
const SHADER_SPRITE_PALETTE: &'static str = include_str!("../assets/shaders/sprite-palette.glsl");
const SHADER_EFFECT_ABERRATION: &'static str = include_str!("../assets/shaders/effect-aberration.glsl");

// built-in palettes
const PALETTE_AYY_4: &'static [u8] = include_bytes!("../assets/palettes/ayy-4.pal");
const PALETTE_DEMICHROME_4: &'static [u8] = include_bytes!("../assets/palettes/demichrome-4.pal");
const PALETTE_HOLLOW_4: &'static [u8] = include_bytes!("../assets/palettes/hollow-4.pal");
const PALETTE_KULE_16: &'static [u8] = include_bytes!("../assets/palettes/kule-16.pal");
const PALETTE_LOW_8: &'static [u8] = include_bytes!("../assets/palettes/low-8.pal");
const PALETTE_SPACE_DUST_9: &'static [u8] = include_bytes!("../assets/palettes/space-dust-9.pal");

/// Represents one of the built-in shaders.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BuiltInShader {
  Sprite(BuiltInSpriteShader),
  Effect(BuiltInEffect),
}

/// Represents one of the built-in sprite shaders.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BuiltInSpriteShader {
  Standard,
  Palette,
}

/// Represents one of the built-in sprite shaders.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BuiltInEffect {
  Aberration,
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
pub fn load_standard_shader(server: &GraphicsServer, shader: BuiltInShader) -> ShaderProgram {
  let shader = match shader {
    BuiltInShader::Sprite(BuiltInSpriteShader::Standard) => ShaderProgram::from_string(server, SHADER_SPRITE_STANDARD),
    BuiltInShader::Sprite(BuiltInSpriteShader::Palette) => ShaderProgram::from_string(server, SHADER_SPRITE_PALETTE),
    BuiltInShader::Effect(BuiltInEffect::Aberration) => ShaderProgram::from_string(server, SHADER_EFFECT_ABERRATION),
  };

  shader.expect("Failed to load standard shader")
}

/// Loads the given built-in color palette.
pub fn load_standard_palette<P>(palette: BuiltInPalette) -> ColorPalette<P> where P: Pixel {
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
#[derive(Default)]
pub struct SpriteContextDescriptor {
  /// A color palette to use for rendering these sprites.
  ///
  /// If a palette is specified, a special shader variant will be loaded that uses the palette.
  pub palette: Option<ColorPalette<Color>>,
}

/// A simple [`RenderContext`] that allows for sprite rendering using the standard sprite shaders.
pub struct SpriteContext {
  /// A material configured to render sprites.
  pub material: Material,
  /// The sprite batch to use for sprite geometry.
  pub batch: SpriteBatch,
}

impl RenderContextDescriptor for SpriteContextDescriptor {
  type Context = SpriteContext;

  fn create(&self, server: &GraphicsServer) -> Self::Context {
    // determine which shader we're using, prepare material
    let shader = match self.palette {
      None => BuiltInShader::Sprite(BuiltInSpriteShader::Standard),
      Some(_) => BuiltInShader::Sprite(BuiltInSpriteShader::Palette),
    };

    let mut material = Material::new(server, &load_standard_shader(server, shader));
    let batch = SpriteBatch::new(server);

    // prepare the palette texture, if enabled
    if let Some(palette) = &self.palette {
      let mut palette_texture = Texture::new(server);

      palette_texture.write_pixels(palette.len(), 1, palette.as_slice());

      material.set_texture("u_palette", &palette_texture, 1, None);
      material.set_uniform("u_paletteWidth", palette.len() as f32);
    }

    // enable alpha blending
    material.set_blend_state(BlendState::Enabled {
      source: BlendFactor::SrcAlpha,
      destination: BlendFactor::OneMinusSrcAlpha,
    });

    Self::Context { material, batch }
  }
}

impl RenderContext for SpriteContext {}
