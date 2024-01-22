//! Shady language support for the shader system

use super::*;

/// The Shady [`ShaderLanguage`] implementation.
pub struct Shady;

impl ShaderProgram {
  /// Loads a [`ShaderProgram`] from the given raw GLSL shader code.
  pub fn from_shady(graphics: &GraphicsEngine, code: &str) -> common::Result<Self> {
    Self::from_code::<Shady>(graphics, code)
  }

  /// Loads a [`ShaderProgram`] from the given raw shady shader code file.
  pub fn from_shady_path<'a>(graphics: &GraphicsEngine, path: impl Into<VirtualPath<'a>>) -> common::Result<Self> {
    Self::from_path::<Shady>(graphics, path)
  }

  /// Loads a [`ShaderProgram`] from the given raw shady stream.
  pub fn from_shady_stream<'a>(graphics: &GraphicsEngine, stream: &mut dyn InputStream) -> common::Result<Self> {
    Self::from_stream::<Shady>(graphics, stream)
  }
}

impl ShaderLanguage for Shady {
  /// Parses the given raw Shady source and performs some basic pre-processing.
  fn parse_kernels(_source_code: &str) -> common::Result<Vec<ShaderKernel>> {
    // TODO: implement support for Shady
    todo!()
  }
}
