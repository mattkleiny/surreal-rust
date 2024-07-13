//! Shader language abstractions and templating.

pub use glsl::*;
pub use shady::*;

use super::*;

mod glsl;
mod shady;

/// Represents a language for [`ShaderKernel`]s.
pub trait ShaderLanguage {
  /// Parses the given raw source code into one or more [`ShaderKernel`]s.
  fn parse_kernels(source_code: &str) -> Result<Vec<ShaderKernel>, ShaderError>;
}
