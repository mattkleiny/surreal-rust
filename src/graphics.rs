//! A lightweight graphics system.

pub use buffers::*;
pub use colors::*;
pub use materials::*;
pub use meshes::*;
pub use shaders::*;
pub use sprites::*;
pub use states::*;
pub use textures::*;

use crate::maths::{RectI, Vec2i};

mod buffers;
mod colors;
mod materials;
mod meshes;
mod shaders;
mod sprites;
mod states;
mod textures;

#[cfg(feature = "opengl")]
pub mod opengl;

pub trait GraphicsDevice: Sized {
  type Buffer;
  type Framebuffer;
  type Program;
  type Shader;
  type Texture;
  type TimerQuery;
  type Uniform;
  type VertexArray;
  type VertexAttr;

  // buffers
  fn get_vertex_attr(&self, program: &Self::Program, name: &str) -> Option<Self::VertexAttr>;
  fn get_uniform(&self, program: &Self::Program, name: &str) -> Self::Uniform;
  fn bind_buffer(&self, vertex_array: &Self::VertexArray, buffer: &Self::Buffer, target: BufferTarget);
  fn configure_vertex_attr(&self, vertex_array: &Self::VertexArray, attr: &Self::VertexAttr, descriptor: &VertexAttrDescriptor);
  fn create_framebuffer(&self, texture: Self::Texture) -> Self::Framebuffer;
  fn create_buffer(&self) -> Self::Buffer;
  fn allocate_buffer<T>(&self, buffer: &Self::Buffer, data: BufferData<T>, target: BufferTarget, mode: BufferUploadMode);

  // shaders
  fn create_shader(&self, kind: ShaderKind) -> Self::Shader;
  fn create_shader_from_source(&self, source: &[u8], kind: ShaderKind) -> Self::Shader;
  fn create_vertex_array(&self) -> Self::VertexArray;
  fn create_program_from_shaders(&self, vertex_shader: Self::Shader, fragment_shader: Self::Shader) -> Self::Program;

  // textures
  fn get_framebuffer_texture<'f>(&self, framebuffer: &'f Self::Framebuffer) -> &'f Self::Texture;
  fn create_texture(&self, format: TextureFormat, size: Vec2i) -> Self::Texture;
  fn create_texture_from_data(&self, size: Vec2i, data: &[u8]) -> Self::Texture;
  fn get_texture_size(&self, texture: &Self::Texture) -> Vec2i;
  fn upload_to_texture(&self, texture: &Self::Texture, size: Vec2i, data: &[u8]);
  fn read_pixels(&self, target: &RenderTarget<Self>, viewport: RectI) -> TextureData;

  // commands
  fn begin_commands(&self);
  fn end_commands(&self);

  // rendering
  fn draw_arrays(&self, index_count: u32, render_state: &RenderState<Self>);
  fn draw_elements(&self, index_count: u32, render_state: &RenderState<Self>);
  fn draw_elements_instanced(&self, index_count: u32, instance_count: u32, render_state: &RenderState<Self>);
}
