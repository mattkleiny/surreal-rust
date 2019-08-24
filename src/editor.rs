//! Editor support for Surreal.

pub trait Editor {
  type Host: EditorHost;
}

pub trait EditorHost {
  fn close(&mut self);
}