use crate::graphics::{GraphicsContext, GraphicsHandle};

/// Texture wrapping modes modes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum WrapFunction {
  Clamp,
  Mirror,
}

/// Texture minify filter modes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MinifyFilter {
  Nearest,
  Linear,
}

/// Texture magnify filter modes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MagnifyFilter {
  Nearest,
  Linear,
}

/// A GPU texture that can read and write pixel data.
pub struct Texture {
  handle: GraphicsHandle,
  context: GraphicsContext,
}

impl Texture {
  /// Creates a new blank texture on the GPU.
  pub fn new(context: &GraphicsContext) -> Self {
    Self {
      handle: unsafe { context.create_texture() },
      context: context.clone(),
    }
  }
}

impl Drop for Texture {
  /// Deletes the texture from the GPU.
  fn drop(&mut self) {
    unsafe {
      self.context.delete_texture(self.handle);
    }
  }
}