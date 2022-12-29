//! A simple set of tools for rapid prototyping of game ideas and etc.

use std::any::Any;

use core::graphics::*;
use core::maths::Mat4;
use core::utilities::Object;
pub use pixels::*;
pub use tiles::*;

mod pixels;
mod tiles;

/// A uniform that contains the [`ColorPalette`] texture for sprite rendering.
pub const UNIFORM_PALETTE: UniformKey<&Texture> = UniformKey::new("u_palette");

/// A uniform that contains the width of the value in [`UNIFORM_PALETTE`].
pub const UNIFORM_PALETTE_WIDTH: UniformKey<u32> = UniformKey::new("u_paletteWidth");

/// A uniform that contains the projection-view matrix for perspective adjustment.
pub const UNIFORM_PROJECTION_VIEW: UniformKey<&Mat4> = UniformKey::new("u_projectionView");

/// Represents one of the built-in shaders.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BuiltInShader {
  /// Standard purpose screen-size sprite shader.
  Canvas,
  /// Simple purpose projected sprite shader.
  SpriteStandard,
  /// Palette-shifted sprite shader.
  SpritePalette,
  /// Shader for wire rendering and basic geometry.
  Wire,
  /// A simple screen-space aberration effect.
  AberrationEffect,
}

// built-in shaders
const SHADER_CANVAS_STANDARD: &str = include_str!("../assets/shaders/canvas-standard.glsl");
const SHADER_SPRITE_STANDARD: &str = include_str!("../assets/shaders/sprite-standard.glsl");
const SHADER_SPRITE_PALETTE: &str = include_str!("../assets/shaders/sprite-palette.glsl");
const SHADER_WIRE_STANDARD: &str = include_str!("../assets/shaders/wire-standard.glsl");
const SHADER_EFFECT_ABERRATION: &str = include_str!("../assets/shaders/effect-aberration.glsl");

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

// built-in palettes
const PALETTE_AYY_4: &[u8] = include_bytes!("../assets/palettes/ayy-4.pal");
const PALETTE_DEMICHROME_4: &[u8] = include_bytes!("../assets/palettes/demichrome-4.pal");
const PALETTE_HOLLOW_4: &[u8] = include_bytes!("../assets/palettes/hollow-4.pal");
const PALETTE_KULE_16: &[u8] = include_bytes!("../assets/palettes/kule-16.pal");
const PALETTE_LOW_8: &[u8] = include_bytes!("../assets/palettes/low-8.pal");
const PALETTE_SPACE_DUST_9: &[u8] = include_bytes!("../assets/palettes/space-dust-9.pal");

/// Loads a built-in [`ShaderProgram`].
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

/// Loads a built-in [`Material`].
pub fn load_built_in_material(graphics: &GraphicsServer, shader: BuiltInShader) -> Material {
  Material::new(graphics, &load_built_in_shader(graphics, shader))
}

/// Loads a built-in [`ColorPalette`].
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

/// A [`RenderContextDescriptor`] for a simple [`SpriteBatchContext`] for use in sprite rendering.
pub struct SpriteBatchDescriptor {
  /// A default projection-view matrix to apply.
  pub projection_view: Mat4,

  /// If a palette is specified, a special shader variant will be loaded that uses the palette.
  /// The palette will be bound to `u_palette` with `u_paletteWidth` texels wide.
  pub palette: Option<ColorPalette<Color>>,

  /// A custom [`ShaderProgram`] to use for rendering.
  pub shader: Option<ShaderProgram>,

  /// The expected number of sprites to use in the batch; used for pre-sizing the batch vertex buffer.
  pub sprite_count: usize,
}

impl Default for SpriteBatchDescriptor {
  fn default() -> Self {
    Self {
      projection_view: Mat4::IDENTITY,
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
        // we need a special variant if we're using palette shifting effects
        None => load_built_in_shader(graphics, BuiltInShader::SpriteStandard),
        Some(_) => load_built_in_shader(graphics, BuiltInShader::SpritePalette),
      },
    };

    // prepare the material and sprite batch
    let mut material = Material::new(graphics, &shader);
    let batch = SpriteBatch::with_capacity(graphics, self.sprite_count);

    // prepare the palette texture, if enabled, upload it once
    if let Some(palette) = &self.palette {
      let palette_texture = Texture::new(graphics);

      palette_texture.write_pixels(palette.len(), 1, palette.as_slice());

      material.set_texture(UNIFORM_PALETTE, &palette_texture, None);
      material.set_uniform(UNIFORM_PALETTE_WIDTH, palette.len() as u32);
    }

    // apply the default projection-view matrix
    material.set_uniform(UNIFORM_PROJECTION_VIEW, &self.projection_view);

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
  /// A [`Material`] configured to render sprites.
  pub material: Material,

  /// The [`SpriteBatch`] to use for sprite geometry.
  pub batch: SpriteBatch,
}

impl RenderContext for SpriteBatchContext {
  fn on_begin_frame(&mut self) {
    self.batch.begin(&self.material);
  }

  fn on_end_frame(&mut self) {
    self.batch.flush();
  }
}

impl Object for SpriteBatchContext {
  fn as_any(&self) -> &dyn Any {
    self as &dyn Any
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as &mut dyn Any
  }
}

/// A descriptor for the `GeometryBatch`.
pub struct GeometryBatchDescriptor {
  /// A default projection-view matrix to apply.
  pub projection_view: Mat4,

  /// A custom shader program to use for rendering.
  pub shader: Option<ShaderProgram>,
}

impl Default for GeometryBatchDescriptor {
  fn default() -> Self {
    Self {
      projection_view: Mat4::IDENTITY,
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
    material.set_uniform(UNIFORM_PROJECTION_VIEW, &self.projection_view);

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
  fn on_begin_frame(&mut self) {
    self.batch.begin(&self.material);
  }

  fn on_end_frame(&mut self) {
    self.batch.flush();
  }
}

impl Object for GeometryBatchContext {
  fn as_any(&self) -> &dyn Any {
    self as &dyn Any
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as &mut dyn Any
  }
}
