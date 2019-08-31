//! GPU buffer management.


#[derive(Clone, Copy, Debug)]
pub enum BufferData<'a, T> {
  Uninitialized(usize),
  Memory(&'a [T]),
}

#[derive(Clone, Copy, Debug)]
pub enum BufferTarget {
  Vertex,
  Index,
}

#[derive(Clone, Copy, Debug)]
pub enum BufferUploadMode {
  Static,
  Dynamic,
}
