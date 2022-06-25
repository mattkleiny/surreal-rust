//! A simple set of tools for rapid prototyping of game ideas and etc.

pub use canvas::*;
pub use sprites::*;
pub use tiles::*;

use crate::graphics::*;
use crate::maths::Matrix4x4;

mod canvas;
mod sprites;
mod tiles;

// built-in shaders
const SHADER_CANVAS_STANDARD: &str = include_str!("../assets/shaders/canvas-standard.glsl");
const SHADER_SPRITE_STANDARD: &str = include_str!("../assets/shaders/sprite-standard.glsl");
const SHADER_SPRITE_PALETTE: &str = include_str!("../assets/shaders/sprite-palette.glsl");
const SHADER_WIRE_STANDARD: &str = include_str!("../assets/shaders/wire-standard.glsl");
const SHADER_EFFECT_ABERRATION: &str = include_str!("../assets/shaders/effect-aberration.glsl");

// built-in palettes
const PALETTE_AYY_4: &[u8] = include_bytes!("../assets/palettes/ayy-4.pal");
const PALETTE_DEMICHROME_4: &[u8] = include_bytes!("../assets/palettes/demichrome-4.pal");
const PALETTE_HOLLOW_4: &[u8] = include_bytes!("../assets/palettes/hollow-4.pal");
const PALETTE_KULE_16: &[u8] = include_bytes!("../assets/palettes/kule-16.pal");
const PALETTE_LOW_8: &[u8] = include_bytes!("../assets/palettes/low-8.pal");
const PALETTE_SPACE_DUST_9: &[u8] = include_bytes!("../assets/palettes/space-dust-9.pal");

// built-in fonts
const FONT_BIT536: &[u8] = include_bytes!("../assets/fonts/bit536_v1.otf");
const FONT_BITBOY_8: &[u8] = include_bytes!("../assets/fonts/bitboy8_v1.otf");

/// Represents one of the built-in shaders.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BuiltInShader {
  /// Standard purpose screen-size sprite shader.
  Canvas,
  /// Standard purpose projected sprite shader.
  SpriteStandard,
  /// Palette-shifted sprite shader.
  SpritePalette,
  /// Shader for wire rendering and basic geometry.
  Wire,
  /// A simple screen-space aberration effect.
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

/// Represents one of the built-in fonts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BuiltInFont {
  Bit536,
  BitBoy8,
}

/// Loads a built-in shader.
pub fn load_built_in_shader(graphics: &GraphicsServer, shader: BuiltInShader) -> ShaderProgram {
  let shader = match shader {
    BuiltInShader::Canvas => ShaderProgram::from_glsl(graphics, SHADER_CANVAS_STANDARD),
    BuiltInShader::SpriteStandard => ShaderProgram::from_glsl(graphics, SHADER_SPRITE_STANDARD),
    BuiltInShader::SpritePalette => ShaderProgram::from_glsl(graphics, SHADER_SPRITE_PALETTE),
    BuiltInShader::Wire => ShaderProgram::from_glsl(graphics, SHADER_WIRE_STANDARD),
    BuiltInShader::AberrationEffect => ShaderProgram::from_glsl(graphics, SHADER_EFFECT_ABERRATION),
  };

  shader.expect("Failed to load build-in shader")
}

/// Loads a built-in material.
pub fn load_built_in_material(graphics: &GraphicsServer, shader: BuiltInShader) -> Material {
  Material::new(graphics, &load_built_in_shader(graphics, shader))
}

/// Loads a built-in color palette.
pub fn load_built_in_palette<P: Pixel>(palette: BuiltInPalette) -> ColorPalette<P> {
  let palette = match palette {
    BuiltInPalette::Ayy4 => ColorPalette::from_bytes(PALETTE_AYY_4),
    BuiltInPalette::Demichrome4 => ColorPalette::from_bytes(PALETTE_DEMICHROME_4),
    BuiltInPalette::Hollow4 => ColorPalette::from_bytes(PALETTE_HOLLOW_4),
    BuiltInPalette::Kule16 => ColorPalette::from_bytes(PALETTE_KULE_16),
    BuiltInPalette::Low8 => ColorPalette::from_bytes(PALETTE_LOW_8),
    BuiltInPalette::SpaceDust9 => ColorPalette::from_bytes(PALETTE_SPACE_DUST_9),
  };

  palette.expect("Failed to load built-in palette")
}

/// Loads a built-in font.
pub fn load_built_in_font(_font: BuiltInFont) -> VectorFont {
  todo!();

  // let font: crate::Result<VectorFont> = match font {
  //   BuiltInFont::Bit536 => todo!(),
  //   BuiltInFont::BitBoy8 => todo!(),
  // };

  // font.expect("Failed to load build-in font")
}

/// A descriptor for the `SpriteContext`.
pub struct SpriteBatchDescriptor {
  /// A default projection-view matrix to apply.
  pub projection_view: Matrix4x4,

  /// If a palette is specified, a special shader variant will be loaded that uses the palette.
  /// The palette will be bound to u_palette with u_paletteWidth texels wide.
  pub palette: Option<ColorPalette<Color>>,

  /// A custom shader program to use for rendering.
  pub shader: Option<ShaderProgram>,

  /// The expected number of sprites to use in the batch; used for pre-sizing the batch vertex buffer.
  pub sprite_count: usize,
}

impl Default for SpriteBatchDescriptor {
  fn default() -> Self {
    Self {
      projection_view: Matrix4x4::IDENTITY,
      palette: None,
      shader: None,
      sprite_count: 1024,
    }
  }
}

impl RenderContextDescriptor for SpriteBatchDescriptor {
  type Context = SpriteBatchContext;

  fn create(&self, graphics: &GraphicsServer) -> Self::Context {
    // determine which shader we're using, prepare material
    let shader = match &self.shader {
      Some(shader) => shader.clone(),
      None => match self.palette {
        None => load_built_in_shader(graphics, BuiltInShader::SpriteStandard),
        Some(_) => load_built_in_shader(graphics, BuiltInShader::SpritePalette),
      },
    };

    let mut material = Material::new(graphics, &shader);
    let batch = SpriteBatch::with_capacity(graphics, self.sprite_count);

    // prepare the palette texture, if enabled
    if let Some(palette) = &self.palette {
      let palette_texture = Texture::new(graphics);

      palette_texture.write_pixels(palette.len(), 1, palette.as_slice());

      material.set_texture("u_palette", &palette_texture, None);
      material.set_uniform("u_paletteWidth", palette.len() as u32);
    }

    // apply the default projection-view matrix
    material.set_uniform("u_projectionView", &self.projection_view);

    // enable default material state
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
  fn as_any(&self) -> &dyn std::any::Any {
    self as &dyn std::any::Any
  }

  fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
    self as &mut dyn std::any::Any
  }

  fn on_begin_frame(&mut self) {
    self.batch.begin(&self.material);
  }

  fn on_end_frame(&mut self) {
    self.batch.flush();
  }
}

/// A descriptor for the `GeometryBatch`.
pub struct GeometryBatchDescriptor {
  /// A default projection-view matrix to apply.
  pub projection_view: Matrix4x4,

  /// A custom shader program to use for rendering.
  pub shader: Option<ShaderProgram>,
}

impl Default for GeometryBatchDescriptor {
  fn default() -> Self {
    Self {
      projection_view: Matrix4x4::IDENTITY,
      shader: None,
    }
  }
}

impl RenderContextDescriptor for GeometryBatchDescriptor {
  type Context = GeometryBatchContext;

  fn create(&self, graphics: &GraphicsServer) -> Self::Context {
    // determine which shader we're using, prepare material
    let shader = match &self.shader {
      Some(shader) => shader.clone(),
      None => load_built_in_shader(graphics, BuiltInShader::Wire),
    };

    let mut material = Material::new(graphics, &shader);
    let batch = GeometryBatch::new(graphics);

    // apply the default projection-view matrix
    material.set_uniform("u_projectionView", &self.projection_view);

    // enable default material state
    material.set_blend_state(BlendState::Enabled {
      source: BlendFactor::SrcAlpha,
      destination: BlendFactor::OneMinusSrcAlpha,
    });

    Self::Context { material, batch }
  }
}

/// A simple [`RenderContext`] that allows for geometry rendering using the standard wire shaders.
pub struct GeometryBatchContext {
  /// A material configured to render geometry.
  pub material: Material,

  /// The geometry batch to use for wire geometry.
  pub batch: GeometryBatch,
}

impl RenderContext for GeometryBatchContext {
  fn as_any(&self) -> &dyn std::any::Any {
    self as &dyn std::any::Any
  }

  fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
    self as &mut dyn std::any::Any
  }

  fn on_begin_frame(&mut self) {
    self.batch.begin(&self.material);
  }

  fn on_end_frame(&mut self) {
    self.batch.flush();
  }
}
