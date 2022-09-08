use super::Shader;

pub use shady::ShadyLanguage;

/// Represents a type of shader language that can be compiled at runtime.
pub trait ShaderLanguage {
  fn compile_shader(shader: &str) -> crate::Result<Vec<Shader>>;
}

mod shady {
  use super::*;

  /// Compiles Shady programs down to GLSL.
  ///
  /// Shady is a simple domain-specific shader language that compiles down to GLSL.
  pub struct ShadyLanguage;

  impl ShaderLanguage for ShadyLanguage {
    fn compile_shader(_shader: &str) -> crate::Result<Vec<Shader>> {
      todo!()
    }
  }
}
