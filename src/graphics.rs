//! A lightweight and fast cross-platform graphics engine.
//!
//! This implementation is designed to be portable across platforms through consumption of a standard graphics API
//! across all of those platforms, as opposed to offering different engines for different platforms (ala gfx-hal).
//!
//! Whilst more directly coupled than other providers, this implementation is simple, direct and fast. It is designed
//! to account for the majority use case as opposed to all possibilities and to do it well.

pub use buffers::*;
pub use colors::*;
pub use commands::*;
pub use mesh::*;
pub use shaders::*;
pub use sprites::*;
pub use states::*;
pub use textures::*;

use crate::maths::{RectI, Vec2i};

mod buffers;
mod colors;
mod commands;
mod mesh;
mod shaders;
mod sprites;
mod states;
mod textures;

#[cfg(feature = "opengl")]
pub mod opengl;

/// An abstraction over a graphics device for use in the graphics rendering pipeline.
///
/// This is a low-level abstraction that allows us to decouple specific GPU vendor libraries from consumption of those
/// libraries across the engine. The API is broadly shaped to look like something OpenGL provides, and the default
/// implementation in the engine is based on OpenGL.
///
/// It should be possible to extend this implementation to support other platforms, like a web-based GL implementation,
/// or DirectX; however more sophisticated/low level APIs such as Metal and Vulkan will not fit this paradigm well.
pub trait GraphicsDevice: Sized {
  type Buffer;
  type Framebuffer;
  type Program;
  type Shader;
  type Texture;
  type Uniform;
  type VertexArray;
  type VertexAttr;

  // buffers
  unsafe fn get_vertex_attr(&self, program: &Self::Program, name: &str) -> Option<Self::VertexAttr>;
  unsafe fn get_uniform(&self, program: &Self::Program, name: &str) -> Self::Uniform;
  unsafe fn bind_buffer(&self, vertex_array: &Self::VertexArray, buffer: &Self::Buffer, target: BufferTarget);
  unsafe fn configure_vertex_attr(&self, vertex_array: &Self::VertexArray, attr: &Self::VertexAttr, descriptor: &VertexAttrDescriptor);
  unsafe fn create_framebuffer(&self, texture: Self::Texture) -> Self::Framebuffer;
  unsafe fn create_buffer(&self) -> Self::Buffer;
  unsafe fn upload_to_buffer<T>(&self, buffer: &Self::Buffer, data: BufferData<T>, target: BufferTarget, mode: BufferUploadMode);

  // shaders
  unsafe fn create_shader_from_source(&self, source: &[u8], kind: ShaderKind) -> Self::Shader;
  unsafe fn create_vertex_array(&self) -> Self::VertexArray;
  unsafe fn create_program_from_shaders(&self, vertex_shader: Self::Shader, fragment_shader: Self::Shader) -> Self::Program;

  // frame buffers
  unsafe fn get_framebuffer_texture<'f>(&self, framebuffer: &'f Self::Framebuffer) -> &'f Self::Texture;
  unsafe fn read_pixels(&self, target: &RenderTarget<Self>, viewport: RectI) -> TextureData;

  // textures
  unsafe fn create_texture(&self, format: TextureFormat, size: Vec2i) -> Self::Texture;
  unsafe fn create_texture_from_data(&self, size: Vec2i, data: &[u8]) -> Self::Texture;
  unsafe fn get_texture_size(&self, texture: &Self::Texture) -> Vec2i;
  unsafe fn upload_to_texture(&self, texture: &Self::Texture, size: Vec2i, data: &[u8]);

  // commands
  unsafe fn begin_commands(&self);
  unsafe fn end_commands(&self);

  // rendering
  unsafe fn draw_arrays(&self, index_count: u32, render_state: &RenderState<Self>);
  unsafe fn draw_elements(&self, index_count: u32, render_state: &RenderState<Self>);
  unsafe fn draw_elements_instanced(&self, index_count: u32, instance_count: u32, render_state: &RenderState<Self>);
}
