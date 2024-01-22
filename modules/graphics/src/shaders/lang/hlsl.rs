//! HLSL language support for the shader system

use super::*;

/// The HLSL [`ShaderLanguage`] implementation.
pub struct HLSL;

impl ShaderProgram {
  /// Loads a [`ShaderProgram`] from the given raw HLSL shader code.
  pub fn from_hlsl(graphics: &GraphicsEngine, code: &str) -> common::Result<Self> {
    Self::from_code::<HLSL>(graphics, code)
  }

  /// Loads a [`ShaderProgram`] from the given raw HLSL shader code file.
  pub fn from_hlsl_path<'a>(graphics: &GraphicsEngine, path: impl Into<VirtualPath<'a>>) -> common::Result<Self> {
    Self::from_path::<HLSL>(graphics, path)
  }

  /// Loads a [`ShaderProgram`] from the given raw HLSL stream.
  pub fn from_hlsl_stream<'a>(graphics: &GraphicsEngine, stream: &mut dyn InputStream) -> common::Result<Self> {
    Self::from_stream::<HLSL>(graphics, stream)
  }
}

impl ShaderLanguage for HLSL {
  /// Parses the given raw HLSL source and performs some basic pre-processing.
  fn parse_kernels(_source_code: &str) -> common::Result<Vec<ShaderKernel>> {
    // TODO: implement support for HLSL
    todo!()
  }
}
