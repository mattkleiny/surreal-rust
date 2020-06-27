// TODO: abstract over shading language, add debugging and profiling/etc?
// TODO: finish implementing the shady language.

use crate::RID;

#[cfg(feature = "shady")]
pub mod shady;

pub struct ShaderProgram {
  id: RID
}
