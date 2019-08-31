//! Graphics device state management

use super::*;

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

#[derive(Clone, Copy)]
pub enum Primitive {
  Triangles,
  Lines,
}

#[derive(Clone, Debug)]
pub struct RenderOptions {
  pub blend: BlendState,
  pub depth: Option<DepthState>,
  pub stencil: Option<StencilState>,
  pub clear_ops: ClearOps,
  pub color_mask: bool,
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

impl Default for BlendState {
  #[inline]
  fn default() -> BlendState {
    BlendState::Off
  }
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

impl Default for DepthFunc {
  #[inline]
  fn default() -> DepthFunc {
    DepthFunc::Less
  }
}

#[derive(Clone, Copy, Debug)]
pub struct StencilState {
  pub func: StencilFunc,
  pub reference: u32,
  pub mask: u32,
  pub write: bool,
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

#[derive(Clone, Copy, Debug)]
pub enum StencilFunc {
  Always,
  Equal,
}

impl Default for StencilFunc {
  #[inline]
  fn default() -> StencilFunc {
    StencilFunc::Always
  }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ClearOps {
  pub color: Option<Color>,
  pub depth: Option<f32>,
  pub stencil: Option<u8>,
}

impl ClearOps {
  #[inline]
  pub fn has_ops(&self) -> bool {
    self.color.is_some() || self.depth.is_some() || self.stencil.is_some()
  }
}