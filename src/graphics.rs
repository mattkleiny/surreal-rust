//! A lightweight cross-platform graphics engine.

pub use buffers::*;
pub use colors::*;
pub use fonts::*;
pub use headless::*;
pub use images::*;
pub use materials::*;
pub use meshes::*;
pub use palettes::*;
pub use rendering::*;
pub use shaders::*;
pub use sprites::*;
pub use targets::*;
pub use textures::*;

mod buffers;
mod colors;
mod fonts;
mod headless;
mod images;
mod materials;
mod meshes;
mod palettes;
mod rendering;
mod shaders;
mod sprites;
mod targets;
mod textures;

/// An opaque handle to resource in the graphics subsystem.
pub type GraphicsHandle = u32;

/// The graphics server implementation.
pub type GraphicsServer = std::rc::Rc<Box<dyn GraphicsBackend>>;

/// Represents a type that possesses a `GraphicsHandle`.
pub trait HasGraphicsHandle {
  fn handle(&self) -> GraphicsHandle;
}

/// Represents a server implementation for the underlying graphics subsystem.
///
/// This is a high-level abstraction that makes use of 'opaque' handles to hide away implementation
/// details. The server is intended to be a low-level unsafe implementation abstraction.
pub trait GraphicsBackend {
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
  fn set_texture_options(&self, texture: GraphicsHandle, sampler: &TextureSampler);
  fn write_texture_data(&self, texture: GraphicsHandle, width: usize, height: usize, pixels: *const u8, internal_format: TextureFormat, pixel_format: TextureFormat, mip_level: usize);
  fn delete_texture(&self, texture: GraphicsHandle);

  // shaders
  fn create_shader(&self) -> GraphicsHandle;
  fn link_shaders(&self, shader: GraphicsHandle, shaders: Vec<Shader>) -> crate::Result<()>;
  fn get_shader_uniform_location(&self, shader: GraphicsHandle, name: &str) -> Option<usize>;
  fn set_shader_uniform(&self, shader: GraphicsHandle, location: usize, value: &ShaderUniform);
  fn set_active_shader(&self, shader: GraphicsHandle);
  fn delete_shader(&self, shader: GraphicsHandle);

  // meshes
  fn create_mesh(&self, vertices: GraphicsHandle, indices: GraphicsHandle, descriptors: &[VertexDescriptor]) -> GraphicsHandle;
  fn draw_mesh(&self, mesh: GraphicsHandle, topology: PrimitiveTopology, vertex_count: usize, index_count: usize);
  fn delete_mesh(&self, mesh: GraphicsHandle);

  // render targets
  fn create_render_target(&self, color_attachment: GraphicsHandle, depth_attachment: Option<GraphicsHandle>, stencil_attachment: Option<GraphicsHandle>) -> GraphicsHandle;
  fn set_active_render_target(&self, render_target: GraphicsHandle);
  fn set_default_render_target(&self);
  fn delete_render_target(&self, render_target: GraphicsHandle);
}
