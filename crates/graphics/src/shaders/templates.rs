#[cfg(feature = "shaderlib")]
pub use embedded::*;

use super::*;

/// A templated shader program that can be used to generate new
/// [`ShaderProgram`] instances at runtime.
///
/// Compilation will be performed on all calls to [`to_program`], so it is
/// recommended to cache the result if possible.
pub struct ShaderTemplate<S: ShaderLanguage> {
  code: &'static str,
  _phantom: std::marker::PhantomData<S>,
}

impl<S: ShaderLanguage> ShaderTemplate<S> {
  /// Loads a shader template from the given raw shader code.
  pub const fn new(code: &'static str) -> Self {
    ShaderTemplate {
      code,
      _phantom: std::marker::PhantomData,
    }
  }

  /// Converts the template into a shader program.
  pub fn to_program(&self) -> Result<ShaderProgram, ShaderError> {
    ShaderProgram::from_code::<S>(self.code)
  }
}

impl Material {
  /// Creates a new material from the given shader template.
  pub fn from_template<S: ShaderLanguage>(template: &ShaderTemplate<S>) -> Result<Self, ShaderError> {
    let program = template.to_program()?;

    Ok(Material::new(&program))
  }
}

/// Loads a shader template from the given path.
#[macro_export]
macro_rules! include_shader {
  ($path:expr) => {
    ShaderTemplate::new(include_str!($path))
  };
}

// Embedded shader code.
#[rustfmt::skip]
#[allow(dead_code)]
#[cfg(feature = "shaderlib")]
mod embedded {
  use super::*;

  pub const PROJECTION_VIEW: ShaderUniformKey<&Mat4> = ShaderUniformKey::new("u_projection_view");

  pub const SHADER_CANVAS_STANDARD: ShaderTemplate<GLSL> = include_shader!("./embedded/canvas-standard.glsl");
  pub const SHADER_MESH_SKINNED: ShaderTemplate<GLSL> = include_shader!("./embedded/mesh-skinned.glsl");
  pub const SHADER_SPRITE_STANDARD: ShaderTemplate<GLSL> = include_shader!("./embedded/sprite-standard.glsl");
  pub const SHADER_SPRITE_STANDARD_PALETTE: ShaderTemplate<GLSL> = include_shader!("./embedded/sprite-standard-palette.glsl");
  pub const SHADER_SPRITE_MULTITEX: ShaderTemplate<GLSL> = include_shader!("./embedded/sprite-multitex.glsl");
  pub const SHADER_SPRITE_MULTITEX_PALETTE: ShaderTemplate<GLSL> = include_shader!("./embedded/sprite-multitex-palette.glsl");
}
