use std::marker::PhantomData;

/// A buffer implementation based on OpenGL.
pub struct Buffer<T> {
  kind: BufferKind,
  usage: BufferUsage,
  phantom: PhantomData<T>,
}

/// Represents abstractly some buffer of data on the GPU.
impl<T> Buffer<T> {
  pub fn new(kind: BufferKind, usage: BufferUsage) -> Self {
    Self { kind, usage, phantom: PhantomData }
  }

  fn kind(&self) -> BufferKind { self.kind }
  fn usage(&self) -> BufferUsage { self.usage }

  fn attributes(&self) -> &[VertexAttribute]
    where T: Vertex {
    T::attributes()
  }

  /// Uploads the given data to the buffer.
  fn upload(&mut self, data: &[T]) -> super::GraphicsResult<()> {
    unimplemented!()
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
