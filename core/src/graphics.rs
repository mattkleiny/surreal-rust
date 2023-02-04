//! A lightweight cross-platform graphics engine.

pub use buffers::*;
pub use colors::*;
pub use fonts::*;
pub use geometry::*;
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
mod opengl;
mod palettes;
mod rendering;
mod shaders;
mod sprites;
mod targets;
mod textures;
mod wgpu;

/// Implements a new opaque identifier for some resource type.
macro_rules! impl_graphics_id {
  ($name:ident) => {
    /// A unique identifier for a kind of graphics resource.
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct $name(u64);

    impl $name {
      /// A value that represents the 'NONE' value of this resource.
      pub const NONE: Self = Self::new(0);

      /// Creates a new ID with the given value.
      #[inline(always)]
      pub const fn new(value: u32) -> Self {
        Self(value as u64)
      }
    }

    impl From<u32> for $name {
      #[inline(always)]
      fn from(value: u32) -> Self {
        Self(value as u64)
      }
    }

    impl From<$name> for u32 {
      #[inline(always)]
      fn from(id: $name) -> Self {
        id.0 as u32
      }
    }

    impl From<crate::collections::ArenaIndex> for $name {
      #[inline(always)]
      fn from(index: crate::collections::ArenaIndex) -> Self {
        Self(index.into())
      }
    }

    impl From<$name> for crate::collections::ArenaIndex {
      #[inline(always)]
      fn from(id: $name) -> Self {
        id.0.into()
      }
    }
  };
}

impl_graphics_id!(BufferId);
impl_graphics_id!(TextureId);
impl_graphics_id!(ShaderId);
impl_graphics_id!(MeshId);
impl_graphics_id!(TargetId);

/// A wrapper for the core [`GraphicsBackend`] implementation.
#[derive(Clone)]
pub struct GraphicsServer {
  backend: std::sync::Arc<Box<dyn GraphicsBackend>>,
}

impl GraphicsServer {
  /// Creates a new [`GraphicsServer`] with a [`headless::HeadlessGraphicsBackend`].
  pub fn headless() -> Self {
    Self::new(headless::HeadlessGraphicsBackend::default())
  }

  /// Creates a new [`GraphicsServer`] with an [`opengl::OpenGLGraphicsBackend`].
  pub fn opengl(window: &winit::window::Window, vsync_enabled: bool, samples: u8) -> crate::Result<Self> {
    unsafe { Ok(Self::new(opengl::OpenGLGraphicsBackend::new(window, vsync_enabled, samples)?)) }
  }

  pub fn wgpu(window: &winit::window::Window, vsync_enabled: bool, samples: u8) -> crate::Result<Self> {
    unsafe { Ok(Self::new(wgpu::WgpuGraphicsBackend::new(window, vsync_enabled, samples)?)) }
  }

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

/// A possible error when interacting with buffer operations in the [`GraphicsBackend`].
#[derive(thiserror::Error, Debug)]
pub enum BufferError {
  #[error("The given buffer ID is invalid.")]
  InvalidId,
  #[error("The buffer is not large enough to hold the requested data.")]
  BufferTooSmall,
  #[error("The given buffer pointer is null")]
  NullPointer,
}

/// An abstraction on top of the underlying graphics API.
///
/// This is a mid-level abstraction that makes use of 'opaque' resource IDs to
/// hide away implementation details and lifetimes. The backend forms the
/// foundation of higher-level abstractions that make it simpler to build
/// graphics programs.
#[allow(clippy::too_many_arguments)]
pub trait GraphicsBackend {
  // frame operations
  fn begin_frame(&self);
  fn end_frame(&self);

  // clear targets
  fn clear_color_buffer(&self, color: Color);
  fn clear_depth_buffer(&self);

  // intrinsics
  fn viewport_size(&self) -> (usize, usize);
  fn set_viewport_size(&self, size: winit::dpi::PhysicalSize<u32>);
  fn set_blend_state(&self, blend_state: BlendState);
  fn set_culling_mode(&self, culling_mode: CullingMode);
  fn set_scissor_mode(&self, scissor_mode: ScissorMode);

  // buffers
  fn buffer_create(&self) -> Result<BufferId, BufferError>;
  fn buffer_read_data(&self, buffer: BufferId, offset: usize, length: usize, pointer: *mut u8) -> Result<(), BufferError>;
  fn buffer_write_data(
    &self,
    buffer: BufferId,
    usage: BufferUsage,
    kind: BufferKind,
    length: usize,
    pointer: *const u8,
  ) -> Result<(), BufferError>;
  fn buffer_delete(&self, buffer: BufferId) -> Result<(), BufferError>;

  // textures
  fn texture_create(&self, sampler: &TextureSampler) -> TextureId;
  fn texture_set_options(&self, texture: TextureId, sampler: &TextureSampler);
  fn texture_initialize(&self, texture: TextureId, width: u32, height: u32, format: TextureFormat);
  fn texture_read_data(&self, texture: TextureId, length: usize, pixel_format: TextureFormat, pixels: *mut u8, mip_level: usize);
  fn texture_write_data(
    &self,
    texture: TextureId,
    width: u32,
    height: u32,
    pixels: *const u8,
    internal_format: TextureFormat,
    pixel_format: TextureFormat,
    mip_level: usize,
  );
  fn texture_write_sub_data(
    &self,
    texture: TextureId,
    region: &Rectangle,
    pixels: *const u8,
    pixel_format: TextureFormat,
    mip_level: usize,
  );
  fn texture_delete(&self, texture: TextureId);

  // shaders
  fn shader_create(&self) -> ShaderId;
  fn shader_link(&self, shader: ShaderId, kernels: &[ShaderKernel]) -> crate::Result<()>;
  fn shader_uniform_location(&self, shader: ShaderId, name: &str) -> Option<usize>;
  fn shader_set_uniform(&self, shader: ShaderId, location: usize, value: &ShaderUniform);
  fn shader_activate(&self, shader: ShaderId);
  fn shader_delete(&self, shader: ShaderId);

  // meshes
  fn mesh_create(&self, vertices: BufferId, indices: BufferId, descriptors: &[VertexDescriptor]) -> MeshId;
  fn mesh_draw(&self, mesh: MeshId, topology: PrimitiveTopology, vertex_count: usize, index_count: usize);
  fn mesh_delete(&self, mesh: MeshId);

  // render targets
  fn target_create(
    &self,
    color_attachment: TextureId,
    depth_attachment: Option<TextureId>,
    stencil_attachment: Option<TextureId>,
  ) -> TargetId;
  fn target_activate(&self, target: TargetId);
  fn target_set_default(&self);
  fn target_blit(&self, from: TargetId, to: TargetId, source_rect: &Rectangle, dest_rect: &Rectangle, filter: TextureFilter);
  fn target_blit_to_display(&self, target: TargetId, source_rect: &Rectangle, dest_rect: &Rectangle, filter: TextureFilter);
  fn target_delete(&self, target: TargetId);
}
