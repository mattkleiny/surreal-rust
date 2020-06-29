//! A lightweight and fast cross-platform graphics engine using OpenGL.

pub use buffers::*;
pub use canvas::*;
pub use colors::*;
pub use images::*;
pub use meshes::*;
pub use shaders::*;
pub use sprites::*;
pub use textures::*;

use crate::RID;

mod buffers;
mod canvas;
mod colors;
mod images;
mod meshes;
mod shaders;
mod sprites;
mod textures;

// TODO: support hot-reloading for textures and shaders?
// TODO: add a singleton graphics server reference?

pub type GraphicsResult<T> = std::result::Result<T, GraphicsError>;

/// Abstracts over a graphics device or GPU.
///
/// Permits interaction with the underlying graphics API through a higher-level abstraction.
pub trait GraphicsDevice {
  // frame buffers
  fn clear_active_framebuffer(&mut self, color: Color);

  // mesh management
  fn create_buffer(&mut self) -> GraphicsResult<RID>;
  fn upload_buffer_data(&mut self, buffer_id: RID, data: &[u8]) -> GraphicsResult<()>;
  fn delete_buffer(&mut self, buffer_id: RID) -> GraphicsResult<()>;

  // shader management
  fn create_shader(&mut self, source: &impl ShaderSource) -> GraphicsResult<RID>;
  fn delete_shader(&mut self, shader_id: RID) -> GraphicsResult<()>;
}

/// A viewport for scissoring operations on a `GraphicsDevice`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Viewport {
  pub width: usize,
  pub height: usize,
}

impl Viewport {
  pub fn new(width: usize, height: usize) -> Self {
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GraphicsError {
  InvalidBuffer,
  InvalidTexture,
  InvalidShaderProgram,
}

impl From<GraphicsError> for crate::Error {
  fn from(_: GraphicsError) -> Self {
    crate::Error::Graphics
  }
}
