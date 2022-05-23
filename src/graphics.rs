//! A lightweight cross-platform graphics engine.

use std::ops::Deref;
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

/// An opaque handle to an underlying resource in the [`GraphicsServer`].
///
/// A handle can represent arbitrarily many different resources, and forms
/// the building blocks for any higher level APIs.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GraphicsHandle {
  pub(crate) id: u32,
}

/// The graphics server implementation.
///
/// Internally we manage a singleton server implementation backed by a single trait.
#[derive(Clone)]
pub struct GraphicsServer {
  server: Rc<dyn GraphicsServerImpl>,
}

impl GraphicsServer {
  /// Creates a new graphics server.
  pub fn new(server: impl GraphicsServerImpl + 'static) -> Self {
    Self {
      server: Rc::new(server),
    }
  }
}

impl Deref for GraphicsServer {
  type Target = dyn GraphicsServerImpl;

  fn deref(&self) -> &Self::Target {
    self.server.as_ref()
  }
}

/// A server for the underlying graphics subsystem.
///
/// This is a high-level abstraction that makes use of 'opaque' handles to hide away implementation
/// details. The server is intended to be a low-level unsafe implementation abstraction.
pub trait GraphicsServerImpl {
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
  fn create_texture(&self, sampler: &TextureSampler) -> GraphicsHandle;
  fn write_texture_data(&self, texture: GraphicsHandle, width: usize, height: usize, pixels: *const u8, format: TextureFormat, mip_level: usize);
  fn delete_texture(&self, texture: GraphicsHandle);

  // shaders
  fn create_shader(&self) -> GraphicsHandle;
  fn link_shaders(&self, shader: GraphicsHandle, shaders: Vec<Shader>) -> GraphicsResult<()>;
  fn get_shader_uniform_location(&self, shader: GraphicsHandle, name: &str) -> Option<usize>;
  fn set_shader_uniform(&self, shader: GraphicsHandle, location: usize, value: &ShaderUniform);
  fn set_active_shader(&self, shader: GraphicsHandle);
  fn delete_shader(&self, shader: GraphicsHandle);

  // meshes
  fn create_mesh(&self, vertices: GraphicsHandle, indices: GraphicsHandle, descriptors: &[VertexDescriptor]) -> GraphicsHandle;
  fn draw_mesh(&self, mesh: GraphicsHandle, topology: PrimitiveTopology, vertex_count: usize, index_count: usize);
  fn delete_mesh(&self, mesh: GraphicsHandle);
}
