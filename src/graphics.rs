//! A lightweight graphics system.

use glam::{Mat4, Vec2};
use glam::f32::Vec4;

pub use buffers::*;
pub use materials::*;
pub use meshes::*;
pub use primitives::*;
pub use shaders::*;
pub use sprites::*;
pub use textures::*;

use crate::maths::{Vec2i, RectI};

mod buffers;
mod materials;
mod meshes;
mod shaders;
mod sprites;
mod textures;
mod primitives;

pub unsafe trait GraphicsDevice: Sized {
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

#[derive(Clone, Copy, Debug)]
pub enum VertexAttrType {
  F32,
  I16,
  I8,
  U16,
  U8,
}

#[derive(Clone, Copy, Debug)]
pub enum BufferData<'a, T> {
  Uninitialized(usize),
  Memory(&'a [T]),
}

#[derive(Clone, Copy, Debug)]
pub enum BufferTarget {
  Vertex,
  Index,
}

#[derive(Clone, Copy, Debug)]
pub enum BufferUploadMode {
  Static,
  Dynamic,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShaderKind {
  Vertex,
  Fragment,
}

#[derive(Clone, Copy)]
pub enum UniformData {
  Int(i32),
  Mat4([Mat4; 4]),
  Vec2(Vec2),
  Vec4(Vec4),
  TextureUnit(u32),
}

#[derive(Clone, Copy)]
pub enum Primitive {
  Triangles,
  Lines,
}

#[derive(Clone)]
pub struct RenderState<'a, D> where D: GraphicsDevice {
  pub target: &'a RenderTarget<'a, D>,
  pub program: &'a D::Program,
  pub vertex_array: &'a D::VertexArray,
  pub primitive: Primitive,
  pub uniforms: &'a [(&'a D::Uniform, UniformData)],
  pub textures: &'a [&'a D::Texture],
  pub viewport: RectI,
  pub options: RenderOptions,
}

#[derive(Clone, Debug)]
pub struct RenderOptions {
  pub blend: BlendState,
  pub depth: Option<DepthState>,
  pub stencil: Option<StencilState>,
  pub clear_ops: ClearOps,
  pub color_mask: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ClearOps {
  pub color: Option<Color>,
  pub depth: Option<f32>,
  pub stencil: Option<u8>,
}

#[derive(Clone, Copy, Debug)]
pub enum RenderTarget<'a, D> where D: GraphicsDevice {
  Default,
  Framebuffer(&'a D::Framebuffer),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlendState {
  Off,
  RGBOneAlphaOne,
  RGBOneAlphaOneMinusSrcAlpha,
  RGBSrcAlphaAlphaOneMinusSrcAlpha,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct DepthState {
  pub func: DepthFunc,
  pub write: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum DepthFunc {
  Less,
  Always,
}

#[derive(Clone, Copy, Debug)]
pub struct StencilState {
  pub func: StencilFunc,
  pub reference: u32,
  pub mask: u32,
  pub write: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum StencilFunc {
  Always,
  Equal,
}

impl Default for RenderOptions {
  #[inline]
  fn default() -> RenderOptions {
    RenderOptions {
      blend: BlendState::default(),
      depth: None,
      stencil: None,
      clear_ops: ClearOps::default(),
      color_mask: true,
    }
  }
}

impl Default for BlendState {
  #[inline]
  fn default() -> BlendState {
    BlendState::Off
  }
}

impl Default for StencilState {
  #[inline]
  fn default() -> StencilState {
    StencilState {
      func: StencilFunc::default(),
      reference: 0,
      mask: !0,
      write: false,
    }
  }
}

impl Default for DepthFunc {
  #[inline]
  fn default() -> DepthFunc {
    DepthFunc::Less
  }
}

impl Default for StencilFunc {
  #[inline]
  fn default() -> StencilFunc {
    StencilFunc::Always
  }
}

#[derive(Clone, Debug)]
pub enum TextureData {
  U8(Vec<u8>),
  U16(Vec<u16>),
}

#[derive(Clone, Copy, Debug)]
pub struct VertexAttrDescriptor {
  pub size: usize,
  pub class: VertexAttrClass,
  pub attr_type: VertexAttrType,
  pub stride: usize,
  pub offset: usize,
  pub divisor: u32,
  pub buffer_index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VertexAttrClass {
  Float,
  FloatNorm,
  Int,
}

#[derive(Copy, Clone, Debug)]
pub enum TextureFormat {
  RGB8,
  RGBA8,
}

impl TextureFormat {
  #[inline]
  pub fn channels(self) -> usize {
    match self {
      TextureFormat::RGB8 => 3,
      TextureFormat::RGBA8 => 4,
    }
  }
}

impl ClearOps {
  #[inline]
  pub fn has_ops(&self) -> bool {
    self.color.is_some() || self.depth.is_some() || self.stencil.is_some()
  }
}