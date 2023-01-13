//! A lightweight cross-platform graphics engine.

pub use buffers::*;
pub use colors::*;
pub use fonts::*;
pub use geometry::*;
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

use crate::maths::Rectangle;

mod buffers;
mod colors;
mod fonts;
mod geometry;
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

/// An opaque handle to a resource in the graphics subsystem.
pub type GraphicsHandle = u32;

/// A wrapper for the core [`GraphicsBackend`] implementation.
#[derive(Clone)]
pub struct GraphicsServer {
  backend: std::sync::Arc<Box<dyn GraphicsBackend>>,
}

impl GraphicsServer {
  /// Creates a new [`GraphicsServer`] for the given [`GraphicsBackend`].
  pub fn new(backend: impl GraphicsBackend + 'static) -> Self {
    Self {
      backend: std::sync::Arc::new(Box::new(backend)),
    }
  }
}

unsafe impl Send for GraphicsServer {}
unsafe impl Sync for GraphicsServer {}

impl std::ops::Deref for GraphicsServer {
  type Target = Box<dyn GraphicsBackend>;

  fn deref(&self) -> &Self::Target {
    self.backend.as_ref()
  }
}

/// The nominal max number of texture units that might be be bound in the GPU.
///
/// This is a hint for sizing arrays and other data structures.
const MAX_TEXTURE_UNITS: usize = 32;

/// Represents a resource that possesses a `GraphicsHandle`.
pub trait GraphicsResource {
  fn handle(&self) -> GraphicsHandle;
}

/// An abstraction on top of the underlying graphics API.
///
/// This is a mid-level abstraction that makes use of 'opaque' resource IDs to
/// hide away implementation details and lifetimes. The backend forms the
/// foundation of higher-level abstractions that make it simpler to build
/// graphics programs.
pub trait GraphicsBackend {
  // frame operations
  fn begin_frame(&self);
  fn end_frame(&self);

  // intrinsics
  fn viewport_size(&self) -> (usize, usize);
  fn set_viewport_size(&self, size: winit::dpi::PhysicalSize<u32>);
  fn set_blend_state(&self, blend_state: BlendState);
  fn set_culling_mode(&self, culling_mode: CullingMode);
  fn set_scissor_mode(&self, scissor_mode: ScissorMode);

  // buffers
  fn buffer_create(&self) -> GraphicsHandle;
  fn buffer_read_data(&self, buffer: GraphicsHandle, offset: usize, length: usize, pointer: *mut u8);
  fn buffer_write_data(&self, buffer: GraphicsHandle, usage: BufferUsage, kind: BufferKind, length: usize, pointer: *const u8);
  fn buffer_delete(&self, buffer: GraphicsHandle);

  // textures
  fn texture_create(&self, sampler: &TextureSampler) -> GraphicsHandle;
  fn texture_set_options(&self, texture: GraphicsHandle, sampler: &TextureSampler);
  fn texture_initialize(&self, texture: GraphicsHandle, width: u32, height: u32, format: TextureFormat);
  fn texture_read_data(&self, texture: GraphicsHandle, length: usize, pixel_format: TextureFormat, pixels: *mut u8, mip_level: usize);
  fn texture_write_data(
    &self,
    texture: GraphicsHandle,
    width: u32,
    height: u32,
    pixels: *const u8,
    internal_format: TextureFormat,
    pixel_format: TextureFormat,
    mip_level: usize,
  );
  fn texture_write_sub_data(
    &self,
    texture: GraphicsHandle,
    region: &Rectangle,
    pixels: *const u8,
    pixel_format: TextureFormat,
    mip_level: usize,
  );
  fn texture_delete(&self, texture: GraphicsHandle);

  // shaders
  fn shader_create(&self) -> GraphicsHandle;
  fn shader_link(&self, shader: GraphicsHandle, kernels: &[ShaderKernel]) -> crate::Result<()>;
  fn shader_uniform_location(&self, shader: GraphicsHandle, name: &str) -> Option<usize>;
  fn shader_set_uniform(&self, shader: GraphicsHandle, location: usize, value: &ShaderUniform);
  fn shader_activate(&self, shader: GraphicsHandle);
  fn shader_delete(&self, shader: GraphicsHandle);

  // meshes
  fn mesh_create(&self, vertices: GraphicsHandle, indices: GraphicsHandle, descriptors: &[VertexDescriptor]) -> GraphicsHandle;
  fn mesh_draw(&self, mesh: GraphicsHandle, topology: PrimitiveTopology, vertex_count: usize, index_count: usize);
  fn mesh_delete(&self, mesh: GraphicsHandle);

  // render targets
  fn target_create(
    &self,
    color_attachment: GraphicsHandle,
    depth_attachment: Option<GraphicsHandle>,
    stencil_attachment: Option<GraphicsHandle>,
  ) -> GraphicsHandle;
  fn target_activate(&self, render_target: GraphicsHandle);
  fn target_set_default(&self);
  fn target_blit(&self, from: GraphicsHandle, to: GraphicsHandle, source_rect: &Rectangle, dest_rect: &Rectangle, filter: TextureFilter);
  fn target_blit_to_display(&self, handle: GraphicsHandle, source_rect: &Rectangle, dest_rect: &Rectangle, filter: TextureFilter);
  fn target_delete(&self, render_target: GraphicsHandle);
}
