//! Shady language support for the shader system

use super::*;

/// The Shady [`ShaderLanguage`] implementation.
pub struct ShadyShaderLanguage;

impl ShaderLanguage for ShadyShaderLanguage {
  fn parse_kernels(_source_code: &str) -> common::Result<Vec<ShaderKernel>> {
    todo!()
  }
}
