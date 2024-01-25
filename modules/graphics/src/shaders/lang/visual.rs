use super::*;

/// The Visual [`ShaderLanguage`] implementation.
pub struct Visual;

impl ShaderProgram {
  /// Loads a [`ShaderProgram`] from the given Visual Shader blueprint.
  pub fn from_visual(graphics: &GraphicsEngine, code: &str) -> Result<Self, ShaderError> {
    Self::from_code::<Visual>(graphics, code)
  }

  /// Loads a [`ShaderProgram`] from the given raw Visual Shader file.
  pub fn from_visual_path<'a>(graphics: &GraphicsEngine, path: impl ToVirtualPath) -> Result<Self, ShaderError> {
    Self::from_path::<Visual>(graphics, path)
  }

  /// Loads a [`ShaderProgram`] from the given raw Visual Shader stream.
  pub fn from_visual_stream(graphics: &GraphicsEngine, stream: &mut dyn InputStream) -> Result<Self, ShaderError> {
    Self::from_stream::<Visual>(graphics, stream)
  }
}

impl ShaderLanguage for Visual {
  fn parse_kernels(_source_code: &str) -> Result<Vec<ShaderKernel>, ShaderError> {
    todo!()
  }
}
