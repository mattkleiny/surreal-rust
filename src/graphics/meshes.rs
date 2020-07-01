use crate::graphics::{Buffer, BufferKind, BufferUsage};

/// Represents a mesh of vertices that can be rendered to a graphics device.
pub struct Mesh<V> {
  vertices: Buffer<V>,
  indices: Buffer<u16>,
}

impl<V> Mesh<V> {
  pub fn new_empty() -> Self {
    Self {
      vertices: Buffer::new(BufferKind::Element, BufferUsage::Static),
      indices: Buffer::new(BufferKind::Index, BufferUsage::Static),
    }
  }
}
