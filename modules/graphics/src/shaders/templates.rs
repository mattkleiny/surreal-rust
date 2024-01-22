//! Template shaders and commonly used shader code.

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
  pub const fn load(code: &'static str) -> Self {
    ShaderTemplate {
      code,
      _phantom: std::marker::PhantomData,
    }
  }

  /// Converts the template into a shader program.
  pub fn to_program(&self, graphics: &GraphicsEngine) -> common::Result<ShaderProgram> {
    ShaderProgram::from_code::<S>(&graphics, &self.code)
  }
}

/// Loads a shader template from the given path.
#[macro_export]
macro_rules! include_shader {
  ($path:expr) => {
    ShaderTemplate::load(include_str!($path))
  };
}

// Embedded shader code.
#[rustfmt::skip]
#[allow(dead_code)]
pub(crate) mod embedded {
  use super::*;

  pub const SHADER_CANVAS_STANDARD: ShaderTemplate<GLSL> = include_shader!("./embedded/canvas-standard.glsl");
  pub const SHADER_MESH_SKINNED: ShaderTemplate<GLSL> = include_shader!("./embedded/mesh-skinned.glsl");
  pub const SHADER_SPRITE_STANDARD: ShaderTemplate<GLSL> = include_shader!("./embedded/sprite-standard.glsl");
  pub const SHADER_SPRITE_STANDARD_PALETTE: ShaderTemplate<GLSL> = include_shader!("./embedded/sprite-standard-palette.glsl");
  pub const SHADER_SPRITE_MULTITEX: ShaderTemplate<GLSL> = include_shader!("./embedded/sprite-multitex.glsl");
  pub const SHADER_SPRITE_MULTITEX_PALETTE: ShaderTemplate<GLSL> = include_shader!("./embedded/sprite-multitex-palette.glsl");
}
