use std::marker::PhantomData;

use crate::utilities::Size;

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
pub struct GraphicsBuffer<T> {
  handle: GraphicsHandle,
  context: GraphicsContext,
  kind: BufferKind,
  usage: BufferUsage,
  size: Size,
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

impl<T> GraphicsBuffer<T> {
  /// Constructs a new empty buffer on the GPU.
  pub fn new(context: &GraphicsContext, kind: BufferKind, usage: BufferUsage) -> Self {
    Self {
      handle: unsafe { context.create_buffer() },
      context: context.clone(),
      kind,
      usage,
      size: Size::from_bytes(0),
      _type: PhantomData,
    }
  }

  /// Reads data from the buffer.
  pub fn read_data(&self) -> Vec<T> {
    todo!()
  }

  /// Uploads the given data to the buffer.
  pub fn write_data(&mut self, data: &[T]) {
    self.size = Size::from_bytes(data.len() * std::mem::size_of::<T>());
  }
}

impl<T> Drop for GraphicsBuffer<T> {
  /// Deletes the buffer from the GPU.
  fn drop(&mut self) {
    unsafe {
      self.context.delete_buffer(self.handle)
    }
  }
}
