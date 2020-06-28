// TODO: abstract over shading language, add debugging and profiling/etc?
// TODO: finish implementing the shady language.

use crate::RID;
use std::collections::HashMap;

#[cfg(feature = "scripting-shady")]
pub mod shady;

/// Provides source code for shader compilation.
pub trait ShaderSource {
  fn get_source(&self) -> &[(ShaderKind, &[u8])];
}

/// Represents a single raw shader program.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Shader {
  source: String,
  kind: ShaderKind,
}

/// A combined set of shaders for a shader program.
#[derive(Clone, Debug)]
pub struct ShaderSet {
  shaders: HashMap<ShaderKind, Shader>,
}

impl ShaderSet {
  pub fn new() -> Self {
    unimplemented!()
  }
}

impl ShaderSource for ShaderSet {
  fn get_source(&self) -> &[(ShaderKind, &[u8])] {
    unimplemented!()
  }
}

/// Represents a single compiled shader program.
#[derive(Debug, Eq, PartialEq)]
pub struct ShaderProgram {
  id: RID
}

/// Different types fo shaders supported by the engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ShaderKind {
  Vertex,
  Fragment,
}
