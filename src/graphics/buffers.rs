use std::marker::PhantomData;
use std::os::raw::c_void;

/// A managed ID for OpenGL buffers.
struct BufferHandle(u32);

impl BufferHandle {
  pub fn new() -> Self {
    let mut id = 0;
    unsafe {
      gl::GenBuffers(1, &mut id)
    }
    Self(id)
  }
}

impl Drop for BufferHandle {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteBuffers(1, &self.0);
    }
  }
}

/// A buffer implementation based on OpenGL.
pub struct Buffer<T> {
  handle: BufferHandle,
  kind: BufferKind,
  usage: BufferUsage,
  phantom: PhantomData<T>,
}

/// Represents abstractly some buffer of data on the GPU.
impl<T> Buffer<T> {
  pub fn new(kind: BufferKind, usage: BufferUsage) -> Self {
    Self {
      handle: BufferHandle::new(),
      kind,
      usage,
      phantom: PhantomData,
    }
  }

  fn kind(&self) -> BufferKind { self.kind }
  fn usage(&self) -> BufferUsage { self.usage }

  fn attributes(&self) -> &[VertexAttribute] where T: Vertex {
    T::attributes()
  }

  /// Uploads the given data to the buffer.
  fn upload(&mut self, data: &[T]) {
    unsafe {
      let kind = match self.kind {
        BufferKind::Element => gl::ARRAY_BUFFER,
        BufferKind::Index => gl::ELEMENT_ARRAY_BUFFER,
      };
      let usage = match self.usage {
        BufferUsage::Static => gl::STATIC_DRAW,
        BufferUsage::Dynamic => gl::DYNAMIC_DRAW,
      };
      gl::BindBuffer(kind, self.handle.0);
      gl::BufferData(kind, data.len() as isize, data.as_ptr() as *const c_void, usage)
    }
  }
}

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

/// Represents a vertex type that possesses `VertexAttribute`s.
// TODO: implement a #[derive] macro for vertex attributes?
pub trait Vertex {
  fn attributes() -> &'static [VertexAttribute];
}

/// Contains rendering attributes about a vertex.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexAttribute {
  pub name: String,
  pub binding: String,
  pub offset: usize,
  pub stride: usize,
}
