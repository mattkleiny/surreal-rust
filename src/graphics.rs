//! A lightweight cross-platform graphics engine.

pub use buffers::*;
pub use colors::*;
pub use images::*;
pub use materials::*;
pub use meshes::*;
pub use palettes::*;
pub use shaders::*;
pub use textures::*;

mod buffers;
mod colors;
mod images;
mod materials;
mod meshes;
mod palettes;
mod shaders;
mod textures;

/// Represents a fallible result in the graphics subsystem.
pub type GraphicsResult<T> = anyhow::Result<T>;

/// An opaque handle to an underlying resource in the [`GraphicsServer`].
///
/// A handle can represent arbitrarily many different resources, and forms
/// the building blocks for any higher level APIs.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GraphicsHandle {
  pub(crate) id: u32,
}

/// The context for graphics operations.
pub type GraphicsContext = super::Context<dyn GraphicsServer>;

impl GraphicsContext {
  pub fn new(value: impl GraphicsServer + 'static) -> Self {
    Self(std::rc::Rc::new(value))
  }
}

/// A server for the underlying graphics subsystem.
///
/// This is a high-level abstraction that makes use of 'opaque' handles to hide away implementation
/// details. The server is intended to be a low-level unsafe implementation abstraction.
pub trait GraphicsServer {
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
  fn create_buffer(&self) -> GraphicsHandle;
  fn read_buffer_data(&self, buffer: GraphicsHandle, kind: BufferKind, offset: usize, length: usize) -> Vec<u8>;
  fn write_buffer_data(&self, buffer: GraphicsHandle, usage: BufferUsage, kind: BufferKind, data: *const u8, length: usize);
  fn delete_buffer(&self, buffer: GraphicsHandle);

  // textures
  fn create_texture(&self, minify_filter: TextureFilter, magnify_filter: TextureFilter, wrap_mode: TextureWrap) -> GraphicsHandle;
  fn write_texture_data(&self, texture: GraphicsHandle, width: usize, height: usize, pixels: *const u8, length: usize, format: TextureFormat, mip_level: usize);
  fn delete_texture(&self, texture: GraphicsHandle);

  // shaders
  fn create_shader(&self) -> GraphicsHandle;
  fn get_shader_uniform_location(&self, shader: GraphicsHandle, name: &str) -> Option<usize>;
  fn set_shader_uniform_u32(&self, shader: GraphicsHandle, location: usize, value: u32);
  fn set_shader_uniform_f32(&self, shader: GraphicsHandle, location: usize, value: f32);
  fn set_shader_uniform_i32(&self, shader: GraphicsHandle, location: usize, value: i32);
  fn set_shader_uniform_uv(&self, shader: GraphicsHandle, location: usize, value: &[u32]);
  fn set_shader_uniform_iv(&self, shader: GraphicsHandle, location: usize, value: &[i32]);
  fn set_shader_uniform_fv(&self, shader: GraphicsHandle, location: usize, value: &[f32]);
  fn link_shaders(&self, shader: GraphicsHandle, shaders: Vec<Shader>) -> GraphicsResult<()>;
  fn delete_shader(&self, shader: GraphicsHandle);

  // meshes
  fn create_mesh(&self, vertices: GraphicsHandle, indices: GraphicsHandle, descriptors: &[VertexDescriptor]) -> GraphicsHandle;
  fn draw_mesh(&self, mesh: GraphicsHandle, topology: PrimitiveTopology, vertex_count: usize, index_count: usize);
  fn delete_mesh(&self, mesh: GraphicsHandle);
}
