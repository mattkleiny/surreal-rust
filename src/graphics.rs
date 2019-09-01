//! A lightweight and fast cross-platform graphics engine.
//!
//! This implementation is designed to be portable across platforms through consumption of a standard graphics API
//! across all of those platforms, as opposed to offering different APIs for different platforms (ala gfx-hal).
//!
//! Whilst more directly coupled than other providers, this implementation is simple, direct and fast. It is designed
//! to account for the majority use case as opposed to all possibilities and to do it well, as opposed to solving
//! the general case and doing it poorly.

use std::sync::Mutex;

use glam::{Mat4, Vec2};
use glam::f32::Vec4;

pub use colors::*;
pub use meshes::*;
pub use sprites::*;

use crate::maths::{RectI, Vec2i};

mod colors;
mod meshes;
mod sprites;

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
  type Buffer: Send + Sync;
  type Framebuffer: Send + Sync;
  type Program: Send + Sync;
  type Shader: Send + Sync;
  type Texture: Send + Sync;
  type Uniform: Send + Sync;
  type VertexArray: Send + Sync;
  type VertexAttr: Send + Sync;

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
  unsafe fn clear_render_target(&self, ops: &ClearOps);
  unsafe fn flush_commands(&self);

  // rendering
  unsafe fn draw_arrays(&self, index_count: u32, render_state: &RenderState<Self>);
  unsafe fn draw_elements(&self, index_count: u32, render_state: &RenderState<Self>);
  unsafe fn draw_elements_instanced(&self, index_count: u32, instance_count: u32, render_state: &RenderState<Self>);

  /// Creates a new command queue for this device; a command queue is share-able between threads, and allows
  /// the deferred execution of work on the graphics device through a common channel.
  fn create_command_queue(&self) -> CommandQueue<Self> { CommandQueue::new() }
}

/// A queue of commands to be executed against a graphics device.
///
/// Synchronized and thread-safe; this is designed to be passed to worker threads and then
/// flushed on the main thread prior to completing the frame.
pub struct CommandQueue<'a, D> where D: GraphicsDevice {
  queue: Mutex<Vec<Command<'a, D>>>,
}

impl<'a, D> CommandQueue<'a, D> where D: GraphicsDevice {
  pub fn new() -> Self {
    Self {
      queue: Mutex::new(Vec::new()),
    }
  }

  /// Enqueues the given command to be executed on the device.
  pub fn enqueue(&self, command: Command<'a, D>) {
    let mut queue = self.queue.lock().unwrap();

    queue.push(command);
  }

  /// Flushes enqueued commands, replaying them on the given device.
  pub fn flush(&self, device: &D) {
    unsafe {
      let mut queue = self.queue.lock().unwrap();

      while let Some(command) = queue.pop() {
        match command {
          Command::SetRenderState(render_state) => {
            unimplemented!()
          }
          Command::ClearRenderTarget(ops) => {
            device.clear_render_target(&ops);
          }
          Command::BindBuffer { vertex_array, buffer, target } => {
            device.bind_buffer(vertex_array, buffer, target);
          }
          Command::DrawArrays { index_count, render_state } => {
            device.draw_arrays(index_count, render_state);
          }
          Command::DrawElements { index_count, render_state } => {
            device.draw_elements(index_count, render_state);
          }
          Command::DrawElementsInstanced { index_count, instance_count, render_state } => {
            device.draw_elements_instanced(index_count, instance_count, render_state);
          }
        }
      }

      device.flush_commands();
    }
  }
}

/// A command that can be placed into a queue for later execution by the graphics device.
#[derive(Copy, Clone)]
pub enum Command<'a, D> where D: GraphicsDevice {
  /// Sets the render state on the graphics device.
  SetRenderState(&'a RenderState<'a, D>),
  /// Clears the active render target.
  ClearRenderTarget(ClearOps),
  /// Binds the given buffer for rendering on the device.
  BindBuffer {
    vertex_array: &'a D::VertexArray,
    buffer: &'a D::Buffer,
    target: BufferTarget,
  },
  /// Draws the bounds arrays on the device.
  DrawArrays {
    index_count: u32,
    render_state: &'a RenderState<'a, D>,
  },
  /// Draws the bounds elements on the device.
  DrawElements {
    index_count: u32,
    render_state: &'a RenderState<'a, D>,
  },
  /// Draws the bound elements in an instanced mode on the device.
  DrawElementsInstanced {
    index_count: u32,
    instance_count: u32,
    render_state: &'a RenderState<'a, D>,
  },
}

/// Encapsulates the rendering state of a particular device, allowing flexible state changes.
///
/// The state contains information, such as the active shader program, viewport, uniforms, etc.
#[derive(Clone)]
pub struct RenderState<'a, D> where D: GraphicsDevice {
  pub render_target: &'a RenderTarget<'a, D>,
  pub shader_program: &'a D::Program,
  pub vertex_array: &'a D::VertexArray,
  pub primitive_type: PrimitiveType,
  pub uniforms: &'a [(&'a D::Uniform, UniformData)],
  pub textures: &'a [&'a D::Texture],
  pub viewport: RectI,
  pub rasterizer: RasterizerState,
}

/// Possible types of primitives that can be rendered by the device.
#[derive(Clone, Copy)]
pub enum PrimitiveType {
  Triangles,
  Lines,
}

/// Encapsulates the state of the device's rasterizer, such as blending/stencil/color ops, etc.
#[derive(Clone, Debug)]
pub struct RasterizerState {
  pub blend: BlendState,
  pub depth: Option<DepthState>,
  pub stencil: Option<StencilState>,
  pub clear_ops: ClearOps,
  pub color_mask: bool,
}

impl Default for RasterizerState {
  #[inline]
  fn default() -> RasterizerState {
    RasterizerState {
      blend: BlendState::default(),
      depth: None,
      stencil: None,
      clear_ops: ClearOps::default(),
      color_mask: true,
    }
  }
}

/// A possible target for rendering.
#[derive(Clone, Copy, Debug)]
pub enum RenderTarget<'a, D> where D: GraphicsDevice {
  /// The default window target; whatever the OpenGL context points to by default.
  Default,
  /// A specific and allocated frame-buffer.
  Framebuffer(&'a D::Framebuffer),
}

/// Defines the possible blending states for the rasterizer.
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

/// Defines the possible depth testing states for the rasterizer.
#[derive(Clone, Copy, Default, Debug)]
pub struct DepthState {
  pub func: DepthFunc,
  pub write: bool,
}

/// Different fixed-function depth tests for the rasterizer.
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

/// Defines the possible stencil buffer states for the rasterizer.
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

/// Different fixed-function stencil tests for the rasterizer.
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

/// Defines the possible clear operations for the rasterizer against the frame-buffer.
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

/// Defines possible data types used in a graphics buffer.
#[derive(Clone, Copy, Debug)]
pub enum BufferData<'a, T> {
  /// The buffer is initialized, upload the given slice directly.
  Memory(&'a [T]),
  /// The buffer is uninitialized, prepare the given number of bytes.
  Uninitialized(usize),
}

/// Different targets for buffer uploads and creation.
#[derive(Clone, Copy, Debug)]
pub enum BufferTarget {
  Vertex,
  Index,
}

/// Different buffer upload methods for optimized memory accesses.
#[derive(Clone, Copy, Debug)]
pub enum BufferUploadMode {
  Static,
  Dynamic,
}

/// Different types of vertex attributes supported by the graphics device.
#[derive(Clone, Copy, Debug)]
pub enum VertexAttrType {
  F32,
  I16,
  I8,
  U16,
  U8,
}

/// A description of a vertex attribute to be used when constructing mesh resources.
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

/// Different types of vertex attribute classes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VertexAttrClass {
  Float,
  FloatNorm,
  Int,
}

/// Different types of shader resources.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShaderKind {
  Vertex,
  Fragment,
}

/// Encapsulates the possible data that can be uploaded to a shader's uniform variable.
#[derive(Clone, Copy)]
pub enum UniformData {
  Int(i32),
  Mat4(Mat4),
  Vec2(Vec2),
  Vec4(Vec4),
  TextureUnit(u32),
}

/// Describes the different representations of texture data supported by the graphics device.
#[derive(Clone, Debug)]
pub enum TextureData {
  U8(Vec<u8>),
  U16(Vec<u16>),
}

/// Different supported texture formats by the graphics device.
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
