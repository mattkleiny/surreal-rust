//! A lightweight cross-platform graphics engine.

use std::fmt::Debug;
use std::rc::Rc;

pub use buffers::*;
pub use colors::*;
pub use images::*;
pub use materials::*;
pub use meshes::*;
pub use palettes::*;
pub use shaders::*;
pub use sprites::*;
pub use textures::*;

mod buffers;
mod colors;
mod images;
mod materials;
mod meshes;
mod palettes;
mod shaders;
mod sprites;
mod textures;

/// Represents a fallible result in the graphics subsystem.
pub type GraphicsResult<T> = anyhow::Result<T>;

/// The graphics server implementation.
pub type GraphicsServer<G> = Rc<G>;

/// Represents a server implementation for the underlying graphics subsystem.
///
/// This is a high-level abstraction that makes use of 'opaque' handles to hide away implementation
/// details. The server is intended to be a low-level unsafe implementation abstraction.
pub trait GraphicsImpl {
  type Handle: Copy + Debug;

  // frame operations
  fn begin_frame(&self);
  fn end_frame(&self);

  // intrinsics
  fn set_viewport_size(&self, viewport: (usize, usize));
  fn set_blend_state(&self, blend_state: BlendState);
  fn clear_color_buffer(&self, color: Color);
  fn clear_depth_buffer(&self);
  fn flush_commands(&self);

  // buffers
  fn create_buffer(&self) -> Self::Handle;
  fn read_buffer_data(&self, buffer: Self::Handle, kind: BufferKind, offset: usize, length: usize) -> Vec<u8>;
  fn write_buffer_data(&self, buffer: Self::Handle, usage: BufferUsage, kind: BufferKind, data: *const u8, length: usize);
  fn delete_buffer(&self, buffer: Self::Handle);

  // textures
  fn create_texture(&self, sampler: &TextureSampler) -> Self::Handle;
  fn write_texture_data(&self, texture: Self::Handle, width: usize, height: usize, pixels: *const u8, format: TextureFormat, mip_level: usize);
  fn delete_texture(&self, texture: Self::Handle);

  // shaders
  fn create_shader(&self) -> Self::Handle;
  fn link_shaders(&self, shader: Self::Handle, shaders: Vec<Shader>) -> GraphicsResult<()>;
  fn get_shader_uniform_location(&self, shader: Self::Handle, name: &str) -> Option<usize>;
  fn set_shader_uniform(&self, shader: Self::Handle, location: usize, value: &ShaderUniform<Self>);
  fn set_active_shader(&self, shader: Self::Handle);
  fn delete_shader(&self, shader: Self::Handle);

  // meshes
  fn create_mesh(&self, vertices: Self::Handle, indices: Self::Handle, descriptors: &[VertexDescriptor]) -> Self::Handle;
  fn draw_mesh(&self, mesh: Self::Handle, topology: PrimitiveTopology, vertex_count: usize, index_count: usize);
  fn delete_mesh(&self, mesh: Self::Handle);
}
