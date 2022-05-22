use std::marker::PhantomData;
use std::slice::from_raw_parts;

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

/// A buffer implementation that can upload data of type `T` to the GPU.
pub struct GraphicsBuffer<T> {
  handle: GraphicsHandle,
  context: GraphicsContext,
  kind: BufferKind,
  usage: BufferUsage,
  _type: PhantomData<T>,
}

impl<T> GraphicsBuffer<T> {
  /// Constructs a new empty buffer on the GPU.
  pub fn new(context: &GraphicsContext, kind: BufferKind, usage: BufferUsage) -> Self {
    Self {
      handle: unsafe { context.create_buffer() },
      context: context.clone(),
      kind,
      usage,
      _type: PhantomData,
    }
  }

  /// Reads data from the buffer.
  pub fn read_data(&self, offset: usize, length: usize) -> Vec<T> where T: Clone {
    unsafe {
      let buffer = self.context.read_buffer_data(self.handle, self.kind, offset, length);
      let slice = from_raw_parts(buffer.as_ptr() as *const T, length);

      Vec::from(slice)
    }
  }

  /// Uploads the given data to the buffer.
  pub fn write_data(&mut self, data: &[T]) {
    unsafe {
      let size = data.len() * std::mem::size_of::<T>();
      let bytes = from_raw_parts(data.as_ptr() as *const u8, size);

      self.context.write_buffer_data(self.handle, self.usage, self.kind, bytes);
    }
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
