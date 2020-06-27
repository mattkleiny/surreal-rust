use std::marker::PhantomData;

/// Represents an arbitrary buffer of `T` on the GPU.
#[derive(Debug)]
pub struct Buffer<T> {
  _data: PhantomData<T>,
}

impl<T> Buffer<T> {
  /// Uploads the given data to the buffer, overwriting old data.
  pub fn upload(&mut self, data: &[T]) {
    unimplemented!()
  }

  /// Gets the `VertexAttribute`s for this buffer.
  #[inline(always)]
  fn attributes(&self) -> &[VertexAttribute]
    where T: Vertex {
    T::attributes()
  }
}

/// A vertex type that possesses `VertexAttribute`s.
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

/// Represents a buffer for frame-by-frame rendering.
#[derive(Debug)]
pub struct FrameBuffer {}
