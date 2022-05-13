use super::{GraphicsHandle, GraphicsServer};

/// The different kinds of buffer we support.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BufferKind {
  Element,
  Index,
  Uniform,
}

/// The usage pattern of the buffer.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BufferUsage {
  Static,
  Dynamic,
}

/// A buffer implementation based on OpenGL.
pub struct Buffer<G: 'static + GraphicsServer> {
  server: &'static G,
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
impl<G: GraphicsServer> Buffer<G> {
  pub fn new(server: &'static G, kind: BufferKind, usage: BufferUsage) -> Self {
    Buffer {
      server,
      handle: unsafe { server.create_buffer() },
      kind,
      usage,
    }
  }

  pub fn kind(&self) -> BufferKind { self.kind }
  pub fn usage(&self) -> BufferUsage { self.usage }

  /// Reads data from the buffer.
  pub unsafe fn read_data<T>(&self) -> Vec<T> {
    self.server.read_buffer_data(self.handle)
  }

  /// Uploads the given data to the buffer.
  pub unsafe fn write_data<T>(&mut self, data: &[T]) {
    self.server.write_buffer_data(self.handle, data);
  }
}

impl<G: GraphicsServer> Drop for Buffer<G> {
  fn drop(&mut self) {
    unsafe {
      self.server.delete_buffer(self.handle);
    }
  }
}
