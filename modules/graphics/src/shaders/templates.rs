use super::*;

/// A templated shader program that can be used to generate new shader programs.
pub struct ShaderTemplate {
  pub name: &'static str,
  kernels: Vec<ShaderKernel>,
}

impl ShaderTemplate {
  /// Loads a shader template from the given raw shader code.
  pub const fn load(name: &'static str, _code: &'static str) -> Self {
    ShaderTemplate {
      name,
      kernels: Vec::new(),
    }
  }

  /// Converts the template into a shader program.
  pub fn to_program(&self, graphics: &GraphicsEngine) -> common::Result<ShaderProgram> {
    ShaderProgram::from_kernels(&graphics, &self.kernels)
  }
}

/// Loads a shader from the given path.
#[macro_export]
macro_rules! include_shader {
  ($path:expr) => {
    ShaderTemplate::load($path, include_str!($path))
  };
}

// Internal shader code.
pub const SHADER_CANVAS_STANDARD: ShaderTemplate = include_shader!("../../shaders/canvas-standard.glsl");
pub const SHADER_MESH_SKINNED: ShaderTemplate = include_shader!("../../shaders/mesh-skinned.glsl");
pub const SHADER_SPRITE_STANDARD: ShaderTemplate = include_shader!("../../shaders/sprite-standard.glsl");
pub const SHADER_SPRITE_MULTITEX: ShaderTemplate = include_shader!("../../shaders/sprite-multitex.glsl");
