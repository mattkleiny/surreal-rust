//! Graphics engine for Surreal.

#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![feature(allocator_api)]

pub use animations::*;
pub use buffers::*;
pub use fonts::*;
pub use geometry::*;
pub use images::*;
pub use lighting::*;
pub use materials::*;
pub use meshes::*;
pub use opengl::*;
pub use rendering::*;
pub use shaders::*;
pub use sprites::*;
pub use targets::*;
pub use textures::*;

mod animations;
mod buffers;
mod fonts;
mod geometry;
mod headless;
mod images;
mod internal;
mod lighting;
mod materials;
mod meshes;
mod opengl;
mod rendering;
mod shaders;
mod sprites;
mod targets;
mod textures;

// Re-export macros for use in other crates.
pub use macros::{ToShaderUniformSet, Vertex};

common::impl_arena_index!(pub BufferId, "Identifies a graphics buffer.");
common::impl_arena_index!(pub TextureId, "Identifies a texture.");
common::impl_arena_index!(pub ShaderId, "Identifies a shader program.");
common::impl_arena_index!(pub MeshId, "Identifies a mesh.");
common::impl_arena_index!(pub TargetId, "Identifies a render target.");

common::impl_server!(GraphicsServer by GraphicsBackend default headless::HeadlessGraphicsBackend);

/// Gets the graphics server instance.
#[inline(always)]
pub fn graphics() -> &'static dyn GraphicsBackend {
  GraphicsServer::instance()
}

/// A possible error when interacting with buffers.
#[derive(Debug)]
pub enum BufferError {
  InvalidId(BufferId),
  BufferTooSmall,
  NullPointer,
}

/// A possible error when interacting with textures.
#[derive(Debug)]
pub enum TextureError {
  InvalidId(TextureId),
  InvalidImage(ImageError),
}

/// A possible error when interacting with shaders.
#[derive(Debug)]
pub enum ShaderError {
  InvalidId(ShaderId),
  CompileError(String),
  FailedToLoad,
  InvalidInclude,
  InvalidUniform,
}

/// A possible error when interacting with meshes.
#[derive(Debug)]
pub enum MeshError {
  InvalidId(MeshId),
  FailedToCreate,
}

/// A possible error when interacting with render targets.
#[derive(Debug)]
pub enum TargetError {
  InvalidId(TargetId),
  FailedToBuildAttachments,
}

/// A memory barrier for synchronising memory access in a shader.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MemoryBarrier {
  ImageAccess,
}

/// An abstraction on top of the underlying graphics API.
///
/// This is a mid-level abstraction that makes use of 'opaque' resource IDs to
/// hide away implementation details and lifetimes. The backend forms the
/// foundation of higher-level abstractions that make it simpler to build
/// graphics programs.
#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
pub trait GraphicsBackend {
  // frame operations
  fn begin_frame(&self);
  fn end_frame(&self);

  // clear targets
  fn clear_color_buffer(&self, color: common::Color);
  fn clear_depth_buffer(&self, depth: f32);

  // intrinsics
  fn viewport_size(&self) -> (usize, usize);
  fn set_viewport_size(&self, size: common::UVec2);
  fn set_blend_state(&self, blend_state: BlendState);
  fn set_culling_mode(&self, culling_mode: CullingMode);
  fn set_scissor_mode(&self, scissor_mode: ScissorMode);

  // buffers
  fn buffer_create(&self) -> Result<BufferId, BufferError>;
  fn buffer_read_data(&self, buffer: BufferId, offset: usize, length: usize, pointer: *mut u8) -> Result<(), BufferError>;
  fn buffer_write_data(&self, buffer: BufferId, usage: BufferUsage, kind: BufferKind, length: usize, pointer: *const u8) -> Result<(), BufferError>;
  fn buffer_delete(&self, buffer: BufferId) -> Result<(), BufferError>;

  // textures
  fn texture_create(&self, sampler: &TextureSampler) -> Result<TextureId, TextureError>;
  fn texture_set_options(&self, texture: TextureId, sampler: &TextureSampler) -> Result<(), TextureError>;
  fn texture_initialize(&self, texture: TextureId, width: u32, height: u32, format: TextureFormat) -> Result<(), TextureError>;
  fn texture_read_data(&self, texture: TextureId, length: usize, pixel_format: TextureFormat, pixels: *mut u8, mip_level: usize) -> Result<(), TextureError>;
  fn texture_write_data(&self, texture: TextureId, width: u32, height: u32, pixels: *const u8, internal_format: TextureFormat, pixel_format: TextureFormat, mip_level: usize) -> Result<(), TextureError>;
  fn texture_write_sub_data(&self, texture: TextureId, region: &common::Rectangle, pixels: *const u8, pixel_format: TextureFormat, mip_level: usize) -> Result<(), TextureError>;
  fn texture_delete(&self, texture: TextureId) -> Result<(), TextureError>;

  // shaders
  fn shader_create(&self) -> Result<ShaderId, ShaderError>;
  fn shader_link(&self, shader: ShaderId, kernels: &[ShaderKernel]) -> Result<(), ShaderError>;
  fn shader_uniform_location(&self, shader: ShaderId, name: &str) -> Option<usize>;
  fn shader_set_uniform(&self, shader: ShaderId, location: usize, value: &ShaderUniform) -> Result<(), ShaderError>;
  fn shader_activate(&self, shader: ShaderId) -> Result<(), ShaderError>;
  fn shader_dispatch_compute(&self, shader: ShaderId, x: u32, y: u32, z: u32) -> Result<(), ShaderError>;
  fn shader_memory_barrier(&self, barrier: MemoryBarrier) -> Result<(), ShaderError>;
  fn shader_delete(&self, shader: ShaderId) -> Result<(), ShaderError>;

  // meshes
  fn mesh_create(&self, vertices: BufferId, indices: BufferId, descriptors: &[VertexDescriptor]) -> Result<MeshId, MeshError>;
  fn mesh_draw(&self, mesh: MeshId, topology: PrimitiveTopology, vertex_count: usize, index_count: usize) -> Result<(), MeshError>;
  fn mesh_delete(&self, mesh: MeshId) -> Result<(), MeshError>;

  // render targets
  fn target_create(&self, color_attachment: TextureId, depth_attachment: Option<TextureId>, stencil_attachment: Option<TextureId>) -> Result<TargetId, TargetError>;
  fn target_activate(&self, target: TargetId) -> Result<(), TargetError>;
  fn target_set_default(&self) -> Result<(), TargetError>;
  fn target_blit_to_active(&self, target: TargetId, source_rect: Option<common::Rectangle>, dest_rect: Option<common::Rectangle>, filter: TextureFilter) -> Result<(), TargetError>;
  fn target_delete(&self, target: TargetId) -> Result<(), TargetError>;
}
