//! A lightweight cross-platform graphics engine.

pub use buffers::*;
pub use colors::*;
pub use fonts::*;
pub use geometry::*;
pub use headless::*;
pub use images::*;
pub use materials::*;
pub use meshes::*;
pub use opengl::*;
pub use palettes::*;
pub use rendering::*;
pub use shaders::*;
pub use sprites::*;
pub use targets::*;
pub use textures::*;

mod buffers;
mod colors;
mod fonts;
mod geometry;
mod headless;
mod images;
mod materials;
mod meshes;
mod opengl;
mod palettes;
mod rendering;
mod shaders;
mod sprites;
mod targets;
mod textures;

/// An opaque handle to a resource in the graphics subsystem.
pub type GraphicsHandle = u32;

/// A pointer to the core [`GraphicsBackend`] implementation.
/// 
/// This pointer is safe to pass around the application.
pub type GraphicsServer = std::rc::Rc<Box<dyn GraphicsBackend>>;

/// Represents a resource that possesses a `GraphicsHandle`.
pub trait GraphicsResource {
  fn handle(&self) -> GraphicsHandle;
}

/// Indicates the kinds of barriers that can be synchronized in the GPU.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GraphicsBarrier {
  ImageAccess,
}

/// Represents a backend implementation for the underlying graphics API.
///
/// This is a high-level abstraction that makes use of 'opaque' handles to hide away implementation
/// details. The server is intended to be a low-level implementation abstraction.
/// 
/// Theoeretically different backends could be supported; though it's unlikely to be anything other
/// than OpenGL. We do provide a headless backend to facilitate testing and related, however.
pub trait GraphicsBackend {
  // frame operations
  fn begin_frame(&self);
  fn end_frame(&self);

  // intrinsics
  fn get_viewport_size(&self) -> (usize, usize);
  fn set_viewport_size(&self, viewport: (usize, usize));
  fn set_blend_state(&self, blend_state: BlendState);
  fn set_culling_mode(&self, culling_mode: CullingMode);
  fn set_scissor_mode(&self, scissor_mode: ScissorMode);
  fn clear_color_buffer(&self, color: Color);
  fn clear_depth_buffer(&self);

  // buffers
  fn create_buffer(&self) -> GraphicsHandle;
  fn read_buffer_data(
    &self,
    buffer: GraphicsHandle,
    offset: usize,
    length: usize,
    pointer: *mut u8,
  );
  fn write_buffer_data(
    &self,
    buffer: GraphicsHandle,
    usage: BufferUsage,
    kind: BufferKind,
    length: usize,
    pointer: *const u8,
  );
  fn delete_buffer(&self, buffer: GraphicsHandle);

  // textures
  fn create_texture(&self, sampler: &TextureSampler) -> GraphicsHandle;
  fn set_texture_options(&self, texture: GraphicsHandle, sampler: &TextureSampler);
  fn read_texture_data(
    &self,
    texture: GraphicsHandle,
    length: usize,
    pixel_format: TextureFormat,
    pixels: *mut u8,
    mip_level: usize,
  );
  fn write_texture_data(
    &self,
    texture: GraphicsHandle,
    width: usize,
    height: usize,
    pixels: *const u8,
    internal_format: TextureFormat,
    pixel_format: TextureFormat,
    mip_level: usize,
  );
  fn write_texture_sub_data(
    &self,
    texture: GraphicsHandle,
    region: &crate::maths::Rectangle<usize>,
    pixels: *const u8,
    pixel_format: TextureFormat,
    mip_level: usize,
  );
  fn delete_texture(&self, texture: GraphicsHandle);

  // shaders
  fn create_shader(&self) -> GraphicsHandle;
  fn link_shaders(&self, shader: GraphicsHandle, shaders: Vec<Shader>) -> crate::Result<()>;
  fn get_shader_uniform_location(&self, shader: GraphicsHandle, name: &str) -> Option<usize>;
  fn set_shader_uniform(&self, shader: GraphicsHandle, location: usize, value: &ShaderUniform);
  fn set_active_shader(&self, shader: GraphicsHandle);
  fn delete_shader(&self, shader: GraphicsHandle);

  // compute
  fn dispatch_compute(&self, shader: GraphicsHandle, x: u32, y: u32, z: u32);
  fn wait_compute_barrier(&self, barrier: GraphicsBarrier);

  // meshes
  fn create_mesh(
    &self,
    vertices: GraphicsHandle,
    indices: GraphicsHandle,
    descriptors: &[VertexDescriptor],
  ) -> GraphicsHandle;
  fn draw_mesh(
    &self,
    mesh: GraphicsHandle,
    topology: PrimitiveTopology,
    vertex_count: usize,
    index_count: usize,
  );
  fn delete_mesh(&self, mesh: GraphicsHandle);

  // render targets
  fn create_render_target(
    &self,
    color_attachment: GraphicsHandle,
    depth_attachment: Option<GraphicsHandle>,
    stencil_attachment: Option<GraphicsHandle>,
  ) -> GraphicsHandle;
  fn set_active_render_target(&self, render_target: GraphicsHandle);
  fn set_default_render_target(&self);
  fn delete_render_target(&self, render_target: GraphicsHandle);
}
