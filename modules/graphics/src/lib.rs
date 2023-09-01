//! Graphics engine for Surreal.
//!
//! The graphics engine is lightweight and cross-platform and mirrors and
//! provides capabilities for simple 2D and 3D graphics. It is a bespoke
//! renderer built entirely in Rust.

#![feature(anonymous_lifetime_in_impl_trait)]

pub use buffers::*;
pub use colors::*;
pub use geometry::*;
pub use materials::*;
pub use meshes::*;
pub use palettes::*;
pub use rendering::*;
pub use shaders::*;
pub use skinning::*;
pub use sprites::*;
pub use targets::*;
pub use textures::*;

mod buffers;
mod colors;
mod geometry;
mod headless;
mod materials;
mod meshes;
mod opengl;
mod palettes;
mod rendering;
mod shaders;
mod skinning;
mod sprites;
mod targets;
mod textures;

surreal::impl_server!(GraphicsServer, GraphicsBackend);

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

    impl From<surreal::collections::ArenaIndex> for $name {
      #[inline(always)]
      fn from(index: surreal::collections::ArenaIndex) -> Self {
        Self(index.into())
      }
    }

    impl From<$name> for surreal::collections::ArenaIndex {
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

impl GraphicsServer {
  /// Creates a new [`GraphicsServer`] with a no-op, headless backend.
  pub fn create_headless() -> Self {
    Self::new(headless::HeadlessGraphicsBackend::default())
  }

  /// Creates a new [`GraphicsServer`] with an OpenGL backend.
  pub fn create_opengl(_host: &dyn GraphicsHost) -> surreal::Result<Self> {
    todo!()
  }
}

/// The nominal max number of texture units that might be be bound in the GPU.
///
/// This is a hint for sizing arrays and other data structures.
const MAX_TEXTURE_UNITS: usize = 32;

/// A possible error when interacting with buffers.
#[derive(thiserror::Error, Debug)]
pub enum BufferError {
  #[error("the given buffer ID {0:?} is invalid")]
  InvalidId(BufferId),
  #[error("the buffer is not large enough to hold the requested data")]
  BufferTooSmall,
  #[error("the given buffer pointer is null")]
  NullPointer,
}

/// A possible error when interacting with textures.
#[derive(thiserror::Error, Debug)]
pub enum TextureError {
  #[error("the given texture ID {0:?} is invalid")]
  InvalidId(TextureId),
}

/// A possible error when interacting with shaders.
#[derive(thiserror::Error, Debug)]
pub enum ShaderError {
  #[error("the given shader ID {0:?} is invalid")]
  InvalidId(ShaderId),
  #[error("the shader failed to compile")]
  CompileError(String),
}

/// A possible error when interacting with meshes.
#[derive(thiserror::Error, Debug)]
pub enum MeshError {
  #[error("the given mesh ID {0:?} is invalid")]
  InvalidId(MeshId),
}

/// A possible error when interacting with render targets.
#[derive(thiserror::Error, Debug)]
pub enum TargetError {
  #[error("the given target ID {0:?} is invalid")]
  InvalidId(TargetId),
}

/// An abstraction over the host capable of running graphics.
///
/// This type implemented by the host application and is used to provide the
/// graphics backend with access to the host's windowing system and other
/// infrastructure.
pub trait GraphicsHost {}

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
  fn set_viewport_size(&self, size: surreal::maths::UVec2);
  fn set_blend_state(&self, blend_state: BlendState);
  fn set_culling_mode(&self, culling_mode: CullingMode);
  fn set_scissor_mode(&self, scissor_mode: ScissorMode);

  // buffers
  fn buffer_create(&self) -> Result<BufferId, BufferError>;
  fn buffer_read_data(
    &self,
    buffer: BufferId,
    offset: usize,
    length: usize,
    pointer: *mut u8,
  ) -> Result<(), BufferError>;
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
  fn texture_create(&self, sampler: &TextureSampler) -> Result<TextureId, TextureError>;
  fn texture_set_options(
    &self,
    texture: TextureId,
    sampler: &TextureSampler,
  ) -> Result<(), TextureError>;
  fn texture_initialize(
    &self,
    texture: TextureId,
    width: u32,
    height: u32,
    format: TextureFormat,
  ) -> Result<(), TextureError>;
  fn texture_read_data(
    &self,
    texture: TextureId,
    length: usize,
    pixel_format: TextureFormat,
    pixels: *mut u8,
    mip_level: usize,
  ) -> Result<(), TextureError>;
  fn texture_write_data(
    &self,
    texture: TextureId,
    width: u32,
    height: u32,
    pixels: *const u8,
    internal_format: TextureFormat,
    pixel_format: TextureFormat,
    mip_level: usize,
  ) -> Result<(), TextureError>;
  fn texture_write_sub_data(
    &self,
    texture: TextureId,
    region: &surreal::maths::Rectangle,
    pixels: *const u8,
    pixel_format: TextureFormat,
    mip_level: usize,
  ) -> Result<(), TextureError>;
  fn texture_delete(&self, texture: TextureId) -> Result<(), TextureError>;

  // shaders
  fn shader_create(&self) -> Result<ShaderId, ShaderError>;
  fn shader_link(&self, shader: ShaderId, kernels: &[ShaderKernel]) -> Result<(), ShaderError>;
  fn shader_uniform_location(&self, shader: ShaderId, name: &str) -> Option<usize>;
  fn shader_set_uniform(
    &self,
    shader: ShaderId,
    location: usize,
    value: &ShaderUniform,
  ) -> Result<(), ShaderError>;
  fn shader_activate(&self, shader: ShaderId) -> Result<(), ShaderError>;
  fn shader_delete(&self, shader: ShaderId) -> Result<(), ShaderError>;

  // meshes
  fn mesh_create(
    &self,
    vertices: BufferId,
    indices: BufferId,
    descriptors: &[VertexDescriptor],
  ) -> Result<MeshId, MeshError>;
  fn mesh_draw(
    &self,
    mesh: MeshId,
    topology: PrimitiveTopology,
    vertex_count: usize,
    index_count: usize,
  ) -> Result<(), MeshError>;
  fn mesh_delete(&self, mesh: MeshId) -> Result<(), MeshError>;

  // render targets
  fn target_create(
    &self,
    color_attachment: TextureId,
    depth_attachment: Option<TextureId>,
    stencil_attachment: Option<TextureId>,
  ) -> Result<TargetId, TargetError>;
  fn target_activate(&self, target: TargetId) -> Result<(), TargetError>;
  fn target_set_default(&self) -> Result<(), TargetError>;
  fn target_blit(
    &self,
    from: TargetId,
    to: TargetId,
    source_rect: &surreal::maths::Rectangle,
    dest_rect: &surreal::maths::Rectangle,
    filter: TextureFilter,
  ) -> Result<(), TargetError>;
  fn target_blit_to_display(
    &self,
    target: TargetId,
    source_rect: &surreal::maths::Rectangle,
    dest_rect: &surreal::maths::Rectangle,
    filter: TextureFilter,
  ) -> Result<(), TargetError>;
  fn target_delete(&self, target: TargetId) -> Result<(), TargetError>;
}
