//! A lightweight and fast cross-platform graphics engine.

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

pub trait GraphicsServer {
  type Buffer;
  type Texture;
  type Shader;

  // frame buffers
  fn create_framebuffer(&mut self) -> Result<RID, GraphicsError>;
  fn delete_framebuffer(&mut self, buffer_id: RID) -> Result<RID, GraphicsError>;
  fn set_active_framebuffer(&mut self, buffer_id: RID) -> Result<(), GraphicsError>;
  fn clear_active_framebuffer(&mut self, color: Color);

  // mesh management
  fn create_vertex_buffer(&mut self) -> Result<RID, GraphicsError>;
  fn create_index_buffer(&mut self) -> Result<RID, GraphicsError>;
  fn draw_mesh(&mut self, count: usize, topology: PrimitiveTopology) -> Result<(), GraphicsError>;
  fn draw_mesh_indexed(&mut self, count: usize, topology: PrimitiveTopology) -> Result<(), GraphicsError>;

  // texture management
  fn create_texture(&mut self) -> Result<RID, GraphicsError>;
  fn create_texture_from_image(&mut self, image: &Image) -> Result<RID, GraphicsError>;
  fn upload_texture_data(&mut self, texture_id: RID, image: &Image) -> Result<(), GraphicsError>;
  fn delete_texture(&mut self, texture_id: RID) -> Result<(), GraphicsError>;

  // shader management
  fn create_shader(&mut self) -> Result<RID, GraphicsError>;
  fn delete_shader(&mut self, shader_id: RID) -> Result<(), GraphicsError>;
}

/// A viewport fo scissoring operations.
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

/// Represents any of the errors that might be exhibited by the graphics components.
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
