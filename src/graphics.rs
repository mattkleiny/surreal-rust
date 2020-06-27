//! A lightweight and fast cross-platform graphics engine.

pub use canvas::*;
pub use colors::*;
pub use images::*;
pub use rendering::*;
pub use shaders::*;
pub use sprites::*;

use crate::RID;

mod canvas;
mod colors;
mod images;
mod rendering;
mod shaders;
mod sprites;

// TODO: support hot-reloading for textures and shaders?
// TODO: make this strongly typed, instead of using RIDs?

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
  fn create_shader(&mut self, source: &impl ShaderSource) -> Result<RID, GraphicsError>;
  fn delete_shader(&mut self, shader_id: RID) -> Result<(), GraphicsError>;
}

pub trait ShaderSource {
  fn get_spirv_binary(&self) -> &[(ShaderKind, &[u8])];
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PrimitiveTopology {
  Points,
  Lines,
  Triangles,
  Quads,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ShaderKind {
  Vertex,
  Fragment,
}

#[repr(u8)]
#[derive(BitFlags, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextureFlags {
  Clamp = 1 << 0,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GraphicsError {
  InvalidTexture,
  InvalidShaderProgram,
  InvalidFrameBuffer,
}

impl From<GraphicsError> for crate::Error {
  fn from(_: GraphicsError) -> Self {
    crate::Error::Graphics
  }
}
