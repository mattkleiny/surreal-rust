#[derive(Error, Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShaderError {
  InvalidSyntax,
  InvalidPrecision,
  InvalidOperation,
  Unexpected,
}

pub trait ShaderCompiler {
  fn compile_shader(&self, shader: &Shader) -> Result<ShaderError, String>;
}

pub struct GlslCompiler {
  pub version: u32,
  pub profile: u32,
}

impl ShaderCompiler for GlslCompiler {

}
