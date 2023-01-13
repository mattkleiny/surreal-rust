//! Scene nodes for 2d/3d graphics.

pub use sprites::*;
use surreal::{
  graphics::{GraphicsServer, ShaderProgram, Texture, UniformKey},
  maths::Mat4,
};

mod sprites;

/// A uniform that contains the [`ColorPalette`] texture for sprite rendering.
const UNIFORM_PALETTE: UniformKey<&Texture> = UniformKey::new("u_palette");

/// A uniform that contains the width of the value in [`UNIFORM_PALETTE`].
const UNIFORM_PALETTE_WIDTH: UniformKey<u32> = UniformKey::new("u_paletteWidth");

/// A uniform that contains the projection-view matrix for perspective adjustment.
const UNIFORM_PROJECTION_VIEW: UniformKey<&Mat4> = UniformKey::new("u_projectionView");

/// Represents one of the built-in shaders.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum BuiltInShader {
  /// Simple purpose projected sprite shader.
  SpriteStandard,
  /// Palette-shifted sprite shader.
  SpritePalette,
}

// built-in shaders
const SHADER_SPRITE_STANDARD: &str = include_str!("../assets/shaders/sprite-standard.glsl");
const SHADER_SPRITE_PALETTE: &str = include_str!("../assets/shaders/sprite-palette.glsl");

/// Loads a built-in [`ShaderProgram`].
fn load_built_in_shader(graphics: &GraphicsServer, shader: BuiltInShader) -> ShaderProgram {
  let program = match shader {
    BuiltInShader::SpriteStandard => ShaderProgram::from_glsl(graphics, SHADER_SPRITE_STANDARD),
    BuiltInShader::SpritePalette => ShaderProgram::from_glsl(graphics, SHADER_SPRITE_PALETTE),
  };

  program.expect(&format!("Failed to load build-in shader {:?}", shader))
}
