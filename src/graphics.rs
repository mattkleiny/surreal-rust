//! A lightweight and fast cross-platform graphics engine using OpenGL.

pub use buffers::*;
pub use colors::*;
pub use commands::*;
pub use images::*;
pub use meshes::*;
pub use shaders::*;
pub use sprites::*;
pub use textures::*;

mod buffers;
mod colors;
mod commands;
mod images;
mod meshes;
mod shaders;
mod sprites;
mod textures;

/// Represents an error in the graphics subsystem.
pub type GraphicsResult<T> = std::result::Result<T, Error>;

/// Abstracts over a graphics device or GPU.
///
/// Permits interaction with the underlying graphics API through a higher-level abstraction.
pub trait GraphicsDevice {
  /// Clears the active frame buffer on the device.
  fn clear_frame_buffer(&mut self, color: Color);

  /// Draws a mesh on the device.
  fn draw_mesh(&mut self, topology: PrimitiveTopology, vertex_buffer: &Buffer, index_buffer: &Buffer, vertex_count: usize);
}

/// A viewport for scissoring operations on a `GraphicsDevice`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Viewport {
  pub width: usize,
  pub height: usize,
}

/// Represents the different topologies supported for a mesh.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PrimitiveTopology {
  Points,
  Lines,
  Triangles,
  Quads,
}

/// Represents the different blending modes for the graphics pipeline.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BlendingMode {
  None,
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
