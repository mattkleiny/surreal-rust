//! Compute shader abstractions for the engine.
//!
//! Compute programs allow for the execution of arbitrary code on the GPU.

use super::{ShaderUniform, Texture, TextureFormat};

/// Indicates the kinds of barriers that can be synchronized in the GPU compute system.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ComputeBarrier {
  ImageAccess,
}

/// Different read/write modes for compute operations.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReadWriteMode {
  ReadOnly,
  WriteOnly,
  ReadWrite,
}

/// A compute image allows bound access to a texture image from compute shaders.
#[derive(Clone)]
pub struct TextureBinding {
  pub texture: Texture,
  pub mode: ReadWriteMode,
  pub format: TextureFormat,
}

impl From<TextureBinding> for ShaderUniform {
  fn from(image: TextureBinding) -> Self {
    ShaderUniform::TextureBinding(image.texture, 0, image.mode, image.format)
  }
}
