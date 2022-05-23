use std::marker::PhantomData;

use crate::graphics::{GraphicsServer, GraphicsHandle};

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

/// A buffer implementation that can upload data of type [`T`] to the GPU.
pub struct GraphicsBuffer<T> {
  server: GraphicsServer,
  pub handle: GraphicsHandle,
  kind: BufferKind,
  usage: BufferUsage,
  length: usize,
  _type: PhantomData<T>,
}

impl<T> GraphicsBuffer<T> {
  /// Constructs a new empty buffer on the GPU.
  pub fn new(server: &GraphicsServer, kind: BufferKind, usage: BufferUsage) -> Self {
    Self {
      server: server.clone(),
      handle: server.create_buffer(),
      kind,
      usage,
      length: 0,
      _type: PhantomData,
    }
  }

  /// The number of elements in the buffer.
  pub fn len(&self) -> usize {
    self.length
  }

  /// Reads data from the buffer.
  pub fn read_data(&self, _offset: usize, _length: usize) -> Vec<T> where T: Clone {
    todo!()
  }

  /// Uploads the given data to the buffer.
  pub fn write_data(&mut self, data: &[T]) {
    self.length = data.len();
    self.server.write_buffer_data(
      self.handle,
      self.usage,
      self.kind,
      data.as_ptr() as *const u8,
      data.len() * std::mem::size_of::<T>(),
    );
  }
}

impl<T> Drop for GraphicsBuffer<T> {
  /// Deletes the buffer from the GPU.
  fn drop(&mut self) {
    self.server.delete_buffer(self.handle)
  }
}
