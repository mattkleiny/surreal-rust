use crate::assets::{AssetContext, AssetResult, LoadableAsset};
use crate::graphics::{Buffer, BufferKind, BufferUsage};
use crate::io::Path;

/// Represents a mesh of vertices that can be rendered to a graphics device.
pub struct Mesh {
  vertices: Buffer,
  indices: Buffer,
}

impl Mesh {
  pub fn new_empty() -> Self {
    Self {
      vertices: Buffer::new(BufferKind::Element, BufferUsage::Static),
      indices: Buffer::new(BufferKind::Index, BufferUsage::Static),
    }
  }
}

impl LoadableAsset for Mesh {
  fn load(path: Path, context: &mut impl AssetContext) -> AssetResult<Self> {
    unimplemented!()
  }
}
