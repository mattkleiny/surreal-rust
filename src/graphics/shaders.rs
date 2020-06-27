// TODO: abstract over shading language, add debugging and profiling/etc?
// TODO: finish implementing the shady language.

use crate::RID;

#[cfg(feature = "scripting-shady")]
pub mod shady;

#[derive(Debug, Eq, PartialEq)]
pub struct Shader {
  id: RID
}

#[derive(Debug, Eq, PartialEq)]
pub struct ShaderProgram {
  id: RID
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ShaderKind {
  Vertex,
  Fragment,
}