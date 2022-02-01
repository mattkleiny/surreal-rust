use std::rc::Rc;

use crate::graphics::{GraphicsHandle, GraphicsServer};

/// The different kinds of buffer we support.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BufferKind {
  Element,
  Index,
}

/// The usage pattern of the buffer.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BufferUsage {
  Static,
  Dynamic,
}

/// A buffer implementation based on OpenGL.
pub struct Buffer {
  server: Rc<dyn GraphicsServer>,
  handle: GraphicsHandle,
  kind: BufferKind,
  usage: BufferUsage,
}

/// Contains rendering attributes about a vertex.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexAttribute {
  pub name: String,
  pub binding: String,
  pub offset: usize,
  pub stride: usize,
}

/// Represents abstractly some buffer of data on the GPU.
impl Buffer {
  pub fn new(server: &Rc<dyn GraphicsServer>, kind: BufferKind, usage: BufferUsage) -> Self {
    let handle = server.create_buffer();

    Self {
      server: server.clone(),
      handle,
      kind,
      usage,
    }
  }

  pub fn kind(&self) -> BufferKind { self.kind }
  pub fn usage(&self) -> BufferUsage { self.usage }

  /// Reads data from the buffer.
  pub unsafe fn read_data<T>(&self) -> &[T] {
    todo!()
  }

  /// Uploads the given data to the buffer.
  pub unsafe fn write_data<T>(&mut self, data: &[T]) {
    todo!()
  }
}

impl Drop for Buffer {
  fn drop(&mut self) {
    self.server.delete_buffer(self.handle);
  }
}