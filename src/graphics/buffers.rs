use std::marker::PhantomData;

use super::{GraphicsContext, GraphicsHandle};

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

/// A GPU buffer implementation that can upload data of type `T`.
pub struct Buffer<T> {
  handle: GraphicsHandle,
  context: GraphicsContext,
  kind: BufferKind,
  usage: BufferUsage,
  _type: PhantomData<T>,
}

/// Contains rendering attributes about a vertex.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexAttribute {
  pub name: String,
  pub binding: String,
  pub offset: usize,
  pub stride: usize,
}

impl<T> Buffer<T> {
  /// Constructs a new empty buffer on the GPU.
  pub fn new(context: &GraphicsContext, kind: BufferKind, usage: BufferUsage) -> Self {
    Self {
      handle: unsafe { context.create_buffer() },
      context: context.clone(),
      kind,
      usage,
      _type: PhantomData
    }
  }

  /// Reads data from the buffer.
  pub unsafe fn read_data(&self) -> Vec<T> {
    todo!()
  }

  /// Uploads the given data to the buffer.
  pub unsafe fn write_data(&mut self, data: &[T]) {
    todo!()
  }
}

impl<T> Drop for Buffer<T> {
  /// Deletes the buffer from the GPU.
  fn drop(&mut self) {
    unsafe {
      self.context.delete_buffer(self.handle)
    }
  }
}
