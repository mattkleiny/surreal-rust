//! A lightweight and fast cross-platform graphics engine using OpenGL.

pub use buffers::*;
pub use colors::*;
pub use images::*;
pub use meshes::*;
pub use shaders::*;
pub use sprites::*;
pub use textures::*;

mod buffers;
mod colors;
mod images;
mod meshes;
mod shaders;
mod sprites;
mod textures;

// TODO: support hot-reloading for textures and shaders?

pub type GraphicsResult<T> = std::result::Result<T, Error>;

/// Abstracts over a graphics device or GPU.
///
/// Permits interaction with the underlying graphics API through a higher-level abstraction.
pub trait GraphicsDevice {
  // frame buffers
  fn clear_active_frame_buffer(&mut self, color: Color);
  fn set_viewport(&mut self, viewport: Viewport);
}

/// A viewport for scissoring operations on a `GraphicsDevice`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Viewport {
  pub width: usize,
  pub height: usize,
}

impl Viewport {
  #[inline]
  pub const fn new(width: usize, height: usize) -> Self {
    Self { width, height }
  }
}

/// A represents the topology of a mesh for draw calls.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PrimitiveTopology {
  Points,
  Lines,
  Triangles,
  Quads,
}

/// Represents an error with graphics.
#[derive(Debug)]
pub enum Error {
  InvalidBuffer,
  InvalidTexture,
  InvalidShaderProgram,
}

impl From<Error> for crate::Error {
  fn from(error: Error) -> Self {
    Self::Graphics(error)
  }
}
