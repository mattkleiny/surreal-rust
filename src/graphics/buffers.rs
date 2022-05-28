use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use super::*;

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
#[derive(Clone)]
pub struct Buffer<T> {
  state: Rc<RefCell<BufferState>>,
  _type: PhantomData<T>,
}

/// The internal state for a buffer.
struct BufferState {
  server: GraphicsServer,
  handle: GraphicsHandle,
  kind: BufferKind,
  usage: BufferUsage,
  length: usize,
}

impl<T> Buffer<T> {
  /// Constructs a new empty buffer on the GPU.
  pub fn new(server: &GraphicsServer, kind: BufferKind, usage: BufferUsage) -> Self {
    Self {
      state: Rc::new(RefCell::new(BufferState {
        server: server.clone(),
        handle: server.create_buffer(),
        kind,
        usage,
        length: 0,
      })),
      _type: PhantomData,
    }
  }

  /// The number of elements in the buffer.
  pub fn len(&self) -> usize {
    self.state.borrow().length
  }

  /// Reads all data from the buffer.
  pub fn read_data(&self) -> Vec<T> {
    let state = self.state.borrow();
    let length = state.length;

    let mut buffer = Vec::with_capacity(length);

    unsafe {
      buffer.set_len(length);

      state.server.read_buffer_data(
        state.handle,
        0, // offset
        length * std::mem::size_of::<T>(),
        buffer.as_mut_ptr() as *mut u8,
      );
    }

    buffer
  }

  /// Uploads the given data to the buffer.
  #[macros::profile_function]
  pub fn write_data(&mut self, data: &[T]) {
    let mut state = self.state.borrow_mut();

    state.length = data.len();
    state.server.write_buffer_data(
      state.handle,
      state.usage,
      state.kind,
      data.len() * std::mem::size_of::<T>(),
      data.as_ptr() as *const u8,
    );
  }
}

impl<T> GraphicsResource for Buffer<T> {
  /// Retrieves the handle for the [`Buffer`].
  fn handle(&self) -> GraphicsHandle {
    self.state.borrow().handle
  }
}

impl Drop for BufferState {
  /// Deletes the [`Buffer`] from the GPU.
  fn drop(&mut self) {
    self.server.delete_buffer(self.handle)
  }
}
