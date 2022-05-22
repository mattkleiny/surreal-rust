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

/// An opaque handle to an underlying resource in the `GraphicsServer`.
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
pub unsafe trait GraphicsServer {
  // frame operations
  unsafe fn begin_frame(&self);
  unsafe fn end_frame(&self);

  // intrinsics
  unsafe fn set_viewport_size(&self, viewport: (usize, usize));
  unsafe fn set_blend_state(&self, blend_state: BlendState);
  unsafe fn clear_color_buffer(&self, color: Color);
  unsafe fn clear_depth_buffer(&self);
  unsafe fn flush_commands(&self);

  // buffers
  unsafe fn create_buffer(&self) -> GraphicsHandle;
  unsafe fn read_buffer_data(&self, buffer: GraphicsHandle, kind: BufferKind, offset: usize, length: usize) -> Vec<u8>;
  unsafe fn write_buffer_data(&self, buffer: GraphicsHandle, usage: BufferUsage, kind: BufferKind, data: &[u8]);
  unsafe fn delete_buffer(&self, buffer: GraphicsHandle);

  // textures
  unsafe fn create_texture(&self, filter_mode: TextureFilter, wrap_mode: TextureWrap) -> GraphicsHandle;
  unsafe fn write_texture_data(&self, texture: GraphicsHandle, width: usize, height: usize, pixels: &[u8], format: TextureFormat, mip_level: usize);
  unsafe fn delete_texture(&self, texture: GraphicsHandle);

  // shaders
  unsafe fn create_shader(&self) -> GraphicsHandle;
  unsafe fn link_shaders(&self, shader: GraphicsHandle, shaders: Vec<Shader>) -> GraphicsResult<()>;
  unsafe fn delete_shader(&self, shader: GraphicsHandle);

  // meshes
  unsafe fn create_mesh(&self, descriptors: &[VertexDescriptor]) -> GraphicsHandle;
  unsafe fn delete_mesh(&self, mesh: GraphicsHandle);
}
