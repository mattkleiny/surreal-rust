use super::{Buffer, BufferKind, BufferUsage};

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
