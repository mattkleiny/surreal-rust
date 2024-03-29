//! Graphics buffer management and abstractions.
//!
//! Buffers typically contain vertex or index data used in mesh rendering,
//! however they can also be used as an in intermediate store for compute
//! shaders.

use std::{cell::RefCell, rc::Rc};

use super::*;

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

/// A buffer implementation that can upload data of type [`T`] to the GPU.
#[derive(Clone)]
pub struct Buffer<T> {
  state: Rc<RefCell<BufferState>>,
  _type: std::marker::PhantomData<T>,
}

/// The internal state for a buffer.
struct BufferState {
  id: BufferId,
  graphics: GraphicsEngine,
  kind: BufferKind,
  usage: BufferUsage,
  length: usize,
}

impl<T> Buffer<T> {
  /// Constructs a new empty buffer on the GPU.
  pub fn new(graphics: &GraphicsEngine, kind: BufferKind, usage: BufferUsage) -> Result<Self, BufferError> {
    Ok(Self {
      state: Rc::new(RefCell::new(BufferState {
        id: graphics.buffer_create()?,
        graphics: graphics.clone(),
        kind,
        usage,
        length: 0,
      })),
      _type: std::marker::PhantomData,
    })
  }

  /// Returns the ID of the underlying buffer.
  pub fn id(&self) -> BufferId {
    self.state.borrow().id
  }

  /// Is the buffer empty?
  pub fn is_empty(&self) -> bool {
    self.state.borrow().length == 0
  }

  /// The number of elements in the buffer.
  pub fn len(&self) -> usize {
    self.state.borrow().length
  }

  /// Reads all data from the buffer.
  #[allow(clippy::uninit_vec)] // immediately fill the buffer from the gpu
  pub fn read_data(&self) -> Vec<T> {
    let state = self.state.borrow();
    let length = state.length;

    let mut buffer = Vec::with_capacity(length);

    unsafe {
      buffer.set_len(length);

      state
        .graphics
        .buffer_read_data(
          state.id,
          0, // offset
          length * std::mem::size_of::<T>(),
          buffer.as_mut_ptr() as *mut u8,
        )
        .expect("Failed to read buffer data");
    }

    buffer
  }

  /// Uploads the given data to the buffer.
  pub fn write_data(&mut self, data: &[T]) {
    let mut state = self.state.borrow_mut();

    state.length = data.len();

    state
      .graphics
      .buffer_write_data(
        state.id,
        state.usage,
        state.kind,
        std::mem::size_of_val(data),
        data.as_ptr() as *const u8,
      )
      .expect("Failed to write buffer data");
  }
}

impl Drop for BufferState {
  fn drop(&mut self) {
    self.graphics.buffer_delete(self.id).expect("Failed to delete buffer")
  }
}
